use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod p1;
pub mod p2;

// Interval via Augmented Tree
#[derive(Debug, Default)]
struct AugmentedTreeNode {
    start: u64,
    end: u64,
    alpha: u64,
    _parent: Option<Weak<RefCell<Self>>>,
    left: Option<Rc<RefCell<Self>>>,
    right: Option<Rc<RefCell<Self>>>,
}
impl AugmentedTreeNode {}

fn add_child(node: &Rc<RefCell<AugmentedTreeNode>>, start: u64, end: u64) {
    let e = add_child_recurse(node, start, end);
    let m = node.borrow().alpha.max(e);
    node.borrow_mut().alpha = m;
}

fn add_child_recurse(node: &Rc<RefCell<AugmentedTreeNode>>, start: u64, end: u64) -> u64 {
    if start <= node.borrow().start {
        if node.borrow().left.is_none() {
            let new = AugmentedTreeNode {
                start,
                end,
                alpha: end,
                _parent: Some(Rc::downgrade(node)),
                left: None,
                right: None,
            };
            node.borrow_mut().left = Some(Rc::new(RefCell::new(new)));
            let m = node.borrow().alpha.max(end);
            node.borrow_mut().alpha = m;
            return m;
        } else {
            let e = add_child_recurse(&Rc::clone(node.borrow().left.as_ref().unwrap()), start, end);
            let m = node.borrow().alpha.max(e);
            node.borrow_mut().alpha = m;
            return m;
        }
    } else {
        if node.borrow().right.is_none() {
            let new = AugmentedTreeNode {
                start,
                end,
                alpha: end,
                _parent: Some(Rc::downgrade(node)),
                left: None,
                right: None,
            };
            node.borrow_mut().right = Some(Rc::new(RefCell::new(new)));
            let m = node.borrow().alpha.max(end);
            node.borrow_mut().alpha = m;
            return m;
        } else {
            let e = add_child_recurse(
                &Rc::clone(node.borrow().right.as_ref().unwrap()),
                start,
                end,
            );
            let m = node.borrow().alpha.max(e);
            node.borrow_mut().alpha = m;
            return m;
        }
    }
}

fn intersects_node(node: &Rc<RefCell<AugmentedTreeNode>>, query: u64) -> bool {
    let (start, end) = (node.borrow().start, node.borrow().end);
    if start <= query && query <= end {
        return true;
    }

    if query < start {
        if node.borrow().left.is_none() {
            return false;
        } else {
            return intersects_node(&Rc::clone(node.borrow().left.as_ref().unwrap()), query);
        }
    } else {
        // end < query
        if node.borrow().alpha < query {
            return false;
        } else {
            let check = if node.borrow().right.is_some() {
                intersects_node(&Rc::clone(node.borrow().right.as_ref().unwrap()), query)
            } else {
                false
            };

            if check {
                return true;
            }

            return if node.borrow().left.is_some() {
                intersects_node(&Rc::clone(node.borrow().left.as_ref().unwrap()), query)
            } else {
                false
            };
        }
    }
}

