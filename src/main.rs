use std::io::{self, Read};
use std::num::ParseIntError;

fn input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}

fn main() {
    let mut operator;
    loop {
        print!("Введите A: ");

        let a = match input().parse::<i32>() {
            Ok(a) => a,
            Err(error) => { println!("А может быть только числом!"); continue; },
        };

        print!("Введите B: ");
        let b = match input().parse::<i32>() {
            Ok(b) => b,
            Err(error) => { println!("B может быть только числом!"); continue; },
        };

        print!("Введите Оператор: ");
        operator = input();

        if operator == "+" {
            println!("{}", a+b);
        } else {
            if operator == "-" {
                println!("{}", a-b);
            } else {
                if operator == "*" {
                    println!("{}", a*b);
                } else {
                    if operator == "/" {
                        println!("{}", a/b);
                    } else {
                        println!("Такого оператора нету!");
                    }
                }
            }
        } 
    }
}
