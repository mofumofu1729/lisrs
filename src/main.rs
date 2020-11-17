use std::io::{stdout, Write};

fn tokenize(s: &str) -> Vec<String> {
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<String> = spreaded.split_whitespace().map(|item| item.to_string()).collect();

    return tokens;
}

fn read_from(tokens: Vec<String>) -> Vec<String>{
    if tokens.len() == 0 {
        println!("unexpected EOF while reading");
    }

    return tokens;
}

fn main() {
    print!("lis.rs> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let split = tokenize(&input);

    let tokens = read_from(split);

    let op = &tokens[1];
    let a: i32 = tokens[2].parse().unwrap();
    let b: i32 = tokens[3].parse().unwrap();

    if op == "+" {
      println!("{}", a + b);
    } else if op == "-" {
      println!("{}", a - b);
    } else if op == "*" {
      println!("{}", a * b);
    } else if op == "/" {
      println!("{}", a / b);
    }
}
