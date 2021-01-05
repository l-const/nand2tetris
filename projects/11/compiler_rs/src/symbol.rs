pub(crate) struct SymbolTable {}

pub(crate) enum IdKind {
    STATIC,
    FIELD,
    ARG,
    NONE,
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        SymbolTable {}
    }

    fn start_subroutine(&mut self) {
        // Starts a new subroutine scope (i.e. erases
        // all names in the previous subroutine’s
        // scope.)
    }

    fn define(&mut self) {
        //   Defines a new identifier of a given name,
        //  type, and kind and assigns it a running
        //  index. STATIC and FIELD identifiers
        //  have a class scope, while ARG and VAR
        //  identifiers have a subroutine scope.
    }

    fn var_count(&self) -> u8 {
        // Returns the number of variables of the
        //given kind already defined in the current
        //scope.
        1
    }

    fn kind_of(&self) -> IdKind {
        // Returns the kind of the named identifier in
        // the current scope. Returns NONE if the
        // identifier is unknown in the current scope.
        //
        IdKind::NONE
    }

    fn type_of(&self) -> &str {
        // Returns the type of the named identifier in
        //the current scope.
        "fdfdfd"
    }

    fn index_of(&self) -> u8 {
        // Returns the index assigned to named
        //identifier.
        1
    }
}
