use ggbc_parser::ast::Expression;

#[derive(Debug)]
pub enum Error<'a> {
    /// Error during the program parsing.
    Parser(ggbc_parser::Error<'a>),
    /// Use of a non-const expression where not allowed, such as in the size of
    /// an array type.
    InvalidExpression {
        /// The invalid expression itself.
        expression: Expression<'a>,
        /// The reason why it's invalid.
        reason: Option<&'static str>,
    },
}
