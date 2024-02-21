use std::collections::HashMap;
use crate::symbol_table::SymbolScope::Global;

#[derive(PartialEq, Clone, Debug)]
pub enum SymbolScope {
    Global,
    Local,
    BuiltIn
}

#[derive(PartialEq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize
}

#[derive(PartialEq, Clone, Debug)]
pub struct SymbolTable {
    pub outer: Option<Box<SymbolTable>>,
    store: HashMap<String, Symbol>,
    pub num_definitions: usize
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            outer: None,
            store: HashMap::new(),
            num_definitions: 0
        }
    }

    pub fn new_enclosed(outer: SymbolTable) -> Self {
        SymbolTable {
            outer: Some(Box::new(outer)),
            store: HashMap::new(),
            num_definitions: 0
        }
    }

    pub fn define(&mut self, name: String) -> Symbol
    {
        let scope = 
        if let Some(_) = self.outer {
            SymbolScope::Local
        } else {
            Global
        };
        let symbol = Symbol{name: name.clone(), scope: scope, index: self.num_definitions};
        self.store.insert(name, symbol.clone());
        self.num_definitions += 1;
        symbol
    }

    pub fn define_builtin(&mut self, index:usize, name: String) -> Symbol {
        let symbol = Symbol {
            name: name.clone(),
            scope: SymbolScope::BuiltIn,
            index
        };

        self.store.insert(name, symbol.clone());

        symbol

    }
    pub fn resolve(&self, name: String) -> Option<Symbol>
    {
        match self.store.get(name.as_str())
        {
            Some(content) => {
                Some(content.clone())
            },
            None => {
                if let Some(content) = &self.outer {
                    content.resolve(name)
                }
                else {
                    None
                }
                
            }
        }
    }
}