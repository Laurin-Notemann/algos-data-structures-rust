use std::fmt::Display;

#[derive(Debug)]
struct List<'a, T: Display> {
   pub first: Option<&'a Node<'a, T>>,
   pub last: Option<&'a Node<'a, T>>,
   pub current: Option<&'a Node<'a, T>>,
   //next_item: Option<Node<'a, T>>,
   // pub previous_item: Option<&'a Node<'a, T>>,
}

impl<'a, T: Display> List<'a, T> {
    pub fn new() -> Self{
        return Self{
            first: None,
            last: None,
            current: None,
            // previous_item: None,
        };
    }

    pub fn is_empty(&self) -> bool {
        if self.first.is_none() {
            return true;
        } else {
            return false;
        }
    }

    pub fn push(&mut self, item: Option<&'a Node<'a, T>>) {
        if self.is_empty() {
            self.first = item;
            self.last = item;
        } else {
            //self.previous_item = self.last;
            self.last.unwrap().set_next_item(item);
            item
                .unwrap_or_else(|| println!("Node value is none"))
                .set_previous_item(self.last);
            self.last = item;
        }
    }

    pub fn pop() {
    }
    // Implement an Iterator
    //    pub fn next(&self) -> T {
    //
    //    }
    //    pub fn insertAt(){}
    //    pub fn removeAt() {}
    //    pub fn removeObj() {}
    //    pub findAt() {}
    //    pub findObj() {}
    //
}

#[derive(Debug)]
struct Node<'a, T: Display> {
   pub content: T,
   pub next_item: Option<&'a Node<'a, T>>,
   pub previous_item: Option<&'a Node<'a, T>>,
}

impl<'a, T: Display> Node<'a, T> {
   pub fn new(content: T) -> Self{
        return Self {
            content,
            next_item: None,
            previous_item: None,
        };
    }

    pub fn set_next_item(&mut self, next_node: &'a Self) {
        self.next_item = Some(next_node);
    }

    pub fn set_previous_item(&mut self, previous_node: &'a Self) {
        self.previous_item = Some(previous_node);
    }
}

pub fn list_test() {
    let mut new_list: List<isize> = List::new();

    let first_item: Node<isize> = Node::new(5);
    let second_item: Node<isize> = Node::new(6);
    let third_item: Node<isize> = Node::new(7);

    new_list.push(Some(&first_item));
    new_list.push(Some(&second_item));
    new_list.push(Some(&third_item));

    println!("THis is the list {:?}", new_list);
    println!("This is the item {:?}", first_item);
}
