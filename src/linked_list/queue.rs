pub struct Node<T> {
    value: T,
    next: Option<T>
}

pub struct Queue<T> {
    // head
    head: Option<Node<T>>,
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
                self.head = Some(Node {
                    value: val,
                    next: None
                });
            }
            
        }
    }
}