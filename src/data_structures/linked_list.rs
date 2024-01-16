use std::ops::Deref;

pub struct LinkedList<T> {
    first: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { first: None }
    }

    pub fn push(&mut self, value: T) {
        match &mut self.first {
            Some(node) => node.push(value),
            None => self.first = Some(Box::new(Node::new(value)))
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &mut self.first {
            None => None,
            Some(node) => {
                if node.has_next() {
                    return node.pop();
                } else {
                    let v = self.first.take();
                    return v.map(|x| { x.value });
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match &self.first {
            None => 0,
            Some(node) => node.len()
        }
    }
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct NodeIterator<'a, T> {
    current: Option<&'a Node<T>>
}

impl<'a, T> NodeIterator<'a, T> {
    pub fn new(node: &'a Node<T>) -> Self {
        Self { current: Some(node) }
    }
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.current {
            self.current = result.next_as_ref();
            return Some(result);
        }

        return None;
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    pub fn push(&mut self, value: T) {
        match &mut self.next {
            None => {
                self.next = Some(Box::new(Node::new(value)));
            },
            Some(node) => {
                node.push(value)
            }
        }
    }

    fn take_next(&mut self) -> Option<T> {
        let res = self.next.take();
        match res {
            None => None,
            Some(val) => Some(val.value)
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(next) = &mut self.next {
            if let None = next.next {
                // self.next = None;
                return self.take_next()
            } else {
                return next.pop();
            }
        }

        panic!();
    }

    pub fn next_as_ref(&self) -> Option<&Node<T>> {
        match &self.next {
            None => return None,
            Some(v) => return Some(&v),
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn clear(&mut self) {
        self.next = None;
    }

    fn length_internal(&self, counter: &mut usize) {
        *counter += 1;
        if let Some(next) = &self.next {
            next.length_internal(counter);
        }
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        self.length_internal(&mut counter);
        return counter;
    }
}

#[cfg(test)]
mod test {
    use super::{Node, NodeIterator, LinkedList};

    #[test]
    fn pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(3), list.pop());
        list.push(5);
        assert_eq!(Some(5), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
        assert_eq!(None, list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn deref() {
        let node = Node::new(5);
        assert_eq!(5, *node);
    }

    #[test]
    fn len() {
        let mut node = Node::new(5);
        assert_eq!(1, node.len());

        node.push(2);
        assert_eq!(2, node.len());

        node.push(3);
        assert_eq!(3, node.len());
    }

    #[test]
    fn iter() {
        let mut node = Node::new(5);
        node.push(2);
        node.push(3);

        let mut iter = NodeIterator::new(&node);

        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
    }
}