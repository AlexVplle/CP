fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut numerator: u32 = 1;
    let mut denominator: u32 = 1;
    for _ in 0..input.trim().parse().unwrap() {
        numerator += denominator;
        std::mem::swap(&mut numerator, &mut denominator);
    }
    println!("{}", numerator);
}
