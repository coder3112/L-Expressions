fn main() {
    let mut expr1 = LExpr::A(Some(Box::new(LExpr::Z(Some((
        Box::new(LExpr::A(None)),
        Box::new(LExpr::Nat(0)),
    ))))));
    let mut expr2 = LExpr::S(Some((
        Box::new(LExpr::Z(None)),
        Box::new(LExpr::A(None)),
        Box::new(LExpr::Nat(0)),
    )));
    // let expr1 = LExpr::A(Some(Box::new(LExpr::Nat(0))));
    // let expr2 = LExpr::Z(Some((Box::new(expr1.clone()), Box::new(LExpr::Nat(2)))));
    // let expr = LExpr::Z(Some((Box::new(LExpr::A(None)), Box::new(LExpr::Nat(0)))));
    // let expr = LExpr::A(Some(Box::new(expr)));
    // let mut expr = LExpr::Z(Some((Box::new(expr1), Box::new(expr2))));
    println!("{:?}", expr2);
}

#[derive(Debug, Clone)]
enum LExpr {
    Nat(u64),
    A(Option<Box<LExpr>>),
    Z(Option<(Box<LExpr>, Box<LExpr>)>),
    S(Option<(Box<LExpr>, Box<LExpr>, Box<LExpr>)>),
}

impl LExpr {
    pub fn reduce(&mut self) {
        match self {
            LExpr::Nat(_) => (),
            LExpr::A(expr) => match expr {
                None => (),
                Some(expr) => match &**expr {
                    LExpr::Nat(n) => *self = LExpr::Nat(n + 1),
                    expr => {
                        let mut expr = expr.clone();
                        expr.reduce();
                        *self = LExpr::A(Some(Box::new(expr)));
                    }
                },
            },
            LExpr::Z(expr) => match expr {
                Some((_u, v)) => *self = *v.clone(),
                None => (),
            },
            LExpr::S(expr) => match expr {
                Some((u, v, w)) => {
                    println!("u: {:?} v: {:?} w: {:?}", u, v, w);
                }
                None => (),
            },
        }
    }
}
