pub fn invert_the_case(s: String) -> String {
    s.chars()
     .map(|c| {
         if c.is_lowercase() {
             c.to_uppercase().to_string()
         } else if c.is_uppercase() {
             c.to_lowercase().to_string()
         } else {
             c.to_string()
         }
     })
     .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_the_case() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
            ("123! ПрИвЕт", "123! привеТ"),
        ];

        for (a, b) in data {
            assert_eq!(invert_the_case(a.to_string()), b);
            assert_eq!(invert_the_case(b.to_string()), a);
        }
    }
}
