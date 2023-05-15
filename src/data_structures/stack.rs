use std::mem::replace;

type Link<T> = Option<Box<T>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    previous_node: Link<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
       return Node{ data, previous_node: None }; 
    }
}

#[derive(Debug)]
struct Stack<T> {
    head: Link<Node<T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        return Stack{ head: None };
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = replace(&mut self.head, None) {
            self.head = match head.previous_node {
                Some(node) => Some(node),
                None => None,
            };
            return Some(head.data);
        }
        return None;
    }

    pub fn push(&mut self, data: T) {
        let mut new_node = Node::new(data);

        if let Some(head) = replace(&mut self.head, None) {
            new_node.previous_node = Some(head);
        }

        self.head = Some(Box::new(new_node));
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(head) => Some(&head.data),
            None => None,
        }
    }

    pub fn peek_next(&self) -> Option<&T> {
        return match &self.head {
            Some(head) => match &head.previous_node {
                Some(node) => Some(&node.data),
                None => None,
            }
            None => None,
        };
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
           Some(_) => false,
           None => true,
        }
    }
}


pub fn test_stack() {
    let mut stack = Stack::new();

    println!("{}", stack.is_empty());
    stack.push(4);
    stack.push(3);
    stack.push(2);
    stack.push(1);

    println!("{:?}", stack);

    match stack.peek() {
        Some(data) => println!("{}", data),
        None => println!("Could not peek"),
    };

    match stack.peek_next() {
       Some(data) => println!("{}", data),
       None => println!("Could not peek next"),
    };

    println!("{}", stack.pop().unwrap());
    println!("{}", stack.pop().unwrap());

    match stack.peek() {
        Some(data) => println!("{}", data),
        None => println!("Could not peek"),
    };

    match stack.peek_next() {
       Some(data) => println!("{}", data),
       None => println!("Could not peek next"),
    };
}
