

fn main() {
    let s: String = String::from("Hello");
    let length: usize = calculate_length(&s);
    println!("String is {} and it's lenght is {}", s, length)
}

fn calculate_length(s_in: &String) -> usize {
    // let s5 = s_in.push("abcd");
    let length: usize = s_in.len();
    length
}