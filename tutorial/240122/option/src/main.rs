fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        //_ => ...
    }
}
