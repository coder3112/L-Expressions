## L-Expressions I,II

This respository contains the solutions to [ProjectEuler](https://www.projecteuler.net/) problems 909 and 910, which deal with _L-Expressions_.

### L-Expressions

An _L-Expression_ (henceforth abbreviated "lexpr") is one of the following:

- $n \in \mathbb{N}$;
- the symbol $A$;
- the symbol $Z$;
- the symbol $S$;
- a pair of lexprs $u, v$ which is written as $u(v)$.

A lexpr may be transformed via the following rules:

- $A(x) \rightarrow x + 1 \hspace{15pt} \forall x \in \mathbb{N}$;
- $Z(u)(v) \rightarrow v$ for any lexprs $u,v$;
- $S(u)(v)(w) \rightarrow v(u(v)(w))$ for any lexprs $u, v, w$.

Example: The transformation of $S(Z)(A)(0)$ results in the number 1:\
$S(Z)(A)(0) \rightarrow A(Z(A)(0)) \rightarrow A(0) \rightarrow 1$

Note: For this exercise, Project Euler seems to recognize $0 \in \mathbb{N}$.

### Solution Attempt 1

The first solution I attempted made use of simple match expressions wherein I tried to reduce expressions by evaulating them according to the given transformations.
Then, a lexpr could simply be considered either a natural number or a symbol/operator which recursively contained a lexpr inside.

```rs
enum LExpr {
    Nat(Nat),
    A(Box<LExpr>),
    Z(Box<LExpr>, Box<LExpr>),
    S(Box<LExpr>, Box<LExpr>, Box<LExpr>),
}
```

Now, to reduce these expressions, we have:

```rs
impl LExpr {
    fn reduce(&mut self) {
        match self {
            LExpr::Nat(non_zero) => Ok(()),
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
            LExpr::S(lexpr1, lexpr2, lexpr3) => unimplemented!()
    }
}
```

However, this piece of code had several problems, not least of which was that it did not support `LExpr::S()` reductions at all, since I could not figure out how to create child expressions by parsing the structure. Furthermore, there was no way to represent something like `(A)` by itself, since it needed some `LExpr` inside it, which could not be empty. I decided to try and solve the latter problem first.

### Solution Attempt 2

Most of my code stayed the same, with the exception of adding `Option<>` to all `Box<LExpr>` types so that I may implement `(A)` and the like. Then, to reduce this, the function was modified as so:

```rs
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
        LExpr::S(_) => todo!(),
    }
}
```

This actually did solve problems such as `Z(A)(0)` but still there was no way to modify `S(...)` since it involved having many child expressions move, and I needed to allow for it to have as many child nodes as deep as it wanted, which made hardcording anything unfeasible.

### Solution Attempt 3

At this point, the constant imagery of child nodes and recursion reminded me of two things: (a) S-Expressions and (b) Trees. Indeed, an S-Expression can be roughly thought of as a binary tree with some exceptions, and as lisp has famously taught us, one can parse them. So, I decided I would try and use trees to solve this problem. This worked much better in that things actually worked. However, my first implementation was a giant mess where I could not access parent info, everythign repeated itself etc. Furthermore, there was the issue of me not being able to work with more complicated structures without a single giant connecting node without ugly hacks. There was no easy way to decide which nodes to reduce, and there were too many bugs. So, I decided to re-implement the solution with structs and maybe think about the problem more before tackling it in a regex or mathematical sense or a FSA as well.
