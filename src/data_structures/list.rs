use std::{rc::{Weak, Rc}, fmt::{Display, Debug}, process::id};
use core::cell::RefCell;

#[derive(Debug, Clone)]
struct Node<T: Display> {
    data: T,
    previous_node: Option<Rc<RefCell<Node<T>>>>, 
    next_node: Option<Rc<RefCell<Node<T>>>>, 
}

impl<T: Display> Node<T>{
    pub fn new(data: T) -> Rc<RefCell<Node<T>>> {
        return Rc::new(RefCell::new(Node {
            data,
            previous_node: None,
            next_node: None 
        }));
    }
}

#[derive(Debug, Clone)]
struct List<T: Display> {
    pub length: i32,
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Weak<RefCell<Node<T>>>>,
    pub current: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Clone + PartialEq + Display + Debug> List<T> {
    pub fn new() -> Self{
        return Self{
            length: 0,
            head: None,
            tail: None,
            current: None,
        };
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, data: T) {
        let new_node = Node::new(data);
        let new_weak_node_one = Rc::downgrade(&new_node);
        let new_weak_node_two = Rc::downgrade(&new_node);
        self.length += 1;

        match &self.tail.take() {
            Some(tail) => {
                let tail_upgrade = tail.upgrade().unwrap(); 
                tail_upgrade.borrow_mut().next_node = Some(Rc::clone(&new_node));
                new_node.borrow_mut().previous_node = Some(tail_upgrade);
                self.tail = Some(new_weak_node_one);
            },
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.current = Some(new_weak_node_two);
                self.tail = Some(new_weak_node_one);
            },
        }
    }

    pub fn pop(&mut self) -> Option<T>{
        if let Some(tail) =  &self.tail.take(){
            self.length -= 1;
            let tail_strong = tail.upgrade().unwrap();
            let mut borrowed_tail_strong = tail_strong.borrow_mut();
            let current_value = borrowed_tail_strong.data.to_owned();
            match borrowed_tail_strong.previous_node.take() {
                Some(prev_node) => {
                    prev_node.borrow_mut().next_node = None;
                    let tail_prev_weak = Rc::downgrade(&Rc::clone(&prev_node));
                    self.tail = Some(tail_prev_weak);
                    return Some(current_value);
                },
                None => {
                    self.tail = None;
                    self.current = None;
                    self.head = None;
                    return Some(current_value);
                },
            }
        }
        return None;
    }
    
    pub fn instert_at(&mut self, idx: i32, data: T) {
        if idx == self.length {
            self.push(data.to_owned());
        }
        match self.find_node_by_idx(idx) {
            Some(node) => {
                let new_node = Node::new(data);
                let mut previous_node_one = node.borrow_mut();
                match previous_node_one.previous_node.as_ref() {
                    Some(previous_node) => {
                        previous_node.borrow_mut().next_node = Some(Rc::clone(&new_node));
                        new_node.borrow_mut().previous_node = Some(Rc::clone(&previous_node));
                        new_node.borrow_mut().next_node = Some(Rc::clone(&node));
                        previous_node_one.previous_node = Some(Rc::clone(&new_node));
                    },
                    None => {
                        new_node.borrow_mut().next_node = Some(Rc::clone(&node));
                        previous_node_one.previous_node = Some(Rc::clone(&new_node));
                    },
                }
            
            },
            None => return,
        }

    }

    pub fn remove_at(&mut self, idx: i32) -> Option<T> {
        if idx >= self.length {
            return None;
        } else if idx == self.length - 1 {
            return self.pop();
        } else if self.length == 0 {
            return None;
        }
        

        if let Some(node) = self.find_node_by_idx(idx) {
            self.length -= 1;
            let node = node.borrow_mut();
            let next_node = node.next_node.as_ref().unwrap();
            let value = node.data.to_owned();
            match node.previous_node.as_ref() {
                Some(previous_node) => {
                    previous_node.borrow_mut().next_node = Some(Rc::clone(&next_node));
                    next_node.borrow_mut().previous_node = Some(Rc::clone(&previous_node));
                },
                None => {
                    next_node.borrow_mut().previous_node = None;
                    let weak_next_node = Rc::downgrade(next_node);
                    self.current = Some(weak_next_node);
                    self.head = Some(Rc::clone(&next_node));

                },
            } 
            
            return Some(value);
        }
        return None;
    }

