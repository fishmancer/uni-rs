use std::io;

fn parse_bool() -> io::Result<bool> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<bool>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn main() {
    println!("Please insert a bool.");
    let a = parse_bool().unwrap_or_else(|_| {
        println!("should be a bool, fallback to true");
        true
    });
    println!("Please insert another bool.");
    let b = parse_bool().unwrap_or_else(|_| {
        println!("should be a bool, fallback to true");
        true
    });
    let c = match (a, b) {
        (true, true) => true,
        (true, false) => false,
        (false, false) => true,
        (false, true) => true,
    };
    println!("Logical result of implication is: {c}");
}
