use num_integer::Integer;
use num_traits::FromPrimitive;

#[derive(Debug)]
struct Tree<T> {
    nodes: Vec<Node<T>>,
    max_depth: usize,
}

#[derive(Debug, Copy, Clone)]
struct Node<T> {
    val: Option<T>,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    level: usize,
    idx: usize,
}

impl<T> Node<T> {
    fn new(
        val: Option<T>,
        left: Option<usize>,
        right: Option<usize>,
        parent: Option<usize>,
        level: usize,
        idx: usize,
    ) -> Self {
        Self {
            val,
            left,
            right,
            parent,
            level,
            idx,
        }
    }
}

impl<T> Tree<T>
where
    T: FromPrimitive + Integer + std::marker::Copy + std::fmt::Debug + std::ops::AddAssign,
{
    fn new() -> Self {
        Self {
            nodes: vec![],
            max_depth: 0,
        }
    }
    fn push_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }
    fn from_str(string: &str) -> Self {
        let mut arena: Tree<T> = Tree::new();
        arena._recurse_tree_building(string, 0, 0, None);
        arena
    }
    fn len(&self) -> usize {
        self.nodes.len()
    }
    fn _recurse_tree_building(
        &mut self,
        string: &str,
        curr_level: usize,
        idx: usize,
        parent: Option<usize>,
    ) -> usize {
        let c = string.chars().next().unwrap();
        // println!("{c}");
        if c != '[' {
            let node = Node::new(
                Some(FromPrimitive::from_u32(c.to_digit(10).unwrap()).unwrap()),
                None,
                None,
                parent,
                curr_level,
                self.nodes.len(),
            );
            if curr_level as usize > self.max_depth {
                self.max_depth = curr_level;
            }
            self.push_node(node);
            return idx;
        }
        let mut level = 0;
        for (i, c) in string.chars().enumerate() {
            if c == '[' {
                level += 1;
            }

            if c == ']' {
                level -= 1;
            }

            if c == ',' && level == 1 {
                let idx = self.len();
                let node: Node<T> = Node::new(None, None, None, parent, curr_level, idx);
                self.push_node(node);

                let left = self._recurse_tree_building(
                    &string[1..i],
                    curr_level + 1,
                    self.len(),
                    Some(idx),
                );

                let right = self._recurse_tree_building(
                    &string[i + 1..(string.len() - 1)],
                    curr_level + 1,
                    self.len(),
                    Some(idx),
                );

                self.nodes[idx].right = Some(right);
                self.nodes[idx].left = Some(left);

                return idx;
            }
        }
        unreachable!();
    }
    fn print_tree(&self, idx: usize) {
        let node = &self.nodes[idx];
        match node.val {
            Some(val) => {
                println!("val: {val:?}\t");
            }
            None => {
                let left = node.left.unwrap();
                let right = node.right.unwrap();
                println!("{left:?}, {right:?}\t");
                self.print_tree(left);
                self.print_tree(right);
            }
        }
    }

    fn explode(&mut self) {
        if self.max_depth < 4 {
            return;
        }
        // for node in self.nodes {}
        self.do_explode(0);
    }

    fn do_explode(&mut self, idx: usize) -> Option<usize> {
        let node = &self.nodes[idx];
        // let node = &self.nodes[idx];
        match (node.level, node.val) {
            (i, Some(_val)) => {
                if i >= 4 {
                    Some(node.idx)
                } else {
                    None
                }
            }
            // (_, Some(_val)) => None,
            (level, None) => {
                let left = node.left.unwrap();
                let right = node.right.unwrap();

                println!("{level:?} {left:?}, {right:?}\t{node:?}");
                // let left_explosion = self.do_explode(left);
                // if let Some(left) = self.do_explode(left) {
                //     let mut sibling: Node<T> = self.find_left_sibling();
                //     if let Some(sibling_val) = sibling.val {
                //         sibling_val += self.nodes[left].val.unwrap();
                //     }
                //
                //     self.explode();
                // }

                match (self.do_explode(left), self.do_explode(right)) {
                    (Some(left_explosion), Some(right_explosion)) => {
                        // if let Some(explosion_idx) = self.do_explode(left) {
                        // if left exploded, go up the tree such that until node is right child
                        println!("LEFT exploded from: {left_explosion:?}");
                        for (i, n) in self.nodes.iter().enumerate() {
                            println!("{i} {:?}", n);
                        }
                        // self.print_tree(0);
                        let mut curr_node = &self.nodes[idx];
                        let mut parent_node;
                        let child_idx;
                        let mut child;
                        // loop {
                        // let parent_idx = curr_node.parent.unwrap();
                        while let Some(parent_idx) = curr_node.parent {
                            parent_node = &self.nodes[parent_idx];
                            if parent_node.right == Some(idx) {
                                child_idx = parent_node.right.unwrap();
                                child = &self.nodes[child_idx];
                                // found needed parent. Goto sibling
                                while let Some(child_idx) = child.right {
                                    // find rightmost element from this tree
                                    child = &self.nodes[child_idx];
                                }
                                if let Some(mut val) = child.val {
                                    val += self.nodes[left_explosion].val.unwrap();
                                }
                                break;
                            }
                            curr_node = parent_node;
                        }
                        println!("RIGHT exploded from: {right_explosion:?}");
                        for (i, n) in self.nodes.iter().enumerate() {
                            println!("{i} {:?}", n);
                        }
                        // self.print_tree(0);
                        let mut curr_node = &self.nodes[idx];
                        let mut parent_node;
                        let child_idx;
                        let mut child;
                        // loop {
                        // let parent_idx = curr_node.parent.unwrap();
                        while let Some(parent_idx) = curr_node.parent {
                            parent_node = &self.nodes[parent_idx];
                            if parent_node.right == Some(idx) {
                                child_idx = parent_node.right.unwrap();
                                child = &self.nodes[child_idx];
                                // found needed parent. Goto sibling
                                while let Some(child_idx) = child.right {
                                    // find rightmost element from this tree
                                    child = &self.nodes[child_idx];
                                }
                                if let Some(mut val) = child.val {
                                    val += self.nodes[right_explosion].val.unwrap();
                                }
                                break;
                            }
                            curr_node = parent_node;
                        }
                        self.nodes.remove(left_explosion);
                        self.nodes.remove(right_explosion);
                        self.nodes[idx].val = FromPrimitive::from_u32(0u32);
                        self.nodes[idx].left = None;
                        self.nodes[idx].right = None;
                        // self.nodes[
                        println!("EXPLODED");
                        for (i, n) in self.nodes.iter().enumerate() {
                            println!("{i} {:?}", n);
                        }
                        // self.print_tree(0);
                    }
                    (None, None) => return None,
                    (a, b) => {
                        println!("left: {:?}", a);
                        println!("right: {:?}", b);
                        unreachable!();
                    }
                }

                // if let Some(explosion_idx) = self.do_explode(left) {
                //     // if left exploded, go up the tree such that until node is right child
                //     println!("LEFT exploded from: {explosion_idx:?}");
                //     for (i, n) in self.nodes.iter().enumerate() {
                //         println!("{i} {:?}", n);
                //     }
                //     // self.print_tree(0);
                //     let mut curr_node = &self.nodes[idx];
                //     let mut parent_node;
                //     let child_idx;
                //     let mut child;
                //     // loop {
                //     // let parent_idx = curr_node.parent.unwrap();
                //     while let Some(parent_idx) = curr_node.parent {
                //         parent_node = &self.nodes[parent_idx];
                //         if parent_node.right == Some(idx) {
                //             child_idx = parent_node.right.unwrap();
                //             child = &self.nodes[child_idx];
                //             // found needed parent. Goto sibling
                //             while let Some(child_idx) = child.right {
                //                 // find rightmost element from this tree
                //                 child = &self.nodes[child_idx];
                //             }
                //             if let Some(mut val) = child.val {
                //                 val += self.nodes[explosion_idx].val.unwrap();
                //             }
                //             break;
                //         }
                //         curr_node = parent_node;
                //     }
                //     self.nodes.remove(explosion_idx);
                //     self.nodes[idx].val = FromPrimitive::from_u32(0u32);
                //     self.nodes[idx].left = None;
                //     self.nodes[idx].right = None;
                //     // self.nodes[
                //     println!("EXPLODED");
                //     // self.print_tree(0);
                // }
                //
                // println!("IN BETWEEN");
                // for (i, n) in self.nodes.iter().enumerate() {
                //     println!("{i} {:?}", n);
                // }
                // if let Some(explosion_idx) = self.do_explode(right) {
                //     // if right exploded, go up the tree such that until node is left child
                //     println!("RIGHT exploded from: {explosion_idx:?}");
                //     for (i, node) in self.nodes.iter().enumerate() {
                //         println!("{i} {:?}", node);
                //     }
                //     let mut curr_node = &self.nodes[idx];
                //     let mut parent_node;
                //     let child_idx;
                //     let mut child;
                //     loop {
                //         let parent_idx = curr_node.parent.unwrap();
                //         parent_node = &self.nodes[parent_idx];
                //         if parent_node.left == Some(idx) {
                //             // found needed parent. Goto sibling
                //             child_idx = parent_node.right.unwrap();
                //             child = &self.nodes[child_idx];
                //             while let Some(child_idx) = child.left {
                //                 // find leftmost element from this tree
                //                 child = &self.nodes[child_idx];
                //             }
                //             match child.val {
                //                 Some(mut val) => val += self.nodes[explosion_idx].val.unwrap(),
                //                 None => unreachable!(),
                //             }
                //             break;
                //             // if let Some(mut val) = child.val {
                //             //     val += self.nodes[explosion_idx].val.unwrap();
                //             // }
                //             //
                //             // self.nodes[child_idx] = *child;
                //         }
                //         curr_node = parent_node;
                //     }
                //     self.nodes[child_idx] = *child;
                //     self.nodes.remove(explosion_idx);
                // }
                // self.nodes[idx].val = FromPrimitive::from_u32(0u32);
                // self.nodes[idx].left = None;
                // self.nodes[idx].right = None;
                self.explode();
                // let right_explosion = self.do_explode(right);
                // } else {
                // let (r1, r2) = self.do_explode(right);
                //     if l1 != l2 {}
                // }
                unreachable!()
            }
        }
    }
}

