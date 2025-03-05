pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        let raw_node = Box::into_raw(new_node);

        unsafe {
            if self.tail.is_null() {
                self.head = Some(Box::from_raw(raw_node));
                self.tail = raw_node;
            } else {
                (*self.tail).next = Some(Box::from_raw(raw_node));
                self.tail = raw_node;
            }
        }
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }
            self.len -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}