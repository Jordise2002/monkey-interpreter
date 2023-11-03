use crate::ast::{Identifier, Program, Statement, Expression, IfStruct, FnStruct, CallStruct, ArrayStruct, IndexStruct, HashStruct};
use crate::lexer::Lexer;
use crate::parser::Precedence::Lowest;
use crate::token::Token;
use crate::token::Token::{COMMA, RBRACE};

#[derive(Copy, Clone, Debug)]
pub enum Precedence{
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
    Index = 7,
    Hash = 8
}

impl Precedence {
    pub fn get_precendence(tok: &Token) -> Precedence {
        match tok {
            Token::EQ => {
                Precedence::Equals
            },
            Token::NotEq => {
                Precedence::Equals
            },
            Token::LT => {
                Precedence::LessGreater
            },
            Token::GT  => {
                Precedence::LessGreater
            },
            Token::PLUS => {
                Precedence::Sum
            },
            Token::MINUS => {
                Precedence::Sum
            },
            Token::SLASH => {
                Precedence::Product
            },
            Token::ASTERISK => {
                Precedence::Product
            },
            Token::LPAREN => {
                Precedence::Call
            },
            Token::LBRACKET => {
                Precedence::Index
            }
            Token::SEMICOLON => {
                Precedence::Hash
            }
            _ => {
                Precedence::Lowest
            }
        }
    }
}
pub struct Parser{
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>
}

