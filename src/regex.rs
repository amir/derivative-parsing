struct Empty;
struct Epsilon;
struct Literal {
    c: char,
}
struct Concat<'a> {
    left: &'a Parser,
    right: &'a Parser,
}

trait Parser {
    fn is_nullable(&self) -> bool;
    fn derive(&self, c: char) -> Box<Parser>;
    fn concat<'a>(&'a self, that: &'a Parser) -> Box<Parser + 'a>
    where
        Self: Sized,
    {
        Box::new(Concat {
            left: self,
            right: that,
        })
    }
}

impl Parser for Epsilon {
    fn is_nullable(&self) -> bool {
        true
    }

    fn derive(&self, _: char) -> Box<Parser> {
        Box::new(Empty {})
    }
}

impl Parser for Empty {
    fn is_nullable(&self) -> bool {
        false
    }

    fn derive(&self, _: char) -> Box<Parser> {
        panic!("Cannot derive the empty parser")
    }
}

impl Parser for Literal {
    fn is_nullable(&self) -> bool {
        false
    }

    fn derive(&self, c: char) -> Box<Parser> {
        if self.c == c {
            Box::new(Epsilon {})
        } else {
            Box::new(Empty {})
        }
    }
}

impl<'a> Parser for Concat<'a> {
    fn is_nullable(&self) -> bool {
        self.left.is_nullable() && self.right.is_nullable()
    }

    fn derive(&self, c: char) -> Box<Parser> {
        if self.left.is_nullable() {
            self.left.derive(c)
        } else {
            self.left.derive(c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Epsilon, Parser};

    #[test]
    fn it_works() {
        fn is_nullable<T: Parser>(t: T) -> bool {
            t.is_nullable()
        }
        assert!(is_nullable(Epsilon {}));
    }
}
