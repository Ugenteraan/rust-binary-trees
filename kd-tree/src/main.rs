use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct KDTreeNode {
    value: (i32, i32),
    left: Option<Rc<RefCell<KDTreeNode>>>,
    right: Option<Rc<RefCell<KDTreeNode>>>,
}

impl KDTreeNode {
    fn new(root_value: (i32, i32)) -> Rc<RefCell<KDTreeNode>> {
        Rc::new(RefCell::new(KDTreeNode {
            value: root_value,
            left: None,
            right: None,
        }))
    }
}

fn insert(root: &Rc<RefCell<KDTreeNode>>, value: (i32, i32), check_left: bool) {
    let mut node = root.borrow_mut();

    if (check_left && value.0 < node.value.0) || (!check_left && value.1 < node.value.1) {
        match &node.left {
            Some(left_child) => insert(left_child, value, !check_left),
            None => node.left = Some(KDTreeNode::new(value)),
        }
    } else {
        match &node.right {
            Some(right_child) => insert(right_child, value, !check_left),
            None => node.right = Some(KDTreeNode::new(value)),
        }
    }
}

fn search(node: &Rc<RefCell<KDTreeNode>>, value: (i32, i32), check_left: bool) -> bool {
    let n = node.borrow();

    if value == n.value {
        return true;
    }

    if (check_left && value.0 < n.value.0) || (!check_left && value.1 < n.value.1) {
        match &n.left {
            Some(left_child) => search(&left_child, value, !check_left),
            None => false,
        }
    } else {
        match &n.right {
            Some(right_child) => search(&right_child, value, !check_left),
            None => false,
        }
    }
}

fn inorder_traversal(node: &Option<Rc<RefCell<KDTreeNode>>>) {
    if let Some(node) = node {
        let n = node.borrow();

        inorder_traversal(&n.left);
        println!("{:?}", n.value);
        inorder_traversal(&n.right);
    }
}

fn main() {
    let root = KDTreeNode::new((8, 9));

    insert(&root, (3, 2), true);
    insert(&root, (10, 12), true);
    insert(&root, (15, 48), true);
    insert(&root, (1, -56), true);
    insert(&root, (-36, -58), true);
    insert(&root, (-56, 11), true);
    insert(&root, (68, -89), true);

    //inorder_traversal(&Some(root));

    let res: bool = search(&root, (-37, -58), true);
    println!("{:?}", res);
}
