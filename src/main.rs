use std::{fmt::Display};

struct TreeNode<T> {
    data: Option<T>,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> 
where
T: Ord
{
    fn new() -> TreeNode<T>
    {
        TreeNode{data: Option::None, left: Option::None, right: Option::None}
    }

    fn insert(&mut self, data: T)
    {
        match &self.data {
            None => {self.data = Option::Some(data);},
            Some(d) => {
                if *d == data {
                    return;
                }
                else if *d < data {
                    match &mut self.right {
                        None => {
                            let mut node = TreeNode::new();
                            node.insert(data);
                            self.right = Option::Some(Box::new(node));
                        }
                        Some(node) => {
                            node.insert(data)
                        }
                    }
                }
                else {
                    match &mut self.left {
                        None => {
                            let mut node = TreeNode::new();
                            node.insert(data);
                            self.left = Option::Some(Box::new(node));
                        }
                        Some(node) => {
                            node.insert(data)
                        }
                    }
                }
            }
        }
    }
}

impl<T> Display for TreeNode<T>
where
    T: Display 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.left {
            node.fmt(f)?;
        }

        match &self.data {
            None => {write!(f, "[]")?;}
            Some(d) => {write!(f, "[{}]", d)?;}
        }

        if let Some(node) = &self.right {
            node.fmt(f)?;
        }

        Result::Ok(())
    }
}


fn main() {
    let mut tree: TreeNode<i32> = TreeNode::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(0);
    tree.insert(-1);

    println!("{}", tree);

}