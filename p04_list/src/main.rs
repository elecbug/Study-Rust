use p04_list::*;

fn main() {
    let mut list = MyList::<i32>::new();
    list.insert(0, &Some(1));
    list.insert(0, &Some(2));
    list.insert(0, &Some(3));
    list.insert(0, &Some(4));
    list.insert(4, &Some(5));
}