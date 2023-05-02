pub mod linked_list {

    struct Node<T> {
        value: Option<T>,
        next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        fn new() -> Node<T> {
            Node {
                value: None,
                next: None,
            }
        }

        fn push(before: &mut Node<T>) -> Node<T> {
            let result = move || Node {
                value: None,
                next: None,
            };
            before.next = Some(Box::new(result()));

            result()
        }
    }

    pub struct List<T> {
        head: Node<T>,
        tail: Node<T>,
        len: u32,
    }

    impl<T> List<T> {
        pub fn new() -> List<T> {
            let mut head = Node::<T>::new();
            let tail = Node::<T>::push(&mut head);
            List {
                head,
                tail,
                len: 0,
            }
        }

        fn at_before_node(&self, index: u32) -> Option<Box<&Node<T>>> {
            let mut now = Some(Box::new(&self.head));
            for _i  in [0..index] {
                now = Some(*Box::new(now.unwrap()));
            };

            now
        }

        pub fn get_value(&self, index: u32) -> &Option<T> {
            let now = self.at_before_node(index);
            let now = Some(*Box::new(now.unwrap()));
            let now = *now.unwrap();

            &now.value
        }

        pub fn set_value(&self, index: u32, value: &Option<T>) {
            let now = self.at_before_node(index);
            let now = Some(*Box::new(now.unwrap()));
            let now = *now.unwrap();
            let mut _now = &now.value; 
            _now = value;
        }
    }
}