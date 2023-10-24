use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
#[allow(dead_code)]
use crate::ast::{Expression, Identifier, Statement};
use crate::ast::Expression::IdentifierExpression;
use crate::ast::Statement::ExpressionStatement;
use crate::token::Token;
use crate::lexer::Lexer;
use crate::parser::Parser;
#[test]
fn test_lexer() {
    let input = String::from("=+{}()");

    let result = vec![
        Token::ASSIGN,
        Token::PLUS,
        Token::LBRACE,
        Token::RBRACE,
        Token::LPAREN,
        Token::RPAREN
        ];
    let mut lexer = Lexer::new(input);
    for tok in result {
        assert_eq!(tok, lexer.next_token());
    }
}
#[test]
pub fn test_lexer2(){
    let input = String::from(
        "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);"
    );

    let result = vec![
        Token::LET,
        Token::IDENTIFIER("five".to_string()),
        Token::ASSIGN,
        Token::INT("5".to_string()),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER("ten".to_string()),
        Token::ASSIGN,
        Token::INT("10".to_string()),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER("add".to_string()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENTIFIER("x".to_string()),
        Token::COMMA,
        Token::IDENTIFIER("y".to_string()),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENTIFIER("x".to_string()),
        Token::PLUS,
        Token::IDENTIFIER("y".to_string()),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER("result".to_string()),
        Token::ASSIGN,
        Token::IDENTIFIER("add".to_string()),
        Token::LPAREN,
        Token::IDENTIFIER("five".to_string()),
        Token::COMMA,
        Token::IDENTIFIER("ten".to_string()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF
    ];

    let mut lexer = Lexer::new(input);
    for i in result{
        let token = lexer.next_token();
        assert_eq!(i,token);
    }
}

#[test]
fn test_lexer3() {
    let input = String::from("! = == != * /");
    let results = vec![
        Token::BANG,
        Token::ASSIGN,
        Token::EQ,
        Token::NotEq,
        Token::ASTERISK,
        Token::SLASH
    ];

    let mut lexer = Lexer::new(input);
    for result in results {
        assert_eq!(result, lexer.next_token());
    }
}

#[test]
fn test_parser() {
    let input = String::from("
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ");

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 3);
    let result_id = vec![
        "x",
        "y",
        "foobar"];
    let value_id = vec! [
        "5",
        "10",
        "838383"
    ];
    for i in 0..3 {
        assert!(test_parse_let_statement(program.statements.get(i).unwrap(), result_id.get(i).unwrap().to_string(), value_id[i].to_string()))
    }
}

fn test_parse_let_statement(stmt: &Statement, name: String, value: String) -> bool {
    if let Statement::LetStatement(id, expr) = stmt {
        assert_eq!(id.get_id(), name);
        assert_eq!(expr.to_string(), value);
        return true;
    }
    false
}
#[test]
fn test_return_statements() {
    let input = "\
    return 5;\
    return x;\
    return joer;\
    ";
    let results = vec![
        "5",
        "x",
        "joer"
    ];
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if program.statements.len() != 3 {
        panic!();
    }
    for i in 0..program.statements.len() {
        if let Statement::ReturnStatement(expr) = &program.statements[i]
        {
            assert_eq!(expr.to_string(), results[i]);
        }
        else {
            panic!();
        }
    }
}
#[test]
fn test_identifier_expression() {
    let input = "hola;\
                     adios;";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 2);
    if let Statement::ExpressionStatement(content) = program.statements.get(1).expect("No statement at 0") {
        if let Expression::IdentifierExpression(id) = content {
            if id.get_id() == "adios" {
                return;
            }
            else {
                panic!("strings didn't match")
            }
        }
        else {
            panic!("Expresion wasn't identifier expression")
        }
    }
    else {
        panic!("No expression estament")
    }
}
#[test]
fn test_integer_literals()  {
    let input = "5;";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);
    if let Statement::ExpressionStatement(expr) = program.statements.get(0).expect("No statement at 0") {
        if let Expression::IntegerExpression(_) = expr {
            assert_eq!(expr.to_string(), "5");
        }
    }
}

#[test]
fn test_parsing_prefix_expr() {
    let input = vec![
        "!5;",
        "-15;"
    ];
    let integers = vec! [
        5,
        15
    ];
    let operator = vec! [
        Token::BANG,
        Token::MINUS
    ];
    for i in 0..input.len() {
        let lexer = Lexer::new(input[i].to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);
        if let Statement::ExpressionStatement(expr) = &program.statements[0]
        {
            if let Expression::PrefixExpression(tok, inner_expr) = expr
            {
                if tok.get_type() == operator[i].get_type()
                {
                    if inner_expr.to_string() == integers[i].to_string()
                    {
                        return;
                    }
                }
            }
        }
        panic!();
    }
}
#[test]
fn test_parsing_infix_expr() {
    let inputs = vec![
        "5 + 5;",
        "5 - 5;",
        "5 / 5;",
        "5 > 5;",
        "5 < 5;",
        "5 == 5;",
        "5 != 5;",
    ];
    let toks = vec![
        Token::PLUS,
        Token::MINUS,
        Token::SLASH,
        Token::GT,
        Token::LT,
        Token::EQ,
        Token::NotEq
    ];
    for i in 0..inputs.len() {
        let lexer = Lexer::new(inputs[i].to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ExpressionStatement(expr) = stmt {
            if let Expression::InfixExpression(right, tok, _left) = expr
                {
                    assert_eq!(tok.clone(), toks[i]);
                    assert_eq!(right.to_string(), "5");
                    assert_eq!(right.to_string(), "5");
                }
        }
    }
}

enum ValueType {
    StringInput(String),
    IntInput(i64),
    BooleanInput(bool)
}

#[test]
fn test_complex_expr() {
    let input = "1 + 2 + 3;";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 1);
    assert_eq!(program.statements[0].to_string(), "((1 + 2) + 3);")
}

