use std::collections::HashMap;
use crate::symbol_table::{Symbol, SymbolScope, SymbolTable};
use crate::symbol_table::SymbolScope::{Global, Local};

#[test]
fn test_define() {
    let expected = HashMap::from(
        [
            ("a".to_string(), Symbol{name:"a".to_string(), scope: Global, index: 0}),
            ("b".to_string(), Symbol{name:"b".to_string(), scope: Global, index: 1})
        ]);

    let mut global = SymbolTable::new();
    let a = global.define("a".to_string());
    assert_eq!(a, expected.get("a").unwrap().clone());
    let b = global.define("b".to_string());
    assert_eq!(b, expected.get("b").unwrap().clone());
}

#[test]
fn test_resolve() {
    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());

    let expected = vec![
        Symbol{name: "a".to_string(), scope: Global, index: 0},
        Symbol{name: "b".to_string(), scope: Global, index: 1}
    ];

    for symbol in expected
    {
        assert_eq!(symbol.clone(), global.resolve(symbol.name).expect("Couldn't find symbol in the table"));
    }
}

#[test]
fn test_resolve_local()
{
    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());
    

    let mut local = SymbolTable::new_enclosed(global);
    local.define("c".to_string());
    local.define("d".to_string());

    let expected = vec![
        Symbol{name: "a".to_string(), scope: Global, index: 0},
        Symbol{name: "b".to_string(), scope: Global, index: 1},
        Symbol{name: "c".to_string(), scope: Local, index: 0},
        Symbol{name: "d".to_string(), scope: Local, index: 1}
    ];

    for symbol in expected {
        assert_eq!(symbol.clone(), local.resolve(symbol.name).unwrap());
    }
}