use std::collections::HashMap;

#[derive(PartialEq)]
struct Empty;

#[derive(PartialEq)]
struct Epsilon;

#[derive(PartialEq)]
struct Literal {
    c: char,
}

struct Concat<'a> {
    left: &'a mut Parser,
    right: &'a mut Parser,
}
impl<'a> PartialEq for Concat<'a> {
    fn eq(&self, other: &Concat<'a>) -> bool {
        self == other
    }
}

struct Union<'a> {
    left: &'a mut Parser,
    right: &'a mut Parser,
    _is_nullable: Option<bool>,
}
impl<'a> PartialEq for Union<'a> {
    fn eq(&self, other: &Union<'a>) -> bool {
        self == other
    }
}


struct MemoizedDerivation<'a> {
    derivations: &'a HashMap<char, &'a mut Parser>,
}
impl<'a> PartialEq for MemoizedDerivation<'a> {
    fn eq(&self, other: &MemoizedDerivation<'a>) -> bool {
        self == other
    }
}

trait Parser {
    fn is_nullable(&mut self) -> bool;
    fn derive(&mut self, c: char) -> Box<Parser>;
    fn concat<'a>(&'a mut self, that: &'a mut Parser) -> Box<Parser + 'a>
    where
        Self: Sized,
    {
        Box::new(Concat {
            left: self,
            right: that,
        })
    }
}

trait Memoized: Parser {
    fn inner_derive(&mut self, c: char) -> Box<Parser>;
}

impl Parser for Epsilon {
    fn is_nullable(&mut self) -> bool {
        true
    }

    fn derive(&mut self, _: char) -> Box<Parser> {
        Box::new(Empty {})
    }
}

impl Parser for Empty {
    fn is_nullable(&mut self) -> bool {
        false
    }

    fn derive(&mut self, _: char) -> Box<Parser> {
        panic!("Cannot derive the empty parser")
    }
}

impl Parser for Literal {
    fn is_nullable(&mut self) -> bool {
        false
    }

    fn derive(&mut self, c: char) -> Box<Parser> {
        if self.c == c {
            Box::new(Epsilon {})
        } else {
            Box::new(Empty {})
        }
    }
}

impl<'a> Parser for Concat<'a> {
    fn is_nullable(&mut self) -> bool {
        self.left.is_nullable() && self.right.is_nullable()
    }

    fn derive(&mut self, c: char) -> Box<Parser> {
        if self.left.is_nullable() {
            self.left.derive(c)
        } else {
            self.left.derive(c)
        }
    }
}

impl<'a> Parser for Union<'a> {
    fn is_nullable(&mut self) -> bool {
        self._is_nullable.unwrap_or_else(|| {
            self._is_nullable = Some(false);
            let mut back = self.left.is_nullable() || self.right.is_nullable();
            self._is_nullable = Some(back);

            while (self.left.is_nullable() || self.right.is_nullable()) != back {
                back = self.left.is_nullable() || self.right.is_nullable();
                self._is_nullable = Some(back);
            }
            back
        })
    }

    fn derive(&mut self, c: char) -> Box<Parser> {
        self.inner_derive(c);
        self.left.derive(c)
    }
}

impl<'a> Memoized for Union<'a> {
    fn inner_derive(&mut self, c: char) -> Box<Parser> {
        if self.left == (Empty {}) {
            self.left.derive(c)
        } else {
            self.right.derive(c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Epsilon, Parser};

    #[test]
    fn it_works() {
        fn is_nullable<T: Parser>(mut t: T) -> bool {
            t.is_nullable()
        }
        assert!(is_nullable(Epsilon {}));
    }
}
