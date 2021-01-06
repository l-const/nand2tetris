use std::collections::HashMap;

pub(crate) struct SymbolTable {
    class_table: HashMap<String, (String, IdKind, u8)>,
    method_table: HashMap<String, (String, IdKind, u8)>,
    counters: Counters,
}

struct Counters {
    stat_count: u8,
    field_count: u8,
    arg_count: u8,
    var_count: u8,
}
#[derive(PartialEq)]
pub(crate) enum IdKind {
    STATIC,
    FIELD,
    ARG,
    VAR,
    NONE,
}

impl Clone for IdKind {
    fn clone(&self) -> Self {
        match *self {
            STATIC => STATIC,
            FIELD => FIELD,
            ARG => ARG,
            VAR => VAR,
            NONE => NONE,
        }
    }
}

impl Copy for IdKind {}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        let counters = Counters {
            stat_count: 0,
            field_count: 0,
            var_count: 0,
            arg_count: 0,
        };
        SymbolTable {
            class_table: HashMap::<String, (String, IdKind, u8)>::new(),
            method_table: HashMap::<String, (String, IdKind, u8)>::new(),
            counters: counters,
        }
    }

    fn start_subroutine(&mut self) {
        // Starts a new subroutine scope (i.e. erases
        // all names in the previous subroutine’s
        // scope.)
        self.method_table.clear();
        self.counters.arg_count = 0;
        self.counters.var_count = 0;
    }

    fn define(&mut self, name: &str, typ: &str) {
        //   Defines a new identifier of a given name,
        //  type, and kind and assigns it a running
        //  index. STATIC and FIELD identifiers
        //  have a class scope, while ARG and VAR
        //  identifiers have a subroutine scope.
    }

    fn var_count(&self, kind: IdKind) -> u8 {
        // Returns the number of variables of the
        //given kind already defined in the current
        //scope.
        match kind {
            IdKind::STATIC | IdKind::FIELD => {
                self.class_table.values().filter(|x| x.1 == kind).count() as u8
            }
            IdKind::ARG | IdKind::VAR => {
                self.method_table.values().filter(|x| x.1 == kind).count() as u8
            }
            IdKind::NONE => 0,
        }
    }

    fn kind_of(&self, name: &str) -> IdKind {
        // Returns the kind of the named identifier in
        // the current scope. Returns NONE if the
        // identifier is unknown in the current scope.
        //
        self.lookup(name).unwrap().1
    }

    fn type_of(&self, name: &str) -> &str {
        // Returns the type of the named identifier in
        //the current scope.
        &self.lookup(name).unwrap().0
    }

    fn index_of(&self, name: &str) -> u8 {
        // Returns the index assigned to named identifier.
        self.lookup(name).unwrap().2
    }

    fn lookup(&self, name: &str) -> Option<&(String, IdKind, u8)> {
        if let Some(t) = self.method_table.get(name) {
            Some(t)
        } else if let Some(t) = self.class_table.get(name) {
            Some(t)
        } else {
            None
        }
    }
}