use strum_macros::IntoStaticStr;
use crate::ast::{Identifier, Statement};
use crate::environment::Environment;

#[derive(Debug, PartialEq, Clone,IntoStaticStr)]
pub enum Object {
    IntegerObject(i64),
    BooleanObject(bool),
    StringObject(String),
    ReturnValue(Box<Object>),
    Error(String),
    Function(FunctionStruct),
    BuiltIn(BuiltInFn),
    Array(Vec<Box<Object>>),
    Null
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::IntegerObject(content) => {
                content.to_string()
            },
            Object::StringObject(content) => {
                content.clone()
            }
            Object::BooleanObject(content) => {
                content.to_string()
            },
            Object::Array(content) => {
                let mut result = "[".to_string();
                result = result + content.clone().into_iter().map(| arg | arg.as_ref().inspect()).collect::<Vec<String>>().join(",").as_str() + "]";
                result
            }
            Object::ReturnValue(content) => {
                content.inspect()
            }
            Object::Null => {
                "Null".to_string()
            },
            Object::Error(content) => {
                "ERROR: ".to_string() + content
            },
            Object::Function(content) => {
                content.inspect()
            },
            Object::BuiltIn(_) => {
                "Built In".to_string()
            }
        }
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Object::IntegerObject(_) => {
                "INTEGER"
            },
            Object::StringObject(_) => {
                "STRING"
            },
            Object::Array(_) => {
                "ARRAY"
            }
            Object::BooleanObject(_) => {
                "BOOLEAN"
            },
            Object::ReturnValue(_) => {
                "RETURN TYPE"
            },
            Object::Error(_) => {
                "ERROR TYPE"
            },
            Object::Null => {
                "NULL TYPE"
            }
            Object::Function(_) => {
                "FUNCTION TYPE"
            }
            Object::BuiltIn(_) => {
                "BUILT IN FUNCTION"
            }
        }
    }

    pub fn is_error(&self) -> bool {
        if let Object::Error(_) = self {
            true
        }
        else {
            false
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionStruct {
    pub parameters: Vec<Identifier>,
    pub body: Vec<Statement>,
    pub env: Environment
}

impl FunctionStruct {

    pub fn new(parameters: Vec<Identifier>, body: Vec<Statement>, env: Environment) -> Self {
        FunctionStruct {
            parameters,
            body,
            env
        }
    }

    pub fn inspect(&self) -> String {
        let mut result = String::from("fn(");
        let x = self.parameters.clone().into_iter().map(|param| param.get_id()).collect::<Vec<String>>();
        result = result + x.join(",").as_str();
        result = result + "){";
        for stmt in &self.body {
            result = result + stmt.to_string().as_str();
        }
        result = result + "}";
        result
    }
}

pub type BuiltInFn = fn(Vec<Object>) -> Object;




