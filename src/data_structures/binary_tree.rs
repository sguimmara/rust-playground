pub struct BinaryTree<T> where T: PartialEq, T: PartialOrd {
    root: Node<T>,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        Self { root: Node::new(value) }
    }

    pub fn insert(&mut self, value: T) {
        if value < *self.root.value {
            self.root.insert_left(value);
        } else {
            self.root.insert_right(value);
        }
    }

    pub fn size(&self) -> usize {
        let mut result = 0;

        self.root.count(&mut result);

        result
    }
}

impl<T> Into<Vec<T>> for BinaryTree<T> where T: PartialEq, T: PartialOrd  {
    fn into(self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size());
        self.root.populate(&mut result);
        result
    }
}

struct Node<T> where T: PartialEq, T: PartialOrd {
    pub value: Box<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> PartialEq for Node<T> where T: PartialEq, T: PartialOrd {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialOrd for Node<T> where T: PartialEq, T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.left.partial_cmp(&other.left) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.right.partial_cmp(&other.right)
    }
}

impl<T: PartialOrd> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value: Box::new(value), left: None, right: None }
    }

    pub fn count(&self, result: &mut usize) {
        *result += 1;

        if let Some(left) = &self.left {
            left.count(result)
        }
        if let Some(right) = &self.right {
            right.count(result)
        }
    }

    pub fn populate(self, vec: &mut Vec<T>) {
        vec.push(*self.value);

        if let Some(left) = self.left {
            left.populate(vec)
        }
        if let Some(right) = self.right {
            right.populate(vec)
        }
    }

    pub fn insert_left(&mut self, value: T) {
        match &mut self.left {
            Some(node) => {
                if value < *node.value {
                    node.insert_left(value)
                } else {
                    node.insert_right(value)
                }
            },
            None => {
                self.left = Some(Box::new(Node::new(value)))
            }
        }
    }

    pub fn insert_right(&mut self, value: T) {
        match &mut self.right {
            Some(node) => {
                if value < *node.value {
                    node.insert_left(value)
                } else {
                    node.insert_right(value)
                }
            },
            None => {
                self.right = Some(Box::new(Node::new(value)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinaryTree;

    #[test]
    fn new() {
        let tree = BinaryTree::new(0);
        assert_eq!(tree.size(), 1);
        let asvec: Vec<usize> = tree.into();
        assert_eq!(asvec.len(), 1);
        assert_eq!(asvec[0], 0);
    }

    #[test]
    fn feature() {
        let mut tree = BinaryTree::new(0);

        tree.insert(9);
        tree.insert(2);
        tree.insert(-1);
        tree.insert(-100);
        tree.insert(0);
        tree.insert(0);

        let asvec: Vec<i32> = tree.into();

        assert_eq!(asvec.len(), 7);

        assert_eq!(asvec[0], -100);
        assert_eq!(asvec[1], -1);
        assert_eq!(asvec[2], 0);
        assert_eq!(asvec[3], 0);
        assert_eq!(asvec[4], 0);
        assert_eq!(asvec[5], 2);
        assert_eq!(asvec[6], 9);
    }
}