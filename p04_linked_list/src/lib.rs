#[derive(Clone)]
pub enum Address {
    Address(Box<MyList>),
    Nil,
}

#[derive(Clone)]
pub struct MyList {
    pub value: u32,
    pub next: Address,
}

impl MyList {
    pub fn append(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(elem);
            }
            Address::Nil => {
                let node = MyList {
                    value: elem,
                    next: Address::Nil,
                };
                self.next = Address::Address(Box::new(node))
            }
        }
    }
    
    pub fn delete(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            Address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }

    pub fn count(&self) -> u32 {
        match self.next {
            Address::Address(ref newaddress) => 1 + newaddress.count(),
            Address::Nil => 1,
        }
    }

    pub fn list(&self) {
        if self.value == 0 {
            println!("The list is empty")
        } else {
            println!("{}", self.value);
            match self.next {
                Address::Address(ref next_address) => next_address.list(),
                Address::Nil => {}
            }
        }
    }

    pub fn update(&mut self, index: u32, elem: u32) {
        let mut i = 0;
        let mut j = self;
        if i == index {
            j.value = elem;
        }
        while i < index {
            // println!("{}", j.value);
            match j.next {
                Address::Address(ref mut next_address) => j = next_address,
                Address::Nil => {}
            }
            i = i + 1;
        }
        j.value = elem;
    }
}

