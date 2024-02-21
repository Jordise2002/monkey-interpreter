use strum_macros::IntoStaticStr;
use crate::ast::{Identifier, Statement};
use crate::environment::Environment;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::code::Instructions;

#[derive(Debug, PartialEq, Clone,IntoStaticStr)]
pub enum Object {
    IntegerObject(i64),
    BooleanObject(bool),
    StringObject(String),
    ReturnValue(Box<Object>),
    Error(String),
    Function(FunctionStruct),
    CompiledFunction(CompiledFunctionStruct),
    BuiltIn(BuiltInFn),
    Array(Vec<Box<Object>>),
    HashMap(HashMap<Object, Object>),
    Closure(ClosureStruct),
    Null
}

impl Eq for Object {}

impl Hash for Object
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self
        {
            Object::IntegerObject(content) => {
                content.hash(state)
            },
            Object::BooleanObject(content) => {
                content.hash(state)
            },
            Object::StringObject(content) => {
                content.hash(state)
            }
            _ => {
                "".hash(state)
            }
        }
    }
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
            },
            Object::HashMap(_) => {
                "HashMap".to_string()
            },
            Object::CompiledFunction(content) => {
                content.inspect()
            },
            Object::Closure(content) =>  {
                content.inspect()
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
            Object::HashMap(_) => {
                "HASH MAP"
            },
            Object::CompiledFunction(_) =>
                {
                    "COMPILED FUNCTION"
                }
            Object::Closure(_) => {
                "CLOSURE"
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

    pub fn is_hashable(&self) -> bool {
        match self {
            Object::IntegerObject(_) => true,
            Object::BooleanObject(_) => true,
            Object::StringObject(_) => true,
            _ => false
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

#[derive(Debug, PartialEq, Clone)]
pub struct CompiledFunctionStruct {
    pub instructions: Instructions,
    pub num_vars: usize,
    pub num_args: usize
}

impl CompiledFunctionStruct
{
    fn inspect(&self) -> String
    {
        format!("CompiledFunction[{}]", self.instructions.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosureStruct {
    pub function: CompiledFunctionStruct,
    pub free_vars: Vec<Box<Object>>
}

impl ClosureStruct {
    fn inspect(&self) -> String {
        format!("Closure[{}]", self.function.inspect())
    }
}