// pub fn build_tree<T>(string: &str, curr_level: u8, idx: usize) -> Tree<T>
// where
//     T: FromPrimitive + Integer + std::marker::Copy + std::fmt::Debug,
// {
//     let arena = Tree { arena: vec![] };
//     // println!("{string}");
//     let c = string.chars().next().unwrap();
//     // println!("{c}");
//     if c != '[' {
//         return Node::new(
//             Some(FromPrimitive::from_u32(c.to_digit(10).unwrap()).unwrap()),
//             None,
//             None,
//             curr_level,
//         );
//     }
//     let mut level = 0;
//     for (i, c) in string.chars().enumerate() {
//         if c == '[' {
//             level += 1;
//         }
//
//         if c == ']' {
//             level -= 1;
//         }
//
//         if c == ',' && level == 1 {
//             let left = build_tree::<T>(&string[1..i], curr_level + 1);
//             let right = build_tree::<T>(&string[i + 1..(string.len() - 1)], curr_level + 1);
//
//             return Box::new(Node::new(None, Some(left), Some(right), curr_level));
//         }
//     }
//
//     unreachable!()
// }

fn main() {
    let mut tree: Tree<u32> = Tree::from_str("[[[[[9,8],1],2],3],4]");
    // let mut tree: Tree<u32> = Tree::from_str("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    println!("first tree {}", tree.max_depth);
    for (i, node) in tree.nodes.iter().enumerate() {
        println!("{i} {:?}", node);
    }
    // tree.print_tree(0);
    tree.explode();
    // number.print_tree();
    println!("second tree");
    // let number: Node<u32> = build_tree("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]", 0, 0);
    // // number.print_tree();
    //
    // println!("third tree");
    // let number: Node<u32> = build_tree("[[1,9],[8,5]]", 0);
    // // number.print_tree();
}
