#[derive(Debug)]
pub enum SymbolType {
    Label,
    Integer,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: Option<u32>,
    #[allow(dead_code)]
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            offset: None,
            symbol_type,
        }
    }

    pub fn new_with_offset(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            offset: Some(offset),
            symbol_type,
        }
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn has_symbol(&self, s: &str) -> bool {
        for symbol in &self.symbols {
            if symbol.name == s {
                return true;
            }
        }
        false
    }

    pub fn set_symbol_offset(&mut self, s: &str, offset: u32) -> bool {
        for symbol in &mut self.symbols {
            if symbol.name == s {
                symbol.offset = Some(offset);
                return true;
            }
        }
        false
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return symbol.offset;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new_with_offset("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(true, v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert_eq!(false, v.is_some());
    }
}
