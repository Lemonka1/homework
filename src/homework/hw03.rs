
fn main() {
   
    const WIDTH: usize = 20; 
    const HEIGHT: usize = 10;

    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!(); 
    }

   
    println!("*{}*", " ".repeat(WIDTH - 2)); 
}
//good