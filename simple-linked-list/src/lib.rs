use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head: None}
    }

    pub fn len(&self) -> usize {
        // TODO: store length as instance variable?
        let mut head = &self.head;
        let mut length = 0;
        while let Some(current) = head {
            length += 1;
            head = &current.next;
        }
        length
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node{
            data: element,
            next: self.head.take()
        }));
    }

    pub fn append(&mut self, element: T) {
        let mut head = &mut self.head;
        while let Some(current) = head {
            head = &mut current.next;
        }
        *head = Some(Box::new(Node{
            data: element,
            next: None
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut node) => {
                self.head = node.next.take();
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => {
                Some(&node.data)
            }
            None => None,
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut head = &self.head;
        while let Some(current) = head {
            new_list.push(current.data.clone());
            head = &current.next;
        }
        return new_list;
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];
        let mut head = &self.head;
        while let Some(current) = head {
            head = &current.next;
            result.push(current.data.clone());
        }
        result.reverse();
        result
    }
}
