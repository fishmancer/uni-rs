use std::io;

fn is_prime(i: i32) -> bool {
    if i == 1 {
        return false;
    }
    for j in 2..i {
        if i % j == 0 {
            return false;
        }
    }

    return true;
}

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn main() {
    println!("first number in range");
    let a = parse_num().unwrap_or_else(|_| {
        println!("fallback to 0");
        0
    });
    println!("first number in range");
    let b = parse_num().unwrap_or_else(|_| {
        println!("fallback to 10");
        0
    });
    print!("The prime numbers in provided range are : ");
    for i in a..=b {
        if is_prime(i) {
            print!(" {i} ")
        }
    }
    println!("");
}
