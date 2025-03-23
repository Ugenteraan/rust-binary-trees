#[derive(Debug)]
struct KDTreeNode {
    value: (i32, i32),
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>,
}

impl KDTreeNode {
    fn new(value: (i32, i32)) -> Self {
        KDTreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: (i32, i32), even_depth: bool) {
        //even depth is used to check if the current depth is at an even level or odd level.
        //it starts with false since the first level (1) is an odd number.
        if (!even_depth && value.0 < self.value.0) || (even_depth && value.1 < self.value.1) {
            if let Some(ref mut left) = self.left {
                left.insert(value, !even_depth); //flip the even_depth.
            } else {
                self.left = Some(Box::new(KDTreeNode::new(value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(value, !even_depth);
            } else {
                self.right = Some(Box::new(KDTreeNode::new(value)));
            }
        }
    }

    fn print_tree(&self, prefix: String, is_left: bool) {
        println!(
            "{}{} ({}, {})",
            prefix,
            if is_left { "├──" } else { "└──" },
            self.value.0,
            self.value.1
        );

        if let Some(ref left) = self.left {
            left.print_tree(format!("{}{}", prefix, if is_left { "│   " } else { "    " }), true);
        }
        if let Some(ref right) = self.right {
            right.print_tree(format!("{}{}", prefix, if is_left { "│   " } else { "    " }), false);
        }
    }


}

fn main() {
    let mut root = KDTreeNode::new((9, 1));

    let points = [
        (3, 6),
        (2, 7),
        (6, 12),
        (17, 15),
        (13, 15),
        (10, 19),
    ];

    for point in points {
        root.insert(point, false);
    }

    println!("KD-Tree:");

    root.print_tree(String::new(), false);
}
