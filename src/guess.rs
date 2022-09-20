use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    println!("GuessNumber");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is:{}",secret_number);
    loop{
        println!("input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("you input {}!",guess);
        let guess:u32=match guess.trim().parse(){//类型转>换 同时复用了变量名。
            Ok(num) => num,
            Err(_) => {
                println!("your input is invalid");
                continue;
            }};
        println!("input your guess");
        println!("helloworld");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("TooBig"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        };
    }
}

