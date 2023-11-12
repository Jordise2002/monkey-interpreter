use std::collections::HashMap;
use crate::symbol_table::SymbolScope::Global;

#[derive(PartialEq, Clone, Debug)]
pub enum SymbolScope {
    Global
}

#[derive(PartialEq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize
}

#[derive(PartialEq, Clone, Debug)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    num_definitions: usize
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            num_definitions: 0
        }
    }

    pub fn define(&mut self, name: String) -> Symbol
    {
        let symbol = Symbol{name: name.clone(), scope: Global, index: self.num_definitions};
        self.store.insert(name, symbol.clone());
        self.num_definitions += 1;
        symbol
    }

    pub fn resolve(&mut self, name: String) -> Option<Symbol>
    {
        match self.store.get(name.as_str())
        {
            Some(content) => {
                Some(content.clone())
            },
            None => {
                None
            }
        }
    }
}