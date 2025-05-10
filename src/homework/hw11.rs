use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1])
}

pub fn print_vector_with_min_sum(data: &[i32]) {
    // Перший рядок — індекси
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Другий рядок — дані
    print!("data:  ");
    for val in data {
        print!("{:>3},", val);
    }
    println!();

    // Підкреслення мінімальної пари
    let (min_i, a, b) = min_adjacent_sum(data);
    print!("indexes:");
    for i in 0..data.len() {
        if i == min_i {
            print!("\\__");
        } else if i == min_i + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a,
        b,
        a + b,
        min_i,
        min_i + 1
    );
}

#[test]
fn test_output() {
    let data = gen_random_vector(20);
    print_vector_with_min_sum(&data);
}
