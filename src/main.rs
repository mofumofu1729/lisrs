use std::io::{stdout, Write};

fn main() {
    print!("lis.rs> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let answer = input.replace("(", " ( ").replace(")", " ) ");

    let split: Vec<&str> = answer.split_whitespace().collect();

    let op = split[1];
    let a: i32 = split[2].parse().unwrap();
    let b: i32 = split[3].parse().unwrap();

    if op == "+" {
      println!("{}", a + b);
    } else if op == "-" {
      println!("{}", a - b);
    }
}
