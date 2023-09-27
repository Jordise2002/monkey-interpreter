use std::collections::HashMap;
use crate::object::Object;


#[derive(PartialEq, Clone, Debug)]
pub struct Environment {
    map: HashMap<String, Object>,
    superior: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            map: HashMap::new(),
            superior:None
        }
    }

    pub fn new_with_superior(env: Box<Environment>) -> Self
    {
        Environment {
            map: HashMap::new(),
            superior: Some(env)
        }
    }

    pub fn get(&self,name: String) -> Option<Object> {
        let result = self.map.get(name.as_str());
        match result {
            Some(content) => {
                Some(content.clone())
            },
            None => {
                if let Some(env) = &self.superior {
                    env.get(name)
                }
                else
                {
                    None
                }
            }
        }
    }

    pub fn set(& mut self, name:String, value:Object) {
        self.map.insert(name, value);
    }
}