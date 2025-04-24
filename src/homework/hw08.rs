/// Повертає `true`, якщо `n` ‑ просте число.
pub fn is_prime(n: u32) -> bool {
    // 0, 1 та всі парні >2 — непрості
    if n < 2 || (n % 2 == 0 && n != 2) {
        return false;
    }
    // перевіряємо лише непарні дільники до √n
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let cases = [
            (0, false), (1, false), (2, true), (3, true),
            (4, false), (5, true), (100, false), (10_007, true),
        ];
        for (n, expected) in cases {
            assert_eq!(is_prime(n), expected);
        }
    }
}
