fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s;
    {
        let r2 = &mut s;
    }
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(" world");
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}
