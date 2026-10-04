use std::{io, process};

#[derive(PartialEq)]
enum TType {
    Error,
    Scalene,
    Equilateral,
    Isosceles,
    Init,
}

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    if x.trim() == "e" {
        process::exit(0);
    }
    let n = x
        .trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if n == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Number cannot be zero",
        ));
    }
    Ok(n)
}

fn main() {
    let mut triangle_type;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    loop {
        triangle_type = TType::Init;
        if triangle_type != TType::Error {
            println!("Please insert the first side of the triangle or Press 'e' to quit");
            a = parse_num().unwrap_or_else(|_| {
                println!("invalid Input");
                triangle_type = TType::Error;
                0
            })
        }
        if triangle_type != TType::Error {
            println!("Please insert the second side of the triangleor Press 'e' to quit");
            b = parse_num().unwrap_or_else(|_| {
                println!("invalid Input");
                triangle_type = TType::Error;
                0
            })
        }
        if triangle_type != TType::Error {
            println!("Please insert the third side of the triangle or Press 'e' to quit");
            c = parse_num().unwrap_or_else(|_| {
                println!("invalid Input");
                triangle_type = TType::Error;
                0
            })
        }
        if !(a <= b + c && b <= a + c && c <= b + a) {
            triangle_type = TType::Error;
        }
        if triangle_type != TType::Error {
            if a == b && b == c {
                triangle_type = TType::Equilateral;
            } else if a == b || b == c || c == a {
                triangle_type = TType::Isosceles;
            } else {
                triangle_type = TType::Scalene;
            }
        }

        match triangle_type {
            TType::Init => {
                println!("Error, Triangle should not be in this state at this point of execution.")
            }
            TType::Error => println!("Triangle is Invalid."),
            TType::Scalene => println!("Traingle is Scalene!"),
            TType::Isosceles => println!("Traingle is Isosceles"),
            TType::Equilateral => println!("Triangle is Equilateral"),
        }
    }
}
