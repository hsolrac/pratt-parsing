#[derive(Debug)]
enum Token {
    Atom(char), 
    Op(char),
    Eof,
}

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens = input 
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 
                'a'..='z' | 'A'..='Z' => Token::Atom(c), 
                _=> Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        
        Lexer { tokens }
    }
}


fn main() {
    let tokens = "1 + 2";

    println!("{:?}", Lexer::new(tokens))
}
