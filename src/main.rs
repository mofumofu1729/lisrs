use std::io::{stdout, Write};

fn tokenize(s: &str) -> Vec<String> {
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<String> = spreaded
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();

    return tokens;
}

fn read_from(tokens: Vec<String>) -> Vec<String> {
    if tokens.len() == 0 {
        println!("unexpected EOF while reading");
    }

    return tokens;
}

fn eval(split: std::vec::Vec<std::string::String>) -> i32 {
    let tokens = read_from(split);

    let op = &tokens[1];
    let a: i32 = tokens[2].parse().unwrap();
    let b: i32 = tokens[3].parse().unwrap();

    if op == "+" {
        return a + b;
    } else if op == "-" {
        return a - b;
    } else if op == "*" {
        return a * b;
    } else if op == "/" {
        return a / b;
    } else {
        return 0; // FIXME
    }
}

fn main() {
    loop {
        print!("lis.rs> ");
        std::io::stdout().flush().unwrap();

        let mut exp = String::new();
        std::io::stdin().read_line(&mut exp).ok();

        let val = eval(tokenize(&exp));
        println!("{}", val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "(+ 1 2)";

        let expected = vec!["(", "+", "1", "2", ")"];
        let actual = tokenize(input);

        for (e, a) in expected.iter().zip(actual.iter()) {
            assert_eq!(e, a);
        }
    }
}
