fn main() {
    let some_ex_1 = None;
    let some_ex_2 = Some(5);
    let some_ex_3 = Some(7);
    test(some_ex_1);
    test(some_ex_2);
    test(some_ex_3);

}

fn test(var: Option<i32>) {
    match var {
        None => {
            println!("It is None");
        }
        Some(5) => {
            println!("Number is {}", 5);
        }
        _ => {
            println!("Other Cases {:?}", var);
        }
    }

}