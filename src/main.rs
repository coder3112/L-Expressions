use std::hint::unreachable_unchecked;

const A: char = 'A';
const Z: char = 'Z';
const S: char = 'S';

fn main() {
    // TODO: MILESTONE 1 - Actually have this return 1.
    // let tree = Node::new_symbol(
    //     S,
    //     vec![
    //         Node::new_symbol(Z, vec![]),
    //         Node::new_symbol(A, vec![]),
    //         Node::new_nat(0),
    //     ],
    // );
    // println!("{:#?}", tree)
    let subtree1 = Node::new_symbol(A, vec![Node::new_nat(4)]);
    let mut tree = Node::new_symbol(A, vec![subtree1]);
    tree.reduce_node();
    tree.reduce_node();
    println!("{:#?}", tree)
}

impl Node {
    fn reduce_node(&mut self) {
        match self {
            Node::Nat(_) => (),
            Node::NodeA { children } => match &children[0] {
                Some(child) => match &**child {
                    Node::Nat(val) => *self = Node::new_nat(val + 1),
                    inner_node => {
                        let mut inner_node = inner_node.clone();
                        inner_node.reduce_node();
                        *self = Node::new_symbol(A, vec![inner_node]);
                    }
                },
                None => {} // Actually can't do anything. Already this node is as reduced as possible for now.
            },
            Node::NodeZ { children } => todo!(),
            Node::NodeS { children } => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    A = 1,
    Z = 2,
    S = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Nat(u64),
    NodeA {
        children: [Option<Box<Node>>; Symbol::A as usize],
    },
    NodeZ {
        children: [Option<Box<Node>>; Symbol::Z as usize],
    },
    NodeS {
        children: [Option<Box<Node>>; Symbol::S as usize],
    },
}

impl Node {
    pub fn new_symbol(val: char, children: Vec<Node>) -> Self {
        match val {
            'A' => match children.len() {
                0 => return Node::NodeA { children: [None] },
                1 => {
                    return Node::NodeA {
                        children: [Some(Box::new(children[0].clone()))],
                    };
                }
                _ => panic!("Asked to construct A with more than one child"),
            },
            'Z' => match children.len() {
                0 => {
                    return Node::NodeZ {
                        children: [None, None],
                    };
                }
                1 => {
                    return Node::NodeZ {
                        children: [Some(Box::new(children[0].clone())), None],
                    };
                }
                2 => {
                    return Node::NodeZ {
                        children: [
                            Some(Box::new(children[0].clone())),
                            Some(Box::new(children[1].clone())),
                        ],
                    };
                }
                _ => panic!("Asked to construct Z with more than two children"),
            },
            'S' => match children.len() {
                0 => {
                    return Node::NodeS {
                        children: [None, None, None],
                    };
                }
                1 => {
                    return Node::NodeS {
                        children: [Some(Box::new(children[0].clone())), None, None],
                    };
                }
                2 => {
                    return Node::NodeS {
                        children: [
                            Some(Box::new(children[0].clone())),
                            Some(Box::new(children[1].clone())),
                            None,
                        ],
                    };
                }
                3 => {
                    return Node::NodeS {
                        children: [
                            Some(Box::new(children[0].clone())),
                            Some(Box::new(children[1].clone())),
                            Some(Box::new(children[2].clone())),
                        ],
                    };
                }
                _ => panic!("Asked to construct S with more than three children"),
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }
    pub fn new_nat(val: u64) -> Self {
        Node::Nat(val)
    }
}
