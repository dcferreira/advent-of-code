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
            one_count: 0,
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

    fn update_with_line(node: &mut Box<BTNode>, s: &str) {
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
                Tree::update_with_line(node.zero.as_mut().unwrap(), &s[1..]);
            }
            '1' => {
                node.one_count += 1;
                if node.one.is_none() {
                    node.one = Some(Box::new(BTNode::new()));
                }
                Tree::update_with_line(node.one.as_mut().unwrap(), &s[1..]);
            }
            _ => {}
        }
    }

    fn get_oxygen(root: &Box<BTNode>) -> i32 {
        let mut node = root;
        let mut bits: Vec<u32> = vec![];
        while node.zero_count + node.one_count > 0 {
            if node.zero_count > node.one_count {
                println!("0: {},{}", node.zero_count, node.one_count);
                bits.push(0);
                node = node.zero.as_ref().unwrap();
            } else {
                println!("1: {},{}", node.zero_count, node.one_count);
                bits.push(1);
                node = node.one.as_ref().unwrap();
            }
        }
        println!("oxy: {:?}", bits);
        return bin_array_to_number(bits);
    }

    fn get_co2(root: &Box<BTNode>) -> i32 {
        let mut node = root;
        let mut bits: Vec<u32> = vec![];
        while node.zero_count + node.one_count > 0 {
            if node.one_count > node.zero_count && node.zero_count > 0 {
                println!("0: {},{}", node.zero_count, node.one_count);
                bits.push(0);
                node = node.zero.as_ref().unwrap();
            } else {
                println!("1: {},{}", node.zero_count, node.one_count);
                bits.push(1);
                node = node.one.as_ref().unwrap();
            }
        }
        println!("co2: {:?}", bits);
        return bin_array_to_number(bits);
    }
}

fn bin_array_to_number(arr: Vec<u32>) -> i32 {
    let base: i32 = 2;
    let mut out = 0;

    for n in arr.iter().rev() {
        out += base.pow(*n);
    }
    return out;
}

fn main() -> Result<(), Error> {
    let path = "./data/input_short.txt";
    let input = File::open(path)?;
    let reader = BufReader::new(input);

    let tree = Tree::new();
    let mut root = Box::new(tree.head);
    for line in reader.lines() {
        let s = line?;
        Tree::update_with_line(&mut root, &s);
    }

    let oxygen = Tree::get_oxygen(&root);
    let co2 = Tree::get_co2(&root);

    println!("oxy: {}, co2: {}", oxygen, co2);
    println!("result: {}", oxygen * co2);

    Ok(())
}