fn test_identifier(expr: &Expression, value: &String) -> bool {
    if let IdentifierExpression(id) = expr {
        if id.get_id() == value.clone() {
            return true;
        }
    }
    false
}

fn test_integer_literal(expr: &Expression, value: i64) -> bool {
   if let Expression::IntegerExpression(content) = expr {
       if value == *content {
           return true;
       }
   };
    false
}

fn test_boolean_literal(expr: &Expression, value: bool) -> bool {
    if let Expression::BoolExpression(content) = expr {
        if *content == value {
            return true;
        }
    }
    return false;
}

fn test_literal(expr: &Expression, value: ValueType) -> bool
{
    match value {
        ValueType::IntInput(content) => {
            test_integer_literal(expr, content)
        },
        ValueType::StringInput(content) => {
            test_identifier(expr, &content)
        },
        ValueType::BooleanInput(content) => {
            test_boolean_literal(expr, content)
        }
    }
}

fn test_infix_expression(expr: Expression, left: ValueType, right: ValueType, operator: Token){
    if let Expression::InfixExpression(left_expr, tok, right_expr) = expr {
        assert!(test_literal(left_expr.as_ref(), left));
        assert!(test_literal(right_expr.as_ref(), right));
        assert_eq!(tok.get_type(), operator.get_type());
        return;
    }
    panic!("Expected an InfixExpression but found otherwise");
}

#[test]
fn test_infixes() {
    let expr = Expression::InfixExpression(Box::new(Expression::IdentifierExpression(Identifier::new("hola".to_string()))), Token::ASSIGN, Box::new(Expression::IntegerExpression(8)));
    test_infix_expression(expr, ValueType::StringInput("hola".to_string()), ValueType::IntInput(8), Token::ASSIGN);
}

#[test]
fn test_booleans() {
    let input = "true";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 1);
    if let ExpressionStatement(expr) = &program.statements[0] {
        if let Expression::BoolExpression(content) = expr {
            assert_eq!(*content, true);
            return;
        }
    }
    panic!();
}
#[test]
fn test_group_expressions() {
    let inputs = vec![
        "(2 + 3) - 7;",
        "(true == true) != false;",
        "7 + (3 + 9);"
    ];
    for input in inputs {
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        for stmt in program.statements
        {

        }

    }
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x } else { y }";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    if let ExpressionStatement(expr) = &program.statements[0] {
        if let Expression::IfExpression(content) = expr {
            println!("{}",expr.to_string());
            test_infix_expression(content.condition.as_ref().clone(), ValueType::StringInput("x".to_string()), ValueType::StringInput("y".to_string()), Token::LT);
            assert_eq!(content.consequence.len(), 1);
            assert_eq!(content.consequence[0].to_string(), "x;");
            if let Some(alt_cont) = &content.alternative {
                assert_eq!(alt_cont.len(), 1);
                assert_eq!(alt_cont[0].to_string(), "y;");
                return;
            }
            else {
                panic!("Alternative was None");
            }
        }
    }

    panic!();
}

#[test]
fn test_fn_expression() {
    let input = "fn(x,y){x + y;}";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    if let ExpressionStatement(expr) = &program.statements[0] {
        if let Expression::FnExpression(content) = expr {
            assert_eq!(content.params.len(), 2);
            assert_eq!(content.params[0].get_id(), "x");
            assert_eq!(content.params[1].get_id(), "y");
            assert_eq!(content.body.len(), 1);
            println!("{}",expr.to_string());
        }
        else {
            panic!("Expression was not FnExpression");
        }
    }
    else {
        panic!("Statement was not ExpressionStatement");
    }
}

#[test]
fn test_call_expression() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 1);
    if let ExpressionStatement(expr) = &program.statements[0]
    {
        if let Expression::CallExpression(content) = expr {
            assert_eq!(content.function.to_string().as_str(), "add");
            assert_eq!(content.args.len(), 3);
            assert_eq!(content.args[0].to_string(), "1");
            assert_eq!(content.args[1].to_string(), "(2 * 3)");
            assert_eq!(content.args[2].to_string(), "(4 + 5)");
        }
        else {
            panic!("Expected call expression");
        }
    }
    else {
        panic!("Expected expression statement")
    }
}

#[test]
fn test_array_literal() {
    let input = "[1,2,3];";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(1, program.statements.len());
    assert_eq!(program.statements[0].to_string(), "[1,2,3];");
}

#[test]
fn test_index_literal()
{
    let input = "array[1 + 2];array[3];";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(2, program.statements.len());
    assert_eq!(program.statements[0].to_string(), "array[(1 + 2)];");
}

#[test]
fn test_hash_literal() {
    let input = "{2: 3, 3: 2}";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(1, program.statements.len());
    assert_eq!(program.statements[0].to_string(), "{2:3,3:2};");
}

#[test]
fn hash_string()
{
    let mut state = DefaultHasher::new();
    assert_eq!("hola".to_string().hash(& mut state), "hola".to_string().hash(& mut state))
}