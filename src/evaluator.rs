use std::fmt::format;
use crate::ast::{Expression, Identifier, IfStruct, Node, Program, Statement};
use crate::environment::Environment;
use crate::object::{FunctionStruct, Object};
use crate::object::Object::ReturnValue;
use crate::token::Token;

pub fn eval(node: Node, env: & mut Environment) -> Option<Object> {
    match node {
        Node::Program(prog) => {
            eval_program(prog.statements, env)
        },
        Node::Statement(stmt) => {
            eval_statement(stmt, env)
        },
        Node::Expression(expr) => {
            eval_expr(&expr, env)
        },
        Node::StatementBlock(block) => {
            eval_block_statement(block,env)
        }
    }
}

fn eval_program(block: Vec<Statement>, env: & mut Environment) -> Option<Object>
{

    let mut result = None;
    for stmt in block {
        result = eval_statement(stmt, env);
        if let Some(content) = result.clone() {
            if let Object::ReturnValue(object) = content {
                return Some(object.as_ref().clone())
            }
            if let Object::Error(_) = content {
                return result;
            }
        }
    }
    result
}

fn eval_block_statement(block: Vec<Statement>, env: & mut Environment) -> Option<Object>
{
    let mut result = None;
    for stmt in block {
        result = eval_statement(stmt, env);
        if let Some(content) = &result {
            if let ReturnValue(_) = content {
                return result;
            }
            if let Object::Error(_) = content {
                return result;
            }
        }
    }

    result
}

fn eval_statement(stmt: Statement, env: & mut Environment) -> Option<Object>
{
    match stmt {
        Statement::ExpressionStatement(expr) => {
            eval(Node::Expression(expr), env)
        },
        Statement::ReturnStatement(expr) => {
            let inner_value = eval(Node::Expression(expr), env);
            match inner_value {
                Some(content) => {
                    Some(ReturnValue(Box::new(content)))
                },
                None => {
                    None
                }
            }
        },
        Statement::LetStatement(id, expr) => {
            let val = eval(Node::Expression(expr), env);
            match val {
                Some(content) => {
                    env.set(id.get_id(), content);
                    Some(Object::Null)
                },
                _=> {
                    None
                }
            }
        },
        _ => {
            panic!()
        }
    }
}

fn eval_expr(expr: &Expression, env: & mut Environment) -> Option<Object> {
    match expr {
        Expression::IntegerExpression(content) =>
            {
                Some(Object::IntegerObject(content.clone()))
            },
        Expression::BoolExpression(content) =>
            {
                Some(Object::BooleanObject(content.clone()))
            },
        Expression::StringExpression(content) => {
            Some(Object::StringObject(content.clone()))
        }
        Expression::PrefixExpression(tok, right) =>
            {
                let right = eval_expr(right.as_ref(), env);
                eval_prefix_expr(tok, right)
            },
        Expression::InfixExpression(right, tok, left) =>
            {
                let right = eval_expr(right.as_ref(), env);
                let left = eval_expr(left.as_ref(), env);
                if left.is_none() || right.is_none() {
                    return None;
                }
                Some(eval_infix_expr(right.unwrap(), left.unwrap(), tok))
            },
        Expression::IfExpression(content) =>
            {
                eval_if_expr(content.clone(), env)
            },
        Expression::IdentifierExpression(id) =>
            {
                Some(eval_identifier(id, env))
            },
        Expression::FnExpression(content) => {
            Some(Object::Function(FunctionStruct::new(content.params.clone(),content.body.clone(), env.clone())))
        },
        Expression::CallExpression(content) => {
           let function =  eval(Node::Expression(content.function.as_ref().clone()), env).expect("Couldn't parse function");
            if let Object::Error(_) = &function {
                return Some(function);
            }
            let args = eval_expressions(content.args.clone(), env);
            if args.len() == 1 {
                if let Object::Error(_) = &args[0] {
                    return Some(args[0].clone());
                }
            }

            Some(apply_function(function, args))

        }
        _ => {
            None
        }
    }
}

fn unwrap_return_value(return_object: Object) -> Object {
    if let ReturnValue(content) = &return_object
    {
        return content.as_ref().clone();
    }
    return_object
}

fn apply_function(function: Object, args: Vec<Object>) -> Object
{
    if let Object::Function(content) = function {
        let mut exp_env = extend_function_env(&content, args);
        let evaluated = eval(Node::StatementBlock(content.body), & mut exp_env).expect("couldn't parse functions body");
        return unwrap_return_value(evaluated);
    }
    Object::Error(format!("Not a function {}", function.get_type()))
}

fn extend_function_env(function_struct: &FunctionStruct, args: Vec<Object>) -> Environment {
    let mut env = Environment::new_with_superior(Box::new(function_struct.env.clone()));
    for i in 0..args.len() {
        env.set(function_struct.parameters[i].get_id(), args[i].clone());
    }
    env
}
fn eval_expressions(exprs: Vec<Expression>, env: & mut Environment) -> Vec<Object>
{
    let mut result = Vec::new();
    for expr in exprs {
        let evaluated = eval(Node::Expression(expr), env);
        if let Some(content) = evaluated {
            if let Object::Error(_) = content {
                return vec![content.clone()];
            }
            result.push(content);
        }
    }
    result
}

