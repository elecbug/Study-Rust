fn main() {
    let a = Some(3u8);
    match a {
        Some(max) => println!("{}", max),
        _ => (),
    };

    let a = Some(3u8);
    if let Some(max) = a {
        println!("{}", max);
    };
}
