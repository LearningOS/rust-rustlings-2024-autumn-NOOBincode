use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root.as_mut() {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(root_node) => {
                match root_node.value.cmp(&value) {
                    Ordering::Less => {
                        if let Some(right) = &mut root_node.right {
                            right.as_mut().insert(value);
                        } else {
                            root_node.right = Some(Box::new(TreeNode::new(value)));
                        }
                    }
                    Ordering::Greater => {
                        if let Some(left) = &mut root_node.left {
                            left.as_mut().insert(value);
                        } else {
                            root_node.left = Some(Box::new(TreeNode::new(value)));
                        }
                    }
                    Ordering::Equal => {}
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            None => false,
            Some(root_node) => {
                let cmp = root_node.value.cmp(&value);
                match cmp {
                    Ordering::Less => {
                        if let Some(right) = &root_node.right {
                            right.as_ref().search(value)
                        } else {
                            false
                        }
                    }
                    Ordering::Greater => {
                        if let Some(left) = &root_node.left {
                            left.as_ref().search(value)
                        } else {
                            false
                        }
                    }
                    Ordering::Equal => true,
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            Ordering::Less => {
                if let Some(right) = &mut self.right {
                    right.as_mut().insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(left) = &mut self.left {
                    left.as_mut().insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {}
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn search(&self, value: T) -> bool {
        if self.value == value {
            return true;
        }
        if value < self.value {
            if let Some(left) = &self.left {
                left.as_ref().search(value)
            } else {
                false
            }
        } else {
            if let Some(right) = &self.right {
                right.as_ref().search(value)
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}