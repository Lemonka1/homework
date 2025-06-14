pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::from("")];
    }

    let prev = gray(n - 1);

    let mut result = Vec::new();

    // Додаємо 0 спереду
    for s in &prev {
        result.push(format!("0{}", s));
    }

    // Додаємо 1 спереду до зворотного порядку
    for s in prev.iter().rev() {
        result.push(format!("1{}", s));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!( "" )),
            (1, vec!( "0", "1" )),
            (2, vec!( "00", "01", "11", "10" )),
            (3, vec!( "000", "001", "011", "010", "110", "111", "101", "100" )),
            (4, vec!( "0000", "0001", "0011", "0010", 
                      "0110", "0111", "0101", "0100", 
                      "1100", "1101", "1111", "1110", 
                      "1010", "1011", "1001", "1000" )),
        ];

        test_data.iter().for_each(|(n, out)| {
            assert_eq!(gray(*n), *out);
        });
    }
}
