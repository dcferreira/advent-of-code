use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct BTNode {
    zero: ChildNode,
    one: ChildNode,
    zero_count: u32,
    one_count: u32,
}

type ChildNode = Option<Box<BTNode>>;

impl BTNode {
    pub fn new() -> Self {
        BTNode {
            zero: None,
            one: None,
            zero_count: 0,
            one_count: 1,
        }
    }
}

struct Tree {
    head: BTNode,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            head: BTNode::new(),
        }
    }

    fn update_with_line(node: &Box<BTNode>, s: &str) {
        if s.len() == 0 {
            return;
        }

        let c = s.chars().next().unwrap();
        match c {
            '0' => {
                node.zero_count += 1;
                if node.zero.is_none() {
                    node.zero = Some(Box::new(BTNode::new()));
                }
                Tree::update_with_line(&node.zero.unwrap(), &s[1..]);
            }
            '1' => {
                node.one_count += 1;
                if node.one.is_none() {
                    node.one = Some(Box::new(BTNode::new()));
                }
                Tree::update_with_line(&node.one.unwrap(), &s[1..]);
            }
            _ => {}
        }
    }

    fn get_oxygen(node: &Box<BTNode>) -> i32 {
        let mut bits: Vec<u32> = vec![];
        while !(node.zero.is_none() | node.one.is_none()) {
            if node.zero_count > node.one_count {
                bits.push(0);
                node = &node.zero.unwrap();
            } else {
                bits.push(1);
                node = &node.one.unwrap();
            }
        }
        return bin_array_to_number(bits);
    }

    fn get_co2(node: &BTNode) -> i32 {
        let mut bits: Vec<u32> = vec![];
        while !(node.zero.is_none() | node.one.is_none()) {
            if node.zero_count <= node.one_count {
                bits.push(0);
                node = &node.zero.unwrap();
            } else {
                bits.push(1);
                node = &node.one.unwrap();
            }
        }
        return bin_array_to_number(bits);
    }
}

// impl BTNode {
//     fn update_with_line(&self, s: &str) {
//         if s.len() == 0 {
//             return;
//         }

//         let c = s.chars().next().unwrap();
//         match c {
//             '0' => {
//                 self.zero_count += 1;
//                 if self.zero.is_none() {
//                     self.zero = Some(Box::new(BTNode::new()));
//                 }
//                 self.zero.unwrap().update_with_line(&s[1..]);
//             }
//             '1' => {
//                 self.one_count += 1;
//                 if self.one.is_none() {
//                     self.one = Some(Box::new(BTNode::new()));
//                 }
//                 self.one.unwrap().update_with_line(&s[1..]);
//             }
//             _ => {}
//         }
//     }
// }

fn bin_array_to_number(arr: Vec<u32>) -> i32 {
    let base: i32 = 2;
    let mut out = 0;

    for n in arr.iter().rev() {
        out += base.pow(*n);
    }
    return out;
}

// fn descend_tree(child_node: &ChildNode, s: &str) {
//     if s.len() == 0 {
//         return;
//     }

//     let mut node = child_node.as_ref().unwrap();
//     let c = s.chars().next().unwrap();
//     match c {
//         '0' => {
//             node.zero_count += 1;
//             if node.zero.is_none() {
//                 node.zero = Some(Box::new(BTNode::new()));
//             }
//             descend_tree(&node.zero, &s[1..])
//         }
//         '1' => {
//             node.one_count += 1;
//             if node.one.is_none() {
//                 node.one = Some(Box::new(BTNode::new()));
//             }
//             descend_tree(&node.one, &s[1..])
//         }
//         _ => {}
//     }
// }

// fn populate_tree(tree: &Tree, reader: BufReader<File>) -> Result<(), Error> {
//     for line in reader.lines() {
//         let s = line?;

//         let mut node: ChildNode = tree.head;
//         for bit in s.chars() {
//             if bit == '0' {
//                 node.zero_count += 1;
//                 if node.zero.is_none() {
//                     node.zero = Some(Box::new(BTNode::new()));
//                 }
//                 node = node.zero.unwrap();
//             } else {
//                 node.one_count += 1;
//                 if node.one.is_none() {
//                     node.one = Some(Box::new(BTNode::new()));
//                 }
//                 node = node.one.unwrap();
//             }
//         }
//     }
//     Ok(())
// }

fn main() -> Result<(), Error> {
    let path = "./data/input.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let tree = Tree::new();
    for line in reader.lines() {
        let s = line?;
        Tree::update_with_line(&Box::new(tree.head), &s);
    }

    // let oxygen = Tree::get_oxygen(&Box::new(tree.head));
    // let co2 = Tree::get_co2(&Box::new(tree.head));

    // println!("oxy: {}, co2: {}", oxygen, co2);
    // println!("result: {}", oxygen * co2);

    Ok(())
}
