#[derive(Clone, Debug, PartialEq)]
enum Parser {
    Empty,
    Epsilon,
    Literal(char),
    Concat(Box<Parser>, Box<Parser>),
    Union(Box<Parser>, Box<Parser>),
}

use self::Parser::*;

fn and(l: Parser, r: Parser) -> Parser {
    Concat(Box::new(l), Box::new(r))
}

fn or(l: Parser, r: Parser) -> Parser {
    Union(Box::new(l), Box::new(r))
}

fn is_nullable(p: &Parser) -> bool {
    match *p {
        Empty => false,
        Epsilon => true,
        Literal(_) => false,
        Concat(ref l, ref r) => is_nullable(l) && is_nullable(r),
        Union(ref l, ref r) => is_nullable(l) || is_nullable(r),
    }
}

fn derive(p: &Parser, c: char) -> Parser {
    match *p {
        Epsilon => Empty,
        Literal(lc) => if c == lc {
            Epsilon
        } else {
            Empty
        }
        Concat(ref l, ref r) => if is_nullable(l) {
            or(and(derive(l, c), (**r).clone()), derive(r, c))
        } else {
            and(derive(l, c), (**r).clone())
        }
        Union(ref l, ref r) => if *l == Box::new(Empty) && *r == Box::new(Empty) {
            Empty
        } else if *l == Box::new(Empty) {
            derive(r, c)
        } else if *r == Box::new(Empty) {
            derive(l, c)
        } else {
            or(derive(l, c), derive(r, c))
        }
        Empty => panic!("Cannot derive the empty parser"),
    }
}

fn parse(p: Parser, chars: &[u8]) -> bool {
    match *chars {
        [] => is_nullable(&p),
        [ref h, ref t..] => parse(derive(&p, *h as char), t)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser::*;
    use super::{parse,derive,and,or};

    #[test]
    fn eq() {
        let p = and(Literal('a'), or(Literal('b'), Literal('c')));
        assert!(parse(p.clone(), "ab".as_bytes()));
        assert!(parse(p, "ac".as_bytes()));
    }
}
