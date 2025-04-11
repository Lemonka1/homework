fn main() {
    const HEIGHT: usize = 7; 
    const WIDTH: usize = 13; 

    let mut output = String::new();

  
    for i in 0..HEIGHT / 2 + 1 {
        for _ in 0..(WIDTH / 2 - i) {
            output.push(' ');
        }
        output.push('/');
        for _ in 0..(i * 2) {
            output.push(' ');
        }
        output.push('\\');
        output.push('\n');
    }

   
    for i in (0..HEIGHT / 2).rev() {
        for _ in 0..(WIDTH / 2 - i) {
            output.push(' ');
        }
        output.push('\\');
        for _ in 0..(i * 2) {
            output.push(' ');
        }
        output.push('/');
        output.push('\n');
    }


    print!("{}", output);
}
