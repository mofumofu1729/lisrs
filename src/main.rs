use std::io::{stdout, Write};

fn tokenize(s: &str) -> Vec<String> {
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<String> = spreaded.split_whitespace().map(|item| item.to_string()).collect();

    return tokens;
}

fn main() {
    print!("lis.rs> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let split = tokenize(&input);

    let op = &split[1];
    let a: i32 = split[2].parse().unwrap();
    let b: i32 = split[3].parse().unwrap();

    if op == "+" {
      println!("{}", a + b);
    } else if op == "-" {
      println!("{}", a - b);
    }
}
