#[derive(Debug, PartialEq)]
enum Token {
    Keyword(String),
    Identifier(String),
    Number(i64),
    Operator(String),
    Punctuation(String),
    Whitespace,
    Unknown(char),
}

struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        if current_char.is_whitespace() {
            self.position += 1;
            return Some(Token::Whitespace);
        }

        if current_char.is_digit(10) {
            let start = self.position;
            while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_digit(10) {
                self.position += 1;
            }
            let number = self.input[start..self.position].parse::<i64>().unwrap();
            return Some(Token::Number(number));
        }

        if current_char.is_alphabetic() {
            let start = self.position;
            while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_alphanumeric() {
                self.position += 1;
            }
            let word = &self.input[start..self.position];
            match word {
                "if" | "else" | "def" | "return" => return Some(Token::Keyword(word.to_string())),
                _ => return Some(Token::Identifier(word.to_string())),
            }
        }

        if "+-*/=".contains(current_char) {
            self.position += 1;
            return Some(Token::Operator(current_char.to_string()));
        }

        if "():,{}".contains(current_char) {
            self.position += 1;
            return Some(Token::Punctuation(current_char.to_string()));
        }

        self.position += 1;
        Some(Token::Unknown(current_char))
    }
}

fn main() {
    let input = String::from("def add(a, b):\n    return a + b");
    let mut lexer = Lexer::new(input);

    while let Some(token) = lexer.next_token() {
        if token != Token::Whitespace {
            println!("{:?}", token);
        }
    }
}
