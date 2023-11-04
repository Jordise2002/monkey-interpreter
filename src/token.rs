use phf::phf_map;
use strum_macros::IntoStaticStr;
#[derive(PartialEq, Debug, Clone, IntoStaticStr, Hash)]
pub enum Token{
    EMPTY,
    ILLEGAL,
    EOF,

    IDENTIFIER(String),
    INT(String),
    STRING(String),

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,
    EQ,
    NotEq,

    COMMA,
    SEMICOLON,
    COLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN
}

  
static KEYWORDS: phf::Map<&'static str, Token> = phf_map!{
    "let" => Token::LET,
    "fn" => Token::FUNCTION,
    "true" => Token::TRUE,
    "false" => Token::FALSE,
    "if" => Token::IF,
    "else" => Token::ELSE,
    "return" => Token::RETURN
};

impl Token {

    pub fn get_type(&self) -> &'static str
    {
        let string: & 'static str = self.into();
        string
    }

    pub fn inspect(&self) -> String
    {
        match self {
            Token::EMPTY=> "".to_string(),
            Token::ILLEGAL => "ILLEGAL".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::IDENTIFIER(content) => content.clone(),
            Token::INT(content) => content.clone(),
            Token::STRING(content) => content.clone(),
            Token::ASSIGN => "=".to_string(),
            Token::PLUS => "+".to_string(),
            Token::MINUS => "-".to_string(),
            Token::BANG => "!".to_string(),
            Token::ASTERISK => "*".to_string(),
            Token::SLASH => "/".to_string(),
            Token::LT => "<".to_string(),
            Token::GT => ">".to_string(),
            Token::EQ => "==".to_string(),
            Token::NotEq => "!=".to_string(),
            Token::COMMA => ",".to_string(),
            Token::SEMICOLON => ";".to_string(),
            Token::COLON => ":".to_string(),
            Token::LPAREN => "(".to_string(),
            Token::RPAREN => ")".to_string(),
            Token::LBRACE => "{".to_string(),
            Token::RBRACE => "}".to_string(),
            Token::LBRACKET => "[".to_string(),
            Token::RBRACKET => "]".to_string(),
            Token::FUNCTION => "fn".to_string(),
            Token::LET => "let".to_string(),
            Token::TRUE => "true".to_string(),
            Token::FALSE => "false".to_string(),
            Token::IF => "if".to_string(),
            Token::ELSE => "else".to_string(),
            Token::RETURN => "return".to_string()
        }
    }

}
pub fn look_up_token(keyword: String) -> Token
{
    match KEYWORDS.get(&keyword)
    {
        Some(tok) =>
        {
            tok.clone()
        },
        None => {
            Token::IDENTIFIER(keyword)
        }
    }
}
