use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct BinTreeNode<T> {
    value: T,
    left: Option<Box<BinTreeNode<T>>>,
    right: Option<Box<BinTreeNode<T>>>,
}

impl<T> BinTreeNode<T>
where
    T: Ord,
{
    fn from_vec(vec: Vec<T>) -> Result<Self, &'static str> {
        let mut items = vec.into_iter();

        let first = items.next().ok_or("Empty vector")?;

        let mut output = BinTreeNode {
            value: first,
            left: None,
            right: None,
        };

        for item in items {
            output.insert(item);
        }

        Ok(output)
    }

    fn insert(&mut self, value: T) {
        let mut current = self;

        let insert_node = loop {
            let next_node = match value.cmp(&current.value) {
                Ordering::Less | Ordering::Equal => &mut current.left,
                Ordering::Greater => &mut current.right,
            };

            match next_node {
                Some(ref mut node) => current = node,
                None => break next_node,
            }
        };

        *insert_node = Some(Box::new(BinTreeNode {
            value,
            left: None,
            right: None,
        }));
    }

    fn contains(&self, value: &T) -> bool {
        let mut current = Some(self);

        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Greater => current = node.right.as_deref(),
                Ordering::Less => current = node.left.as_deref(),
                Ordering::Equal => return true,
            }
        }

        false
    }
}

fn main() {
    let my_bin_tree = BinTreeNode::from_vec(vec![3, 5, 6, 1, 7, 8]).unwrap();
    println!("{:?}", my_bin_tree.contains(&3));
}
