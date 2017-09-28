trait Parser {
    fn derive(&self, c: char) -> Box<Parser>;
    fn is_nullable(&self) -> bool;
}

struct Empty;
struct Epsilon;
struct Literal {
    c: char,
}

impl Parser for Empty {
    fn derive(&self, _: char) -> Box<Parser> {
        Box::new(Empty {})
    }

    fn is_nullable(&self) -> bool {
        false
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
