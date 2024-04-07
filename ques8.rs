pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  if let Some(node) = root {
      let left_depth = Self::max_depth(node.borrow().left.clone());
      let right_depth = Self::max_depth(node.borrow().right.clone());
      return 1 + left_depth.max(right_depth);
  }
  0
}