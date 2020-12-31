struct Parser;

impl Parser {
    fn new(input: String, out: String) -> Parser;
    fn compile_class(&mut self);
    //compiles a complete class.

    fn compile_class_vardec(&mut self);
    //compiles a static declaration or a field declaration.

    fn compile_subroutine(&mut self);
    // compiles a complete method, function,
    // or constructor.

    fn compile_parameter_list(&mut self);
    // compiles a (possibly empty) parameter
    //list, not including the enclosing “()”.
    fn compile_vardec(&mut self);

    fn compile_statements(&mut self);

    fn compile_do(&mut self);

    fn compile_let(&mut self);

    fn compile_while(&mut self);
    //compiles a sequence of statements, not
    // including the enclosing “{}”.
    fn compile_return(&mut self);

    fn compile_if(&mut self);
    // compiles an if statement, possibly
    //with a trailing else clause.
    fn compile_expression(&mut self);

    fn compile_term(&mut self);

    fn compile_expression_list(&mut self);
    // compiles a (possibly empty) comma-
    // separated list of expressions.
}
