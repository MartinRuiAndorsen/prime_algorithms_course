pub struct Node<T> {
    pub value: T,
    next: Option<T>
}

impl<T> Node {
    pub fn new(val: T) {
        return Node {
            
        }
    }

    pub fn add(&mut self, val: T) {
        match &self.next {
            Some(next) => {
                next.add(val);
            },
            None => {
                next = 
            }

        }
    }
}

pub struct Queue<T> {
    // head
    pub head: Option<Node<T>>,
    // Tail
    tail: Option<Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { head: None, tail: None}
    }

    pub fn add(&mut self, val: T) {
        match &self.head { // THIS LOGIC NOT CORRECT
            None => {
                self.head = Some(Node {
                    value: val,
                    next: None
                });
            },
            Some(head) => {
                head.next.add(val)
            }
            
        }
    }
}