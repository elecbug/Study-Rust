fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let arr: [i32; 5] = [1,2,3,4,5];
    let a = [3; 5]; //[3,3,3,3,3]

    let first = arr[0];
    
}
