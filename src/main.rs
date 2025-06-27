use std::num::NonZeroU64;

fn main() {
    let lexpr1 = LExpr::A(Box::new(LExpr::Nat(3)));
    let lexpr2 = LExpr::Nat(0);
    let mut lexpr = LExpr::Z(Box::new(lexpr2), Box::new(lexpr1));
    loop {
        let lexpr_old = lexpr.clone();
        lexpr.reduce().unwrap();
        if lexpr_old == lexpr {
            break;
        }
    }
    println!("{:?}", lexpr);
}

type Nat = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LExpr {
    Nat(Nat),
    A(Box<LExpr>),
    Z(Box<LExpr>, Box<LExpr>),
    S(Box<LExpr>, Box<LExpr>, Box<LExpr>),
}

impl LExpr {
    fn reduce(&mut self) -> Result<(), ReduceError> {
        match self {
            LExpr::Nat(non_zero) => Ok(()),
            //TODO: This cannot yet do stuff like A(Z(A)(0)) since A does not support it directly.
            LExpr::A(expr) => match &**expr {
                LExpr::Nat(nat) => {
                    *self = LExpr::Nat(*nat + 1);
                    Ok(())
                }
                _ => self.reduce(),
            },
            LExpr::Z(_lexpr1, lexpr2) => {
                *self = *lexpr2.clone();
                Ok(())
            }
            LExpr::S(lexpr, lexpr1, lexpr2) => todo!(),
        }
    }
}

#[derive(Debug)]
enum ReduceError {
    ANotNat,
}
