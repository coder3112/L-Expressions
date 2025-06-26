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
