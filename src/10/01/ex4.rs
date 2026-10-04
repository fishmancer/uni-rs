use std::io;

fn sum_up_to(a: i32) -> i32 {
    let mut sum = 0;
    for i in 0..=a {
        sum += i;
    }
    sum
}

fn parse_num() -> io::Result<i32> {
    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    x.trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}
fn main() {
    println!("Insert number to sum up to:");
    let a = parse_num().unwrap_or_else(|_| {
        println!("fallback to 1:");
        1
    });
    let sum = sum_up_to(a);
    println!("sum is : {sum}");
}
