use crate::ast::{Expression, Identifier, IfStruct, Node,Statement};
use crate::builtins::get_built_in;
use crate::environment::Environment;
use crate::object::{FunctionStruct, Object};
use crate::object::Object::{Null, ReturnValue};
use crate::token::Token;

pub fn eval(node: Node, env: & mut Environment) -> Object {
    match node {
        Node::Program(prog) => {
            eval_program(prog.statements, env)
        },
        /*Node::Statement(stmt) => {
            eval_statement(stmt, env)
        },*/
        Node::Expression(expr) => {
            eval_expr(&expr, env)
        },
        Node::StatementBlock(block) => {
            eval_block_statement(block,env)
        }
    }
}

fn eval_program(block: Vec<Statement>, env: & mut Environment) -> Object
{

    let mut result = Null;
    for stmt in block {
        result = eval_statement(stmt, env);
        if let Object::ReturnValue(object) = &result {
            return object.as_ref().clone()
        }
        if let Object::Error(_) = &result {
            return result;
        }
    }
    result
}

fn eval_block_statement(block: Vec<Statement>, env: & mut Environment) -> Object
{
    let mut result = Object::Null;
    for stmt in block {
        result = eval_statement(stmt, env);
        if let ReturnValue(_) = &result {
            return result;
        }
        if let Object::Error(_) = &result {
            return result;
        }
    }

    result
}

fn eval_statement(stmt: Statement, env: & mut Environment) -> Object
{
    match stmt {
        Statement::ExpressionStatement(expr) => {
            eval(Node::Expression(expr), env)
        },
        Statement::ReturnStatement(expr) => {
            let inner_value = eval(Node::Expression(expr), env);
            ReturnValue(Box::new(inner_value))
        },
        Statement::LetStatement(id, expr) => {
            let val = eval(Node::Expression(expr), env);
            if val.is_error() {
                return val;
            }
            env.set(id.get_id(), val);
            Null
        },

    }
}

fn eval_expr(expr: &Expression, env: & mut Environment) -> Object {
    match expr {
        Expression::IntegerExpression(content) =>
            {
                Object::IntegerObject(content.clone())
            },
        Expression::BoolExpression(content) =>
            {
                Object::BooleanObject(content.clone())
            },
        Expression::StringExpression(content) => {
            Object::StringObject(content.clone())
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
                if left.is_error() {
                    return left;
                }
                if right.is_error() {
                    return right;
                }
                eval_infix_expr(right, left, tok)
            },
        Expression::IfExpression(content) =>
            {
                eval_if_expr(content.clone(), env)
            },
        Expression::IdentifierExpression(id) =>
            {
                eval_identifier(id, env)
            },
        Expression::FnExpression(content) => {
            Object::Function(FunctionStruct::new(content.params.clone(),content.body.clone(), env.clone()))
        },
        Expression::CallExpression(content) => {
           let function =  eval(Node::Expression(content.function.as_ref().clone()), env);
            if let Object::Error(_) = &function {
                return function;
            }
            let args = eval_expressions(content.args.clone(), env);
            if args.len() == 1 {
                if let Object::Error(_) = &args[0] {
                    return args[0].clone();
                }
            }

            apply_function(function, args)

        }
        _ => {
            Object::Error(format!("Expression not suported: {}", expr.to_string()))
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
        let evaluated = eval(Node::StatementBlock(content.body), & mut exp_env);
        if evaluated.is_error()
        {
            return evaluated;
        }
        return unwrap_return_value(evaluated);
    }
    else if let Object::BuiltIn(content) = function {
        return content(args);
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
        if let Object::Error(_) = evaluated {
            return vec![evaluated.clone()];
        }
        result.push(evaluated);
    }
    result
}

fn eval_identifier(id: &Identifier, env: & mut Environment) -> Object
{

    if let Some(content) = env.get(id.get_id())
    {
        content.clone()
    }
    else if let Some(content) = get_built_in(id.get_id()){
        content.clone()
    }
    else {
        Object::Error(format!("identifier not found: {}", id.get_id()))
    }
}

fn eval_prefix_expr(tok:&Token, right: Object) -> Object
{
    match tok {
        Token::BANG =>
            {
                eval_bang_operator(right)
            },
        Token::MINUS =>
            {
                eval_minus_operator(right)
            },
        _ => {
                Object::Error(format!("unknown operator: {}", tok.get_type()))
            }
    }
}

fn eval_if_expr(if_struct: IfStruct, env: & mut Environment) -> Object
{
    let condition = eval(Node::Expression(if_struct.condition.as_ref().clone()), env);
    if condition.is_error() {
        return condition;
    }
    if is_true(condition)
    {
        eval(Node::StatementBlock(if_struct.consequence), env)
    }
    else
    {
        if let Some(alternative) = if_struct.alternative {
            eval(Node::StatementBlock(alternative), env)
        }
        else
        {
            Null
        }
    }
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