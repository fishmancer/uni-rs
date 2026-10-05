use std::io;

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn calculate_abs(a: i32, b: i32) -> i32 {
    let c: bool = a >= b;
    let d: bool = a < b;

    (a - b) * c as i32 + (a - b) * (d as i32 * -1)
}

fn main() {
    println!("Input first number");
    let a = parse_num().unwrap_or_else(|_| {
        println!("invalid input: fallback to 1");
        1
    });
    println!("Input second number");

    let b = parse_num().unwrap_or_else(|_| {
        println!("invalid input: fallback to 2");
        2
    });

    println!("Result of a-b : {}", calculate_abs(a, b));
}
