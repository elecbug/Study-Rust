use p04_linked_list::MyList;
use p04_linked_list::Address;

fn main() {
    let mut head = MyList {
        value: 8,
        next: Address::Nil,
    };
    head.append(9);
    head.append(10);
    head.append(11);
    head.list();
    println!("The size of the list is {}", head.count());
    head.update(4, 20);
    head.update(0, 6);
    head.list();
}