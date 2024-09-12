fn main() {
    let string = "Hello, world!";
    let index = 7;

    let char = string.chars().nth(index).unwrap();
    println!("Character at index {}: {}", index, char);
}