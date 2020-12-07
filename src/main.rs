use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let guess = rand::thread_rng().gen_range(1, 101);
    println!("Enter the number: ");
    let mut number;
    loop {
        number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Error On Getting Data Required!");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&number) {
            Ordering::Greater => println!("Too Small!"),
            Ordering::Less => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        };
    }
}
