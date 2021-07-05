#[derive(Debug)]
struct Node<T> {
    data: Option<T>,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new<'a>() -> &'a LinkedList<T> {
        &LinkedList {
            head: Node {
                data: None,
                next: None,
            },
        }
    }
}
