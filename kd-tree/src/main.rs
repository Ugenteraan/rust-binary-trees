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
            left.print_tree(
                format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                true,
            );
        }
        if let Some(ref right) = self.right {
            right.print_tree(
                format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                false,
            );
        }
    }

    fn find_points_in_range(
        &self,
        query: (i32, i32),
        radius_sq: i32,
        even_depth: bool,
        results: &mut Vec<(i32, i32)>,
    ) {
        let dx = self.value.0 - query.0;
        let dy = self.value.1 - query.1;

        let dist_sq: i32 = (dx * dx) + (dy * dy);

        if dist_sq <= radius_sq {
            results.push(self.value);
        }

        let difference = if !even_depth { dx } else { dy };

        //if the difference is more than 0, that means the query's val (in that dimension) is
        //smaller than the node's val (in the same dimension). Therefore, we go left.
        if difference >= 0 {
            if let Some(left_node) = &self.left {
                left_node.find_points_in_range(query, radius_sq, !even_depth, results);
            }
            //after exhausting the last node, when we traverse back, we check the right node to see
            //if the value intersect with our radius.
            if difference * difference <= radius_sq {
                if let Some(right_node) = &self.right {
                    right_node.find_points_in_range(query, radius_sq, !even_depth, results);
                }
            }
        } else {
            //else, we go right.
            if let Some(right_node) = &self.right {
                right_node.find_points_in_range(query, radius_sq, !even_depth, results);
            }

            //same thing as before but we go to the left instead.
            if difference * difference <= radius_sq {
                if let Some(left_node) = &self.left {
                    left_node.find_points_in_range(query, radius_sq, even_depth, results);
                }
            }
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
        (17, 0),
        (1, 4),
        (5, 2),
        (2, 3),
    ];

    for point in points {
        root.insert(point, false);
    }

    println!("KD-Tree:");

    root.print_tree(String::new(), false);

    let query = (14, 14);
    let mut results = Vec::new();
    let radius = 9;
    root.find_points_in_range(query, radius * radius, false, &mut results);

    println!("{:?}", results);
}
