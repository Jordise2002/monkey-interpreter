use crate::object::Object;


pub fn get_built_in(id: String) -> Option<Object> {
    match id.as_str() {
        "len" =>
            {
                Some(Object::BuiltIn(len))
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
