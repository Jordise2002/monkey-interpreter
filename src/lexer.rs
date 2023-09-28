use crate::token::Token;
use crate::token;

pub struct Lexer{
    input: String, 
    position: usize,
    next_position:usize,
    ch: char
}

impl Lexer {
    pub fn new(input: String) -> Self
    {
        let mut l = Lexer {
            input,
            position : 0,
            next_position: 0,
            ch: '\0'
        };
        l.read_char();
        return l;

    }

    fn read_char(&mut self)
    {
        let aux = self.input.chars().nth(self.next_position);
        if let None = aux{
            self.ch = '\0';
        }
        else {
            self.ch = aux.unwrap();
        }
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn read_identifier(& mut self) -> String
    {
        let position = self.position;
        while self.ch.is_ascii_alphabetic(){
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    fn read_number(& mut self) -> String
    {
        let position = self.position;
        while self.ch.is_ascii_digit()
        {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    fn skip_withespace(& mut self){
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r'
        {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char
    {
        let aux = self.input.chars().nth(self.next_position);
        if let None = aux {
            '\0'
        }
        else
        {
            aux.unwrap()
        }
    }

    fn read_string(&mut self) -> Token{
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0'{
                break
            }
        }
        Token::STRING(self.input[position..self.position].to_string())
    }

    pub fn next_token(& mut self) -> Token
    {


        self.skip_withespace();

        let tok = match self.ch
        {
            '=' => {
                if self.peek_char() == '='
                {
                    self.read_char();
                    Token::EQ

                }
                else
                {
                    Token::ASSIGN
                }
            },
            ';' => {
                Token::SEMICOLON
            },
            '(' => {
                Token::LPAREN
            },
            ')' => {
                Token::RPAREN
            },
            ',' => {
                Token::COMMA
            },
            '+' => {
                Token::PLUS
            },
            '-' => {
                Token::MINUS
            },
            '!' => {
                if self.peek_char() == '='
                {
                    self.read_char();
                    Token::NotEq
                }
                else
                {
                    Token::BANG
                }
            },
            '*' => {
                Token::ASTERISK
            },
            '/' => {
                Token::SLASH
            },
            '<' => {
                Token::LT
            },
            '>' => {
                Token::GT
            }
            '{' => {
                Token::LBRACE
            },
            '[' => {
                Token::LBRACKET
            },
            ']' => {
                Token::RBRACKET
            }
            '}' => {
                Token::RBRACE
            }
            '\0' => {
                Token::EOF
            }
            '"' => {
                self.read_string()
            }
            _ =>
            {
                if self.ch.is_ascii_alphabetic() {
                    let tok = token::look_up_token(self.read_identifier());
                    return tok;
                }
                else if self.ch.is_ascii_digit(){
                    let tok = Token::INT(self.read_number());
                    return tok;
                }
                else {
                    let tok = Token::ILLEGAL;
                    return tok;
                }
            }
        };

        self.read_char();

        return tok;
    }
}

