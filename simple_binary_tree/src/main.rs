
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
     val: i32,
     left: Option<Rc<RefCell<TreeNode>>>,
     right: Option<Rc<RefCell<TreeNode>>>,
}


impl TreeNode {

    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(
            TreeNode {
                val,
                left: None,
                right: None,
            }
        ))
    }
}


fn insert(val: i32, root: &Rc<RefCell<TreeNode>>) {

    let mut node = root.borrow_mut(); //to be able to modify the left and right of the root.

    if val < node.val {
        match &node.left{
            Some(left_child) => insert(val, left_child),
            None => {node.left = Some(TreeNode::new(val))}
        }
    } else {
        match &node.right {
            Some(right_child) => insert(val, right_child),
            None => {node.right = Some(TreeNode::new(val))}
        }
    }
}


fn inorder_traversal(node: &Option<Rc<RefCell<TreeNode>>>) {

    if let Some(node) = node {
        let n = node.borrow();

        inorder_traversal(&n.left);
        println!("{}", n.val);
        inorder_traversal(&n.right);
    }
}


fn main() {
    let root = TreeNode::new(10);
    insert(0, &root);
    insert(10, &root);
    insert(6, &root);
    insert(11, &root);
    insert(15, &root);
    insert(30, &root);
    insert(1, &root);


    inorder_traversal(&Some(root));
}
