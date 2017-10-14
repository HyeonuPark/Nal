use ast::common::Span;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    /// This variable is not declared in this scope
    ///
    /// VarNotDecl(reference)
    VarNotDecl(Span),
    /// This variable is not mutable
    ///
    /// VarNotMut(reference, declaration)
    VarNotMut(Span, Span),
    /// `mut` keyword in assign pattern is not allowed
    ///
    /// AssignMutPattern(mut pattern)
    AssignMutPattern(Span),
    /// Can't use reserved keywords as identifier
    ///
    /// IdentIsKeyword(identifier)
    IdentIsKeyword(Span),
}
