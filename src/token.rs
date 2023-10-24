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
