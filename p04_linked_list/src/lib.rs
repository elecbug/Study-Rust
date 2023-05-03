
#[derive(Clone)]
pub struct MyNode<T> {
    value: Option<T>,
    next: Option<Box<MyNode::<T>>>,
}

#[derive(Clone)]
pub struct MyList<T> {
    head: Box<MyNode::<T>>,
    tail: Box<MyNode::<T>>,
    length: u32,
}

impl<T> MyNode<T> {
    fn new() -> MyNode<T> {
        MyNode { 
            value: None,
            next: None,
        }
    }
}

impl<T> MyList<T> {
    pub fn new() -> MyList<T> {
        let tail = Box::new(MyNode::<T>::new());
        let head = Box::new(MyNode {
            value: None,
            next: Some(tail),
        });

        MyList {
            head,
            tail,
            length: 0,
        }
    }
}
