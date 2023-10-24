#[allow(dead_code)]
use crate::ast::Node;
use crate::environment::Environment;
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::object::Object::IntegerObject;
use crate::parser::Parser;

#[test]
fn test_eval_integer_expr() {
    let inputs = vec![
        ("5",5 ),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("2 * (5 + 10)", 30),
        ("50 / 2 * 2 + 10", 60)
    ];

    for input in inputs {
        let evaluated = test_eval(input.0.to_string()).expect("Error evaluating integer expression");
        if let Object::IntegerObject(content) = evaluated {
            assert_eq!(content, input.1, "{}", input.0);
        }
        else {
            panic!("wrong type of object");
        }
    }
}

fn test_eval(input: String) -> Option<Object> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut env = Environment::new();
    Some(eval(Node::Program(program), & mut env))
}

#[test]
fn test_boolean_expr() {
    let inputs = vec![
        ("true", true),
        ("false", false),
        ("1 == 2", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("true != true", false),
        ("(1 < 2) == true", true),
        ("(1 > 2) == false", true)
    ];

    for input in inputs {
        let evaluated = test_eval(input.0.to_string()).expect("Error evaluating bool expression");
        if let Object::BooleanObject(content) = evaluated {
            assert_eq!(content, input.1, "{}", input.0);
        }
        else
        {
            panic!("wrong type of object");
        }
    }
}

#[test]
fn test_bang_operator() {
    let inputs = vec!
    [ ("!true", false),
      ("!false", true),
      ("!5", false),
      ("!!true", true),
      ("!!false", false),
      ("!!5", true)
    ];

    for input in inputs {
        let evaluated = test_eval(input.0.to_string()).expect("Error evaluating boolean expression using bang operator");
        if let Object::BooleanObject(content) = evaluated {
            assert_eq!(content,input.1, "{}", input.0);
        }
        else {
            panic!("expected boolean object but didn't match");
        }
    }
}

#[test]
fn test_if_else_expr() {
    let inputs = vec![
        ("if (true) { 10 }", Object::IntegerObject(10)),
        ("if (false) { 10 }", Object::Null),
        ("if (1) { 10 }", Object::IntegerObject(10)),
        ("if (1 < 2) { 10 }", Object::IntegerObject(10)),
        ("if (1 > 2) { 10 }", Object::Null),
        ("if (1 > 2) { 10 } else { 20 }", Object::IntegerObject(20)),
        ("if (1 < 2) { 10 } else { 20 }", Object::IntegerObject(10))];

        for input in inputs {
            assert_eq!(test_eval(input.0.to_string()), Some(input.1));
        }
}

#[test]
fn test_return_expression() {
    let inputs = vec![
        ("return 10;", Object::IntegerObject(10)),
        ("return 10; 9;", Object::IntegerObject(10)),
        ("return 2 * 5; 9;", Object::IntegerObject(10)),
        ("9; return 2 * 5; 9;", Object::IntegerObject(10)),
        ("if(10 > 1) \
        { if(10 > 1) \
        { \
        return 10; \
        } \
        return 1; \
        }", Object::IntegerObject(10))
    ];
    for input in inputs {
        assert_eq!(test_eval(input.0.to_string()), Some(input.1), "{}", input.0);
    }
}

#[test]
fn test_errors() {
    let inputs = vec![
        ("5 + true;", "ERROR: type mismatch: INTEGER PLUS BOOLEAN"),
        ("5 + true; 5;", "ERROR: type mismatch: INTEGER PLUS BOOLEAN"),
        ("-true", "ERROR: unknown operator: MINUS BOOLEAN"),
        ("true + false;", "ERROR: unknown operator: BOOLEAN PLUS BOOLEAN"),
        ("5; true + false; 5;", "ERROR: unknown operator: BOOLEAN PLUS BOOLEAN"),
        ("if(10 > 1){true + false;}", "ERROR: unknown operator: BOOLEAN PLUS BOOLEAN"),
        ("foobar", "ERROR: identifier not found: foobar")
    ];

    for input in inputs {
        let eval = test_eval(input.0.to_string()).expect(format!("Error evaluating {}", input.0).as_str());
        assert_eq!(eval.inspect(), input.1, "testing \"{}\"", input.0);
    }
}

#[test]
fn test_let_statements() {
    let inputs = vec! [
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15)
    ];

    for input in inputs {
        let result = test_eval(input.0.to_string()).expect("Expected some(integerObject) found None");
        if let Object::IntegerObject(content) = result {
            assert_eq!(content, input.1);
        }
        else {
            panic!("Expected IntegerObject");
        }
    }
}

#[test]
fn test_function_object() {
    let input = "fn(x) { x + 2 };";
    let eval = test_eval(input.to_string()).expect("Error parsing input");
    if let Object::Function(content) = eval {
        assert_eq!(content.parameters.len(), 1, "parameteres length");
        assert_eq!(content.parameters[0].get_id(), "x");
        assert_eq!(content.body.len(), 1, "body length");
        assert_eq!(content.body[0].to_string(), "(x + 2);");
    }
    else {
        panic!("Expected function object but found otherwise");
    }
}

#[test]
fn test_function_application() {
    let inputs = vec![
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y ; }; add(5,5);", 10),
        ("let add = fn(x, y) { x + y ; }; add(5 + 5, add(5,5))", 20),
        ("let fn(x) { x; }(5)", 5)
    ];

    for input in inputs {
        let eval = test_eval(input.0.to_string()).expect("couldn't parse function");
        if let Object::IntegerObject(content) = eval {
            assert_eq!(content, input.1, "Error parsing: \"{}\"", input.0);
        }
        else {
            panic!("Expected integer object but found otherwise");
        }
    }

}

#[test]
fn test_len_function() {
    let inputs = vec![
        ("len(\"hola\")",IntegerObject(4)),
        ("len(\"\")", IntegerObject(0)),
        ("len(1)", Object::Error("not suported type: INTEGER".to_string())),
        ("len(\"one\", \"two\")", Object::Error("wrong number of arguments: got = 2, want = 1".to_string()))
    ];

    for input in inputs {
        assert_eq!(test_eval(input.0.to_string()).expect("Couldn't eval"), input.1);
    }
}