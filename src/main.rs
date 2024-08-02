use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");
    let secrete_num: u32 = rand::thread_rng().gen_range(1, 100);
    println!("{}", secrete_num);
    loop {
        println!("please input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your guess is:{}", guess);

        match guess.cmp(&secrete_num) {
            Ordering::Less => println!("{}", "too small!!".red()),
            Ordering::Greater => println!("{}", "toooo big!!".red()),
            Ordering::Equal => {
                println!("{}", "your guess is correct...💀".green());
                break;
            }
        }
    }
}
