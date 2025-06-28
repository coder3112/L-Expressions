fn main() {
    let tree = NodeStruct {
        value: Node::Symbol(Symbol::S),
        children: Some(vec![
            NodeStruct {
                value: Node::Symbol(Symbol::Z),
                children: None,
            },
            NodeStruct {
                value: Node::Symbol(Symbol::A),
                children: None,
            },
            NodeStruct {
                value: Node::Nat { value: 0 },
                children: None,
            },
        ]),
    };

    println!("{:#?}", tree);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    A,
    Z,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Nat { value: u64 },
    Symbol(Symbol),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeStruct {
    value: Node,
    children: Option<Vec<NodeStruct>>,
}
