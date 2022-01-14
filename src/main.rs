use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct BinTreeNode<T>
where
    T: Ord + Clone,
{
    value: T,
    left: Option<Box<BinTreeNode<T>>>,
    right: Option<Box<BinTreeNode<T>>>,
}

impl<T> BinTreeNode<T>
where
    T: Ord + Clone,
{
    fn from_vec(vec: Vec<T>) -> Option<Self> {
        if vec.is_empty() {
            return None;
        }

        let mut output = BinTreeNode {
            value: vec[0].clone(),
            left: None,
            right: None,
        };

        if vec.len() == 1 {
            return Some(output);
        }

        for i in vec.iter().skip(1) {
            output.insert(i.clone());
        }

        Some(output)
    }

    fn insert(&mut self, value: T) {
        let mut current_node = Box::new(self.clone());
        loop {
            match value.cmp(&current_node.value) {
                Ordering::Greater => {
                    if current_node.clone().right.is_some() {
                        current_node.as_mut().right.as_mut().unwrap().insert(value);
                        break;
                    }

                    current_node.right = Some(Box::new(BinTreeNode {
                        value,
                        left: None,
                        right: None,
                    }));
                    break;
                }

                Ordering::Less => {
                    if current_node.clone().left.is_some() {
                        current_node.as_mut().left.as_mut().unwrap().insert(value);
                        break;
                    }

                    current_node.left = Some(Box::new(BinTreeNode {
                        value,
                        left: None,
                        right: None,
                    }));
                    break;
                }

                Ordering::Equal => continue,
            }
        }
        *self = *current_node;
    }

    fn contains(&self, value: T) -> bool {
        let mut current_node = Some(Box::new(self.clone()));
        loop {
            if current_node.is_none() {
                break;
            }
            let unwrapped_node = current_node.clone().unwrap();
            match value.cmp(&unwrapped_node.value) {
                Ordering::Greater => current_node = unwrapped_node.right,
                Ordering::Less => current_node = unwrapped_node.left,
                Ordering::Equal => return true,
            }
        }

        false
    }
}

fn main() {
    let my_bin_tree = BinTreeNode::from_vec(vec![3, 5, 6, 1, 7, 8]).expect("Empty vector");
    println!("{:?}", my_bin_tree.contains(3));
}
