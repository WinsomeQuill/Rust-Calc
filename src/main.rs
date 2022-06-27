use std::io::{
    self,
    Read
};
use std::num::ParseIntError;
use std::io::{
    stdin,
    stdout,
    Write
};

fn input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

fn main() {
    let mut operator: String;
    loop {
        print!("Введите A: ");

        let a = match input().parse::<i32>() {
            Ok(a) => a,
            Err(error) => {
                println!("А может быть только числом!");
                continue;
            },
        };

        print!("Введите B: ");
        let b = match input().parse::<i32>() {
            Ok(b) => b,
            Err(error) => {
                println!("B может быть только числом!");
                continue;
            },
        };

        print!("Введите Оператор: ");
        operator = input();

        match operator.as_str() {
            "+" => println!("{}", a + b),
            "-" => println!("{}", a - b),
            "*" => println!("{}", a * b),
            "/" => {
                if a == 0 {
                    println!("Делить на ноль нельзя!");
                    continue;
                }
                println!("{}", a / b);
            },
            _ => println!("Такого оператора нету!"),
        };
    };
}