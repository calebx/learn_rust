use std::io;
use std::cmp::Ordering::*;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number: u32 = rng.gen_range(0, 100);
    println!("the secret_number is {}", secret_number);

    loop {
        println!("input a number to guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail to read line");
        
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_number.cmp(&secret_number) {
            Less => println!("too small"),
            Greater => println!("too big"),
            Equal => {
                println!("yes, you win");
                break;
            }
        }
    }
}
