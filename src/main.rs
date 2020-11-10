use std::io::{stdout, Write};

fn main() {
    print!("lis.rs> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    let answer = input.replace("(", " ( ").replace(")", " ) ");

    for t in answer.split_whitespace() {
      println!("{}", t);
    }
}
