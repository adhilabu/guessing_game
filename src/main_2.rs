
fn main() {
    let s1: String = get_owner();
    let s2 = String::from("Hello");
    let s2_clone = s2.clone();
    let s3 = gives_and_get_back(s2_clone);
    println!("s1 {}, s2 {}, s3 {}", s1, s2, s3)
}

fn get_owner() -> String {
    let s1 = String::from("get owner");
    s1
}


fn gives_and_get_back(s_in: String) -> String {
    s_in
}