    pub fn find_at(&mut self, idx: i32) -> Option<T> {
        if let Some(node) = self.find_node_by_idx(idx) {
            let value = node.borrow_mut().data.to_owned();
            return Some(value);
        }
        return None;
    }

    pub fn find_obj(&mut self, data: T) -> Option<i32> {
        if let Some(idx) = self.find_idx_by_obj(data) {
            return Some(idx);
        }
        return None;
    }

    pub fn remove_obj(&mut self, data: T) -> Option<T> {
        if let Some(idx) = self.find_idx_by_obj(data) {
            return self.remove_at(idx);
        }
        
        return None;
    }

    fn find_idx_by_obj(&mut self, data: T) -> Option<i32>{
        for i in 0..self.length {
            let current_node = match &self.current {
                Some(current) => current.upgrade().unwrap(),
                None => return None,
            };

            if data == current_node.borrow_mut().data.to_owned() {
                let head_node = match &self.head {
                    Some(head) => head,
                    // list is empty
                    None => {
                        self.current = None;
                        return None;
                    },
                };
                let weak_node = Rc::downgrade(head_node);
                self.current = Some(weak_node);
                return Some(i);
            }
            match self.iterate_current(&current_node) {
                None => return None,
                _ => {},
            }
        }
        return None;    
    }

    fn iterate_current(&mut self, current_node: &Rc<RefCell<Node<T>>>) -> Option<bool>{
        match current_node.borrow_mut().next_node.as_ref() {
            Some(next_node) => {
                self.current = Some(Rc::downgrade(next_node));
            },
            None => { 
                let head_node = match &self.head {
                    Some(head) => head,
                    None => {
                        self.current = None;
                        return None;
                    },
                };
                let weak_node = Rc::downgrade(head_node);
                self.current = Some(weak_node);
                return None;
            },
        };
        return Some(true);
    }

    fn find_node_by_idx (&mut self, idx: i32) -> Option<Rc<RefCell<Node<T>>>> {
        if idx >= self.length {
            return None;
        } else if self.length == 0 {
            return None;
        } 

        for i in 0..self.length {
            let current_node = match &self.current {
                Some(current) => current.upgrade().unwrap(),
                None => return None,
            };

            if idx == i{
                let head_node = match &self.head {
                    Some(head) => head,
                    // list is empty
                    None => {
                        self.current = None;
                        return None;
                    },
                };
                let weak_node = Rc::downgrade(head_node);
                self.current = Some(weak_node);
                return Some(current_node);
            }
            match self.iterate_current(&current_node) {
                None => return None,
                _ => {},
            }
        }
        return None;    
    }    
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;

        let mut head = self.head.to_owned();
        while let Some(n) = head {
            write!(f, "{}", n.borrow().data)?;
            head = n.borrow().next_node.to_owned();
            if head.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_list() {

    }
}
pub fn list_test() {
    let mut new_list = List::new();

    println!("List empty one (true): {}", new_list.is_empty());
    println!("List first time: {:#?}", new_list);

    new_list.push(1);
    new_list.push(2);
    new_list.push(5);
    new_list.push(56);
    new_list.push(7);
    new_list.push(23);
    new_list.push(9);
    new_list.push(234);

    println!("List one time: {}", new_list);
    println!("Pop 0 (234): {:?}", new_list.pop());
    println!("Pop 2 (9): {:?}", new_list.pop());
    println!("Remove at2 1 (5): {:?}", new_list.remove_at(2));

    println!("List second time: {}", new_list);
    
    println!("Remove obj7 1 (7): {:?}", new_list.remove_obj(7));

    new_list.instert_at(2, 111);

    println!("List third time (111 should be at idx 2 between 56 and 23): {}", new_list);

    println!("Find obj111 1 (idx: 2): {:?}", new_list.find_obj(111));
    println!("Find at1 (2): {:?}", new_list.find_at(1));

    println!("List empty two (false): {}", new_list.is_empty());

}
