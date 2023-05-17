extern crate rand;
use std::io;
use rand::Rng;
fn main() {
    let mut guess = String::new();
    let low =0;

   
    println!("Hello, world!");
    println!("Please input your guess number :");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let my_int:i32 = guess.trim().parse().unwrap();
    println!("Your Code: {guess}",);
    let secret_num = rand::thread_rng().gen_range(low,my_int);
    let mul_num:i32 = secret_num*my_int;
    println!("Random number : {secret_num}");
    println!("Multiple Number : {mul_num}");

    if mul_num<20 {
        println!("Less than 20");
    }else if mul_num>=20 && mul_num<=40{
        println!("More than 20 And less than 40");
    }else{
        println!("More than 40 ");
    }
}
