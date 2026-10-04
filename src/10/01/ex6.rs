use std::io;

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn check(a: i32, b: i32) -> bool {
    let check: bool;
    let c = a % 10;
    let d = b % 10;
    if c == d {
        check = true;
    } else {
        check = false;
    }
    check
}

fn main() {
    println!("Insert first number to compare last digit of");
    let a = parse_num().unwrap_or_else(|_| {
        println!("fallback to 1");
        1
    });
    println!("Insert second number to compare last digit of");
    let b = parse_num().unwrap_or_else(|_| {
        println!("fallback to 101");
        101
    });

    println!("Is the last digit equal?  : {}", check(a, b));
}
