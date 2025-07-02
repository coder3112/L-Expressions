use std::hint::unreachable_unchecked;

const A: char = 'A';
const Z: char = 'Z';
const S: char = 'S';

fn main() {
    let mut tree = Node::new_symbol(
        S,
        vec![
            Node::new_symbol(S, vec![]),
            Node::new_symbol(S, vec![Node::new_symbol(S, vec![])]),
            Node::new_symbol(S, vec![Node::new_symbol(Z, vec![])]),
        ],
    );
    tree.reduce_node();
    println!("{:#?}", tree);
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
            Node::NodeZ { children } => match children.iter().filter(|x| x.is_some()).count() {
                0 => {}
                1 => {} // I don't know if this should be fixed here at all anyways. Maybe some other outer expression automatically takes cares of it.
                2 => {
                    let [_u, v] = children;
                    *self = *v.clone().unwrap() //This should be fine since we only do this op if both are Some() due to the `count . filter` above.
                }
                _ => unsafe { unreachable_unchecked() },
            },
            Node::NodeS { children } => match children.iter().filter(|x| x.is_some()).count() {
                0 => {}
                1 => {}
                2 => {}
                3 => {
                    let v_child = children
                        .clone()
                        .iter()
                        .flatten()
                        .map(|x| *x.clone())
                        .collect::<Vec<Node>>();
                    let u = &v_child[0];
                    let v = &v_child[1];
                    let w = &v_child[2];
                    // println!("u {:?} v{:?} w{:?}", u, v, w);
                    // let mut v_child = &children
                    //     .iter()
                    //     .flatten()
                    //     .map(|x| *x.clone())
                    //     .collect::<Vec<Node>>();
                    let vw = [v, w];
                    let (mut i, mut nodes) = match v {
                        Node::Nat(_) => (0, vec![]),
                        Node::NodeA { children } => (
                            children.iter().filter(|x| x.is_some()).count(),
                            children
                                .iter()
                                .flatten()
                                .map(|x| *x.clone())
                                .collect::<Vec<Node>>(),
                        ),
                        Node::NodeZ { children } => (
                            children.iter().filter(|x| x.is_some()).count(),
                            children
                                .iter()
                                .flatten()
                                .map(|x| *x.clone())
                                .collect::<Vec<Node>>(),
                        ),
                        Node::NodeS { children } => (
                            children.iter().filter(|x| x.is_some()).count(),
                            children
                                .iter()
                                .flatten()
                                .map(|x| *x.clone())
                                .collect::<Vec<Node>>(),
                        ),
                    };
                    for node in vw {
                        if i < u.capacity() {
                            nodes.push(node.clone());
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    let s_child: Vec<Node>;
                    let v_child = Node::new_symbol(u.to_char(), nodes);
                    if i < v.capacity() && i < 2 {
                        s_child = vec![v_child, w.clone()];
                    } else {
                        s_child = vec![v_child];
                    }
                    *self = Node::new_symbol(v.to_char(), s_child);
                }
                _ => unsafe { unreachable_unchecked() },
            },
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
    fn to_char(&self) -> char {
        match self {
            Node::Nat(_) => panic!("Numbers not supported like that"),
            Node::NodeA { children: _ } => A,
            Node::NodeZ { children: _ } => Z,
            Node::NodeS { children: _ } => S,
        }
    }
    fn to_int(&self) -> usize {
        match self {
            Node::Nat(_) => panic!("Numbers not supported like that"),
            Node::NodeA { children: _ } => Symbol::A as usize,
            Node::NodeZ { children: _ } => Symbol::Z as usize,
            Node::NodeS { children: _ } => Symbol::S as usize,
        }
    }
    fn capacity(&self) -> usize {
        match self {
            Node::Nat(_) => 0,
            _ => self.to_int(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sza0() {
        let mut tree = Node::new_symbol(
            S,
            vec![
                Node::new_symbol(Z, vec![]),
                Node::new_symbol(A, vec![]),
                Node::new_nat(0),
            ],
        );
        tree.reduce_node();
        tree.reduce_node();
        tree.reduce_node();
        println!("{:#?}", tree)
    }
}
