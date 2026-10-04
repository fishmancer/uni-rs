use std::io;

fn factorial_recursive(number: i32) -> i32 {
    // Base Case
    if number <= 1 {
        return 1;
    }

    // Recursive Case
    return number * factorial_recursive(number - 1);
}

fn printline(n: i32) {
    let mut computed_c: i32;

    for c in 0..=n {
        computed_c = factorial_recursive(n) / (factorial_recursive(n - c) * factorial_recursive(c));
        print!("{computed_c}");
    }

    println!("");
}

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn main() {
    println!("Insert number of rows!");
    let x = parse_num().unwrap_or_else(|_| {
        println!("Fallback value for x: ");
        1
    });
    println!("{x}");

    for n in 0..=x {
        printline(n);
    }

    //commento test
}
