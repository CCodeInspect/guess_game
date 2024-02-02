use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("please input your guess.");

    let secret_number=rand::thread_rng().gen_range(1..=100);
    
    println!("please input your guess.");
    
    loop{
    let mut guess = String::new(); //创建了一个可变的变量guess，并且将该变量绑定到一个新的空实String上

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    // let guess:u32=guess.trim().parse().expect("please input a number");
    
    let guess :u32=match guess.trim().parse(){
        Ok(num) =>num,
        Err(_) =>continue,
    };



    match guess.cmp(&secret_number){
        Ordering::Less =>println!("Too small"),
        Ordering::Greater =>println!("Too big"),
        Ordering::Equal =>println!("you win!"),

    }
    break;

    }
}