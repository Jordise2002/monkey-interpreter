use crate::token::Token;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    //Statement(Statement),
    Program(Program),
    Expression(Expression),
    StatementBlock(Vec<Statement>)
}

#[derive(PartialEq, Debug, Clone)]
pub struct Program{
    pub statements: Vec<Statement>
}

#[allow(dead_code)]
impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new()
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for statement in &self.statements {
            result += statement.to_string().as_str();
        }
        result
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression)
}

impl Statement{
    pub fn to_string(&self) -> String {
        match self {
            Statement::LetStatement(id, expr) => {
                let str = "let ".to_string() + id.get_id().as_str() + " = " + expr.to_string().as_str() + ";";
                str
            },
            Statement::ReturnStatement(expr) => {
                let str = "return ".to_string() + expr.to_string().as_str() + ";";
                str
            },
            Statement::ExpressionStatement(expr) => {
                let str = expr.to_string() + ";";
                str
            }
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    IdentifierExpression(Identifier),
    IntegerExpression(i64),
    PrefixExpression(Token, Box<Expression>),
    InfixExpression(Box<Expression>, Token, Box<Expression>),
    BoolExpression(bool),
    IfExpression(IfStruct),
    FnExpression(FnStruct),
    CallExpression(CallStruct),
    StringExpression(String),
    ArrayLiteral(ArrayStruct),
    IndexExpression(IndexStruct),
    None
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::IdentifierExpression(id) => {
                id.get_id()
            },
            Expression::IntegerExpression(content) => {
                content.to_string()
            },
            Expression::StringExpression(content) => {
                content.clone()
            }
            Expression::BoolExpression(content) => {
                content.to_string()
            }
            Expression::PrefixExpression(tok, content) => {
                tok.get_type().to_string() + content.to_string().as_str()
            }
            Expression::InfixExpression(left, tok, right) => {
                "(".to_string() + left.to_string().as_str() + " " + tok.get_type() + " " + right.to_string().as_str() + ")"
            },
            Expression::IfExpression(content) => {
                content.to_string()
            },
            Expression::FnExpression(content) => {
                content.to_string()
            },
            Expression::CallExpression(content) => {
                content.to_string()
            },
            Expression::ArrayLiteral(content) => {
                content.to_string()
            },
            Expression::IndexExpression(content) => {
                content.to_string()
            }
            Expression::None => {
                String::from("None")
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Identifier {
    pub id: String
}

impl Identifier {
    pub fn new(id: String) -> Self {
        Identifier {
            id
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IfStruct {
    pub condition: Box<Expression>,
    pub consequence: Vec<Statement>,
    pub alternative: Option<Vec<Statement>>
}

impl IfStruct {
    pub fn to_string(&self) -> String {
        let mut result = String::from("If ");
        result = result + self.condition.to_string().as_str() + "{";
        for stmt in &self.consequence {
           result = result + stmt.to_string().as_str();
        }
        result = result + "}";
        if let Some(content) = &self.alternative {
            result = result + "{";
            for stmt in content {
                result = result + stmt.to_string().as_str();
            }
            result = result + "}";
        }
        result
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FnStruct {
    pub params: Vec<Identifier>,
    pub body: Vec<Statement>
}

impl FnStruct {
    pub fn to_string(&self) -> String{
        let mut result = String::from("fn(");
        let x = self.params.clone().into_iter().map(|param| param.get_id()).collect::<Vec<String>>();
        result = result + x.join(",").as_str();
        result = result + "){";
        for stmt in &self.body {
            result = result + stmt.to_string().as_str();
        }
        result = result + "}";
        result
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CallStruct {
    pub function: Box<Expression>,
    pub args: Vec<Expression>
}

impl CallStruct {
    pub fn to_string(&self) -> String {
        let mut result = self.function.to_string();
        let arg_list = self.args.clone().into_iter().map(|arg | arg.to_string()).collect::<Vec<String>>().join(",");
        result = result + "(" + arg_list.as_str() + ")";
        result
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayStruct {
    pub elements: Vec<Expression>
}

impl ArrayStruct {
    pub fn to_string(&self) -> String {
        let mut result = "[".to_string();
        let element_list = self.elements.clone().into_iter().map(| arg | arg.to_string()).collect::<Vec<String>>().join(",");
        result = result + element_list.as_str() + "]";
        result
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexStruct {
    pub left: Box<Expression>,
    pub index: Box<Expression>
}

impl IndexStruct {
    pub fn to_string(&self) -> String {
        let mut result = self.left.to_string();
        result = result + "[" + self.index.to_string().as_str() + "]";
        result
    }
}