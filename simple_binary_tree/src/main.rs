
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


fn insert(root: &Rc<RefCell<TreeNode>>, val: i32) {

    let mut node = root.borrow_mut(); //to be able to modify the left and right of the root.

    if val < node.val {
        match &node.left{
            Some(left_child) => insert(left_child, val),
            None => {node.left = Some(TreeNode::new(val))}
        }
    } else {
        match &node.right {
            Some(right_child) => insert(right_child, val),
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

fn find_min(node: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let mut current = node.clone();  

    loop {
        let right = current.borrow().right.clone();

        match right {
            Some(right) => current = right,
            None => break
        }
    }

    current
    
}


fn remove(root_node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {

    if let Some(node) = root_node {
        let mut n = node.borrow_mut();

        if val == n.val { //we have found our node.

            //case 1: when there's no leaf node.
            if n.left.is_none() && n.right.is_none() {
                return None;
            } else if n.left.is_none() { //case 2: when there's only 1 child (either side).
                return n.right.take();
            } else if n.right.is_none() {
                return n.left.take();
            } else { //both children are present.
                let min_node_left_subtree = find_min(n.left.as_ref().unwrap());
                n.val = min_node_left_subtree.borrow().val;
                let temp_val = n.val.clone();
                n.left = remove(&mut n.left, temp_val);
            }

        } else if val < n.val {
            //go find it at the left side.
            n.left = remove(&mut n.left, val);
        } else if val > n.val {
            n.right = remove(&mut n.right, val);
        }
        
    } 

    root_node.clone()
    

}


fn main() {
    let root = Some(TreeNode::new(10));
    
    if let Some(ref r) = root {
        insert(r, 5);
        insert(r, 15);
        insert(r, 1);
        insert(r, 7);
        insert(r, 12);
        insert(r, 20);
    }

    inorder_traversal(&root);

    let mut root = remove(&root, 1);

    inorder_traversal(&root);



}
