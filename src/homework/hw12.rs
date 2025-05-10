use rand::Rng;

/// Функція підрахунку мінімальної кількості переміщень вантажу
fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total as usize % n != 0 {
        return -1; // неможливо зробити вантажі однаковими
    }

    let avg = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as i32 - avg as i32;
        moves += balance.abs();
    }

    moves
}

/// Функція генерації shipment-вектора, який завжди може бути вирівняний
fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = rand::thread_rng().gen_range(10..50);
    let mut shipments = vec![avg; n];

    // Робимо n−1 елемент трохи більшим, а останній компенсуємо
    for i in 0..n - 1 {
        let delta = rand::thread_rng().gen_range(0..=avg.min(10));
        shipments[i] += delta;
        shipments[n - 1] -= delta;
    }

    shipments
}

/// Для тестування
fn main() {
    let test1 = vec![8, 2, 2, 4, 4]; // avg = 4, результат = 4
    println!("Moves needed: {}", count_permutation(&test1));

    let test2 = vec![9, 3, 7, 2, 9]; // avg = 6, результат = 7
    println!("Moves needed: {}", count_permutation(&test2));

    let test3 = vec![1, 1, 1, 1, 6]; // avg = 2, результат = 4
    println!("Moves needed: {}", count_permutation(&test3));

    let random = gen_shipments(10);
    println!("Generated shipments: {:?}", random);
    println!("Moves needed: {}", count_permutation(&random));
}