fn eval_identifier(id: &Identifier, env: & mut Environment) -> Object
{
    env.get(id.get_id())
}

fn eval_prefix_expr(tok:&Token, right: Option<Object>) -> Option<Object>
{
    if let Some(right_content) = right {
        match tok {
            Token::BANG =>
                {
                    Some(eval_bang_operator(right_content))
                },
            Token::MINUS =>
                {
                    Some(eval_minus_operator(right_content))
                },
            _ => {
                Some(Object::Error(format!("unknown operator: {}", tok.get_type())))
            }
        }
    }
    else {
        None
    }
}

fn eval_if_expr(if_struct: IfStruct, env: & mut Environment) -> Option<Object>
{
    let condition = eval(Node::Expression(if_struct.condition.as_ref().clone()), env);
    if let Some(condition) = condition {
        if is_true(condition)
        {
            return eval(Node::StatementBlock(if_struct.consequence), env);
        }
        else {
            if let Some(alternative) = if_struct.alternative
            {
                return eval(Node::StatementBlock(alternative), env);
            }
            return Some(Object::Null);
        }
    }
    return None;
}

fn is_true(object: Object) -> bool {
    match object {
        Object::IntegerObject(content) =>
            {
                if content != 0
                {
                    true
                }
                else {
                    false
                }
            },
        Object::BooleanObject(content) => {
            content
        },
        _ => {
            false
        }
    }
}


fn eval_bang_operator(inner_object: Object) -> Object
{
    match inner_object
    {
        Object::BooleanObject(content) =>
            {
                if content {
                    Object::BooleanObject(false)
                }
                else {
                    Object::BooleanObject(true)
                }
            },
        Object::IntegerObject(content) =>
            {
                if content == 0 {
                    Object::BooleanObject(true)
                }
                else {
                    Object::BooleanObject(false)
                }
            }
        _ => {
            Object::Null
        }
    }
}

fn eval_minus_operator(inner_object: Object) -> Object
{
    if let Object::IntegerObject(content) = inner_object {
        Object::IntegerObject(-content)
    }
    else {
        Object::Error(format!("unknown operator: {} {}", Token::MINUS.get_type(), inner_object.get_type()))
    }
}

fn eval_infix_expr(right:Object, left:Object, operator: &Token) -> Object
{
    if let Object::IntegerObject(right_content) = right {
        if let Object::IntegerObject(left_content) = left {
            return eval_integer_infix_expr(right_content, left_content, operator);
        }
        else {
            return Object::Error(format!("type mismatch: {} {} {}", right.get_type(), operator.get_type(), left.get_type()))
        }
    }
    if let Object::BooleanObject(right_content) = right {
        if let Object::BooleanObject(left_content) = left {
            return eval_bool_infix_expr(right_content, left_content, operator);
        }
        else {
            return Object::Error(format!("type mismatch: {} {} {}", right.get_type(), operator.get_type(), left.get_type()))
        }
    }
    if let Object::StringObject(right_content) = &right {
        if let Object::StringObject(left_content) = &left
        {
            return eval_string_infix_expr(right_content.clone(), left_content.clone(), operator)
        }
        else
        {
            return Object::Error(format!("type mismatch: {} {} {}", right.get_type(), operator.get_type(), left.get_type()))
        }
    }
    Object::Error(format!("unknown operator: {}", operator.get_type()))
}

fn eval_string_infix_expr(right: String, left: String, operator: &Token) -> Object {
    match operator
    {
        Token::PLUS => {
            Object::StringObject(right + left.as_str())
        },
        _ => {
            Object::Error(format!("unknown operator: STRING {} STRING", operator.get_type()))
        }
    }
}
fn eval_integer_infix_expr(right:i64, left:i64, operator: &Token) -> Object
{
    match operator
    {
        Token::PLUS =>
            {
                Object::IntegerObject(right + left)
            },
        Token::MINUS =>
            {
                Object::IntegerObject(right - left)
            },
        Token::ASTERISK =>
            {
                Object::IntegerObject(right * left)
            },
        Token::SLASH =>
            {
                Object::IntegerObject(right / left)
            },
        Token::LT =>
            {
                Object::BooleanObject(right < left)
            },
        Token::GT =>
            {
                Object::BooleanObject(right > left)
            },
        Token::EQ =>
            {
                Object::BooleanObject(right == left)
            },
        Token::NotEq =>
            {
                Object::BooleanObject(right != left)
            }
        _ => {
            Object::Null
        }
    }
}

fn eval_bool_infix_expr(right: bool, left: bool, tok: &Token) -> Object
{
    match tok {
        Token::EQ => {
            Object::BooleanObject(right == left)
        }
        Token::NotEq => {
            Object::BooleanObject(right != left)
        }
        _ => {
            Object::Error(format!("unknown operator: {} {} {}", Object::BooleanObject(right).get_type(),tok.get_type(), Object::BooleanObject(left).get_type() ))
        }
    }
}