#[derive(Clone)]
pub struct MyNode<T> {
    value: Option<T>,
    next: Option<Box<MyNode::<T>>>,
}

#[derive(Clone)]
pub struct MyList<T> {
    head: Box<MyNode::<T>>,
    length: u32,
}

impl<T> MyNode<T> {
    fn new() -> MyNode<T> {
        MyNode { 
            value: None,
            next: None,
        }
    }

    fn add(&mut self, next_value: &Option<T>) {
        match self.next {
            Some(_) => {
                let insert = Box::new(MyNode {
                    value: next_value.as_ref(),
                    next: None,
                });
                insert.next = Some(_);

                Some(insert)
            },
            None => {
                let insert = Box::new(MyNode {
                    value: next_value.as_ref(),
                    next: None,
                });

                Some(insert)
            },
        }; 
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
            length: 0,
        }
    }

    fn at_node(&self, index: u32) -> &Box<MyNode<T>> {
        let mut now = &self.head;

        for _i in [0..index] {
            now = match &now.next {
                Some(value)=> &value,
                None => break,
            };
        };
        
        now
    }

    pub fn at(&self, index: u32) -> &Option<T> {
        &self.at_node(index + 1).value
    }

    pub fn insert(&self, index: u32, value: &Option<T>) {
        self.at_node(index).add(&value);
    }
}