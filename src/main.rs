fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    let parts: Vec<&str> = input.split_whitespace().collect();
    let a: i32 = parts[0].parse().unwrap();
    let b: i32 = parts[1].parse().unwrap();
    
    println!("{}", a + b);
}