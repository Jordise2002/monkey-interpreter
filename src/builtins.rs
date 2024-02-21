use crate::object::{BuiltInFn, Object};
#[allow(dead_code)]

pub const BUILT_INS: [&'static str; 6] = [
        "len",
        "first",
        "rest",
        "last",
        "push",
        "puts"
];
pub fn get_built_in(id: String) -> Option<Object> {
    match id.as_str() {
        "len" =>
            {
                Some(Object::BuiltIn(len))
            },
        "first" =>
            {
                Some(Object::BuiltIn(first))
            },
        "rest" =>
            {
                Some(Object::BuiltIn(rest))
            },
        "last" =>
            {
                Some(Object::BuiltIn(last))
            },
        "push" =>
            {
                Some(Object::BuiltIn(push))
            }
        "puts" =>
            {
                Some(Object::BuiltIn(puts))
            }
        _ => {
            None
        }
    }
}


fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        Object::Error(format!("wrong number of arguments: got = {}, want = 1",args.len()))
    }
    else {
        if let Object::StringObject(content) = &args[0] {
            Object::IntegerObject(content.len() as i64)
        }
        else if let Object::Array(content) = &args[0] {
            Object::IntegerObject(content.len() as i64)
        }
        else {
            Object::Error(format!("not suported type: {}", args[0].get_type()))
        }
    }
}

fn first(args:Vec<Object>) -> Object
{
    if args.len() != 1 {
        Object::Error(format!("wrong number of arguments: got = {}, want = 1",args.len()))
    }
    else
    {
        if let Object::Array(content) = &args[0]
        {
            content[0].as_ref().clone()
        }
        else
        {
            Object::Error(format!("not suported type: {}", args[0].get_type()))
        }
    }
}

fn last(args:Vec<Object>) -> Object
{
    if args.len() != 1 {
        Object::Error(format!("wrong number of arguments: got = {}, want = 1",args.len()))
    }
    else
    {
        if let Object::Array(content) = &args[0]
        {
            content[content.len()-1].as_ref().clone()
        }
        else
        {
            Object::Error(format!("not suported type: {}", args[0].get_type()))
        }
    }
}

fn rest(args:Vec<Object>) -> Object {
    if args.len() != 1 {
        Object::Error(format!("wrong number of arguments: got = {}, want = 1",args.len()))
    }
    else {
        if let Object::Array(content) = &args[0]
        {
            let mut result = Vec::new();
            for i in 1..content.len()
            {
                result.push(content[i].clone());
            }
            Object::Array(result)
        }
        else
        {
            Object::Error(format!("not suported type: {}", args[0].get_type()))
        }
    }
}

fn push(args:Vec<Object>) -> Object
{
    if args.len() != 2
    {
        Object::Error(format!("wrong number of arguments: got = {}, want = 1",args.len()))
    }
    else {
        if let Object::Array(content) = &args[0]
        {
            let mut content = content.clone();
            content.push(Box::new(args[1].clone()));
            Object::Array(content)
        }
        else
        {
            Object::Error(format!("not suported type: {} in position 0, expected ARRAY", args[0].get_type()))
        }
    }
}

fn puts(args:Vec<Object>) -> Object
{
    for arg in args {
        print!("{} ",arg.inspect())
    }
    print!("\n");
    Object::Null
}
