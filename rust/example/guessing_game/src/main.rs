use std::io;    // prelude
use rand::Rng;  // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏!!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("无法读取行");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };
        println!("你猜测的数是: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}