use std::io;

fn max_two(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn main() {
    println!("Now insert first number");
    let a = parse_num().unwrap_or_else(|_| {
        println!("fallback to 0");
        0
    });
    println!("Now insert second number");
    let b = parse_num().unwrap_or_else(|_| {
        println!("fallback to 0");
        0
    });
    println!("Bigger number of the two is: {}", max_two(a, b));
}
