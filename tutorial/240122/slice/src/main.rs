fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();


    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     // s.len()
//     return s.len();
// }

