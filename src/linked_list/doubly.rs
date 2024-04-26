pub struct List<T> {
    val: T,
    prev: *mut Option<List<T>>,
    next: *mut Option<List<T>>,
}

// pub fn Set(node: Node<T>, value: T) {
//     node.val = value;
// }