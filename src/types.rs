#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Atom(char), 
    Op(char),
    Eof,
}

pub enum S {
    Atom(char), 
    Cons(char, Vec<S>)
}
