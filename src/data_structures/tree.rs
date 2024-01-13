use crate::traits::functor::Functor;

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: Box<T>,
    children: Option<Vec<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value: Box::new(value), children: None }
    }

    pub fn add(&mut self, value: T) {
        let new_node = Node::new(value);

        match &mut self.children {
            Some(children) => children.push(new_node),
            None => {
                self.children = Some(vec![new_node]);
            }
        }
    }

    fn populate(self, target: &mut Vec<T>) {
        target.push(*self.value);

        if let Some(children) = self.children {
            for child in children {
                child.populate(target)
            }
        }
    }

    fn size(&self, accum: &mut usize) {
        *accum += 1;

        if let Some(children) = &self.children {
            for child in children {
                child.size(accum)
            }
        }
    }
}

pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        return Self { root: Node::new(root) };
    }

    fn with_root(root: Node<T>) -> Self {
        return Self { root }
    }

    pub fn root(&self) -> &Node<T> {
        &self.root
    }

    pub fn size(&self) -> usize {
        let mut result = 0;
        self.root.size(&mut result);

        result
    }
}

// pub struct TreeIterator<'a, T> {
//     stack: Vec<&'a Node<T>>,
//     child_index: usize,
// }

// impl<'a, T> TreeIterator<'a, T> {
//     pub fn new(tree: &'a Tree<T>) -> Self {
//         let stack = vec![&tree.root];
//         let child_index = 0;
//         return Self { stack, child_index }
//     }
// }

// impl<'a, T> Iterator for TreeIterator<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         let current = self.stack.last();
//         match current {
//             None => None,
//             Some(node) => {
//                 match node.children {
//                     None => { self.stack.pop(); self.child_index += 1 }
//                     Some(_)
//                 }
//                 return Some(node.value.as_ref())
//             },
//         }
//     }
// }

impl<T> Functor<T> for Tree<T> {
    type Functor<U> = Tree<U>;

    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Self::Functor<U> {
        Tree::with_root(self.root.fmap(f))
    }
}

impl<T> Functor<T> for Node<T> {
    type Functor<U> = Node<U>;

    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Self::Functor<U> {
        let value = f(&self.value);
        let mut result = Node::new(value);

        match &self.children {
            None => return result,
            Some(children) => {
                let mapped_children = children.fmap(|n| { Node::new(f(&n.value)) });
                result.children = Some(mapped_children);
                return result;
            }
        }
    }
}

impl<T> Into<Vec<T>> for Tree<T> {
    fn into(self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        self.root.populate(&mut result);
        return result;
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use crate::traits::functor::Functor;

    use super::Tree;

    #[test]
    fn tree_constructor() {
        let tree = Tree::new(5);

        assert_eq!(tree.root.value.as_ref(), &5);
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn fmap() {
        let odd = Tree::new(5).fmap(|x| { x % 2 == 0 });
        let even = Tree::new(4).fmap(|x| { x % 2 == 0 });

        assert_eq!(*odd.root.value.deref(), false);
        assert_eq!(*even.root.value.deref(), true);
    }

    #[test]
    fn fmap_depth_2() {
        let mut tree = Tree::new(1);
        tree.root.add(2);
        tree.root.add(3);

        let mapped : Vec<i32> = tree.fmap(|x| { x * 2 }).into();

        assert_eq!(mapped.len(), 3);
        assert_eq!(mapped[0], 2);
        assert_eq!(mapped[1], 4);
        assert_eq!(mapped[2], 6);
    }

    #[test]
    fn size_depth_2() {
        let mut tree = Tree::new(1);
        tree.root.add(2);
        tree.root.add(3);

        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn size_depth_3() {
        let mut tree = Tree::new(1);
        tree.root.add(2);
        tree.root.add(3);

        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn into_vec_depth_2() {
        let mut tree = Tree::new(1);
        tree.root.add(2);
        tree.root.add(3);

        let vec: Vec<i32> = tree.into();

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }
}