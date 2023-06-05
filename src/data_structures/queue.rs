use core::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug,Clone)]
struct Node<T> {
    data: T,
    next_node: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Node<T>>> {
        return Rc::new(RefCell::new(Node { 
            data, 
            next_node: None 
        }));
    }
}

#[derive(Debug, Clone)]
struct Queue<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        return Queue { head: None, tail: None };
    }



    pub fn enqueue(&mut self, data: T) {
        let new_node = Node::new(data);
        let new_node_weak = Rc::downgrade(&new_node);

        match self.tail.take() {
            Some(tail) => {
                if let Some(tail_strong) = tail.upgrade() {
                    tail_strong.borrow_mut().next_node = Some(new_node.clone());
                } else {
                    self.head = Some(new_node.clone());
                }
                self.tail = Some(new_node_weak);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node_weak);

            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {

        if let Some(head) = &self.head.take() {
            let current_head = Some(head.borrow_mut().data.to_owned());
            self.head = head.borrow_mut().next_node.to_owned();
            if self.head.is_none(){
                self.tail = None;
            }
            current_head
        } else {
            None    
        }
    }


    pub fn peek(&self) -> Option<T> {
        match &self.head {
            Some(head) => Some(head.borrow_mut().data.to_owned()),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool{
        return match self.head {
            Some(_) => false,
            None => true,
        }
    }
}
pub fn queue_test(){
    let mut queue = Queue::new();

    println!("{}", queue.is_empty());
    queue.enqueue(4);
    queue.enqueue(3);
    queue.enqueue(2);
    queue.enqueue(1);

    println!("{:?}", queue);

    match queue.peek() {
        Some(data) => println!("{}", data),
        None => println!("Could not peek"),
    };


    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().unwrap());

    match queue.peek() {
        Some(data) => println!("{}", data),
        None => println!("Could not peek"),
    };

    println!("{}", queue.dequeue().unwrap());
    println!("{:?}", queue);
    println!("{}", queue.dequeue().unwrap());
    println!("{:?}", queue);
    match queue.dequeue(){
        Some(dongs) => println!("{}", dongs),
        _ => println!("dings"),
    }
}
