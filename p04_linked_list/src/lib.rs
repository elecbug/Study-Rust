#[derive(Clone)]
pub enum Address<T> {
    Address(Box<MyNode::<T>>),
    Null,
}

#[derive(Clone)]
pub struct MyList<T> {
    head: MyNode::<T>,
    tail: MyNode::<T>,
    length: u32,
}

#[derive(Clone)]
pub struct MyNode<T> {
    value: T,
    next: Address<T>,
}

impl<T> MyNode<T> {
}

impl<T> MyList<T> {
}