impl Parser{
    pub fn new(lexer: Lexer) -> Self
    {
        let errors = Vec::new();
        let mut p = Parser{
            lexer,
            cur_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
            errors
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn peek_error(&mut self, token: Token) {
        self.errors.push(format!("Expected {} and found {}", self.peek_token.get_type(), token.get_type()));
    }

    fn peek_precedence(&self) -> Precedence{
        Precedence::get_precendence(&self.peek_token)
    }

    fn peek_token(&mut self, token: Token) -> bool
    {
        return if self.peek_token.get_type() == token.get_type() {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }

    }

    fn parse_let_statement(&mut self) -> Option<Statement>{
        if self.peek_token(Token::IDENTIFIER("".to_string()))
        {
            if let Token::IDENTIFIER(content) = self.cur_token.clone()
            {
                let identifier = Identifier::new(content);
                let expr = if self.peek_token(Token::ASSIGN)
                {
                    self.next_token();
                    self.parse_expr(Lowest)
                }
                else {
                    self.next_token();
                    return None;
                };

                if self.peek_token == Token::SEMICOLON {
                    self.next_token();
                }
                return Some(Statement::LetStatement(identifier, expr));
            }
        }
        None
    }

    fn parse_return_statement(&mut self) -> Option<Statement>
    {
        self.next_token();
        let expr = self.parse_expr(Lowest);
        if self.peek_token == Token::SEMICOLON {
            self.next_token();
        }
        Some(Statement::ReturnStatement(expr))
    }

    fn parse_expr_statement(&mut self) -> Option<Statement>
    {
        let stmt = Statement::ExpressionStatement(self.parse_expr(Precedence::Lowest));
        if self.peek_token == Token::SEMICOLON
        {
            self.next_token();
        }
        Some(stmt)
    }

    fn parse_prefix_expr(& mut self) -> Expression {
        let tok = self.cur_token.clone();
        self.next_token();
        let expr = self.parse_expr(Precedence::Prefix);
        Expression::PrefixExpression(tok, Box::new(expr))
    }

    fn parse_infix_expr(&mut self, left: Expression) -> Expression {
        let tok = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expr(Precedence::get_precendence(&tok));
        Expression::InfixExpression(Box::new(left), tok.clone(), Box::new(right))
    }

    fn parse_group_expression(& mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expr(Lowest);

        if !self.peek_token(Token::RPAREN)
        {
            return None;
        }
        Some(exp)
    }

    fn parse_if_expression(& mut self) -> Option<Expression> {
        if !self.peek_token(Token::LPAREN)
        {
            return None;
        }

        self.next_token();
        let condition = self.parse_expr(Lowest);

        if !self.peek_token(Token::RPAREN)
        {
            return None;
        }

        if !self.peek_token(Token::LBRACE)
        {
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token == Token::ELSE {
            self.next_token();

            if !self.peek_token(Token::LBRACE)
            {
                 None
            }
            else {
                Some(self.parse_block_statement())
            }

        }
        else {
            None
        };
        Some(Expression::IfExpression(IfStruct{condition: Box::new(condition), consequence: consequence, alternative}))


    }

    fn parse_block_statement(&mut self) -> Vec<Statement>
    {
        let mut block_statement = Vec::new();
        self.next_token();
        while self.cur_token != Token::RBRACE && self.cur_token != Token::EOF {
            let stmt = self.parse_statement();
            if let Some(content) = stmt {
                block_statement.push(content)
            }
            self.next_token();
        }
        block_statement
    }

    fn parse_params(&mut self) -> Option<Vec<Identifier>>
    {
        let mut params = Vec::new();
        if self.peek_token == Token::RPAREN {
            self.next_token();
            return Some(params);
        }

        self.next_token();

        if let Token::IDENTIFIER(content) = &self.cur_token {
            params.push(Identifier::new(content.clone()));
        }
        else {
            return None;
        }

        while self.peek_token == Token::COMMA {
            self.next_token();
            self.next_token();
            if let Token::IDENTIFIER(content) = &self.cur_token {
                params.push(Identifier::new(content.clone()));
            }
            else{
                return None;
            }
        }

        if !self.peek_token(Token::RPAREN) {
            return None;
        }

        Some(params)
    }
    
    fn parse_fn_literal(& mut self) -> Option<Expression> {
        if !self.peek_token(Token::LPAREN){
            return None;
        }

        let params = self.parse_params().expect("Error reading params");

        if !self.peek_token(Token::LBRACE){
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FnExpression(FnStruct{params, body}))
    }
    
    fn parse_call_args_expr(& mut self) -> Option<Vec<Expression>> {
        let mut args = Vec::new();

        if self.peek_token == Token::RPAREN {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        args.push(self.parse_expr(Lowest));

        while self.peek_token == Token::COMMA {
            self.next_token();
            self.next_token();
            args.push(self.parse_expr(Lowest));
        }

        if !self.peek_token(Token::RPAREN)
        {
            return None;
        }
        Some(args)
    }
    
    fn parse_call_expr(& mut self, function: Expression) -> Option<Expression> {
        let args = self.parse_call_args_expr();
        match args {
            Some(content) =>
                {
                    Some(Expression::CallExpression(CallStruct{function: Box::new(function), args: content}))
                }
            None =>
                {
                    None
                }
        }
    }

    fn parse_expression_list(&mut self) -> Option<Vec<Expression>>
    {
        let mut args = Vec::new();

        if self.peek_token == Token::RBRACKET {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        args.push(self.parse_expr(Lowest));

        while self.peek_token == Token::COMMA {
            self.next_token();
            self.next_token();
            args.push(self.parse_expr(Lowest));
        }

        if !self.peek_token(Token::RBRACKET)
        {
            return None;
        }
        Some(args)
    }
    fn parse_array_literal(&mut self) -> Option<Expression>
    {
        if let Some(content) = self.parse_expression_list()
        {
            Some(Expression::ArrayLiteral(ArrayStruct{ elements: content}))
        }
        else {
            None
        }
    }

    fn parse_index_expr(&mut self, left: Expression) -> Option<Expression>
    {
        self.next_token();
        let index = self.parse_expr(Lowest);

        if !self.peek_token(Token::RBRACKET)
        {
            None
        }
        else {
            Some(Expression::IndexExpression(IndexStruct{left: Box::new(left), index: Box::new(index) }))
        }
    }

    fn parse_hash_expr(&mut self) -> Option<Expression>
    {
        let mut result = HashStruct::new();
        while self.peek_token != Token::RBRACE
        {
            self.next_token();
            let key = self.parse_expr(Lowest);
            if !self.peek_token(Token::COLON)
            {
                return None;
            }
            self.next_token();
            let value = self.parse_expr(Lowest);
            result.pairs.push((key.clone(), value.clone()));
            if self.peek_token != RBRACE && !self.peek_token(COMMA)
            {
                return None;
            }
        }
        if !self.peek_token(Token::RBRACE)
        {
            return None;
        }
        Some(Expression::HashExpression(result))
    }

    fn parse_expr(& mut self, prec: Precedence) -> Expression
    {

        let mut expr = match &self.cur_token
        {
            Token::IDENTIFIER(_) =>
                {
                    self.parse_identifier().unwrap()
                },
            Token::TRUE =>
                {
                Expression::BoolExpression(true)
                },
            Token::STRING(content) =>
                {
                    Expression::StringExpression(content.clone())
                }
            Token::FUNCTION =>
                {
                    self.parse_fn_literal().expect("Couldn't parse function expression")
                }
            Token::IF => {
                self.parse_if_expression().expect("Couldn't parse if expression")
            }
            Token::FALSE =>
                {
                Expression::BoolExpression(false)
                },
            Token::LPAREN =>
                {
                    self.parse_group_expression().expect("No ) found to match (")
                }
            Token::INT(content) =>
                {
                    self.parse_integer(content).expect("The content of the token couldn't be parse to an integer")
                },
            Token::BANG =>
                {
                    self.parse_prefix_expr()
                },
            Token::MINUS =>
                {
                    self.parse_prefix_expr()
            },
            Token::LBRACKET =>
                {
                    self.parse_array_literal().expect("Couldn't parse array literal")
                }
            Token::LBRACE =>
                {
                    self.parse_hash_expr().expect("Couldn't parse hash literal")
                }
            _ => {
                return Expression::None;
            }
        };
        while self.peek_token != Token::SEMICOLON && (prec as u32) < (self.peek_precedence() as u32) {
            expr = match self.peek_token {
                Token::PLUS => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::MINUS => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::LPAREN => {
                    self.next_token();
                    self.parse_call_expr(expr).expect("Couldn't parse call expression")
                },
                Token::LBRACKET => {
                    self.next_token();
                    self.parse_index_expr(expr).expect("Couldn't parse index expression")
                }
                Token::SLASH => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::ASTERISK => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::EQ => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::NotEq => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::LT => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                },
                Token::GT => {
                    self.next_token();
                    self.parse_infix_expr(expr)
                }
                _ => {
                    return expr;
                }
            };
        }
        expr
    }

    fn parse_integer(&self, content: &String) -> Option<Expression> {
        match content.parse::<i64>() {
            Ok(content) =>
                {
                    Some(Expression::IntegerExpression(content))
                },
            _  =>
                {
                    None
                }
        }
    }
    fn parse_identifier(&mut self) -> Option<Expression> {

        if let Token::IDENTIFIER(content) = &self.cur_token {
            return Some(Expression::IdentifierExpression(Identifier::new(content.clone())))
        }
        None
    }
    fn parse_statement(&mut self) -> Option<Statement>
    {
        match self.cur_token {
            Token::LET =>
                {
                    self.parse_let_statement()
                },
            Token::RETURN =>
            {
                self.parse_return_statement()
            },
            _ => {
                self.parse_expr_statement()
            }
        }
    }

    pub fn parse_program(&mut self) -> Program
    {
        let mut program = Program::new();
        while self.cur_token != Token::EOF
        {
            let stmt = self.parse_statement();
            if let Some(content) = stmt {
                program.statements.push(content);
            }
            self.next_token();
        }
        program
    }
}
