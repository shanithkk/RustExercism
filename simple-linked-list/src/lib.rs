use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current_node = &self.head;

        while let Some(node) = current_node {
            current_node = &node.next;
            count += 1;
        }

        count
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        self.to_rev_iterator().collect()
    }

    fn to_rev_iterator(self) -> std::vec::IntoIter<T> {
        let mut vector = Vec::new();
        let mut current_node = self.head;

        while let Some(node) = current_node {
            current_node = node.next;
            vector.push(node.data);
        }

        vector.into_iter()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = SimpleLinkedList::new();

        for x in iter {
            linked_list.push(x);
        }

        linked_list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        self.to_rev_iterator().rev().collect()
    }
}