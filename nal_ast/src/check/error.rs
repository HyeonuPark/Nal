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
    /// Function statement and object method should always be named
    ///
    /// FuncNotNamed(func stmt)
    FuncNotNamed(Span),
    /// Control expressions should live inside their matching context
    /// For example, "return" can only be used inside function body
    /// as well as "break" and "continue" for while/for-in body
    ///
    /// ContextNotFound(control expr)
    ContextNotFound(Span),
    /// Object literal should not have multiple properties with same name
    ///
    /// DupedPropName(elem1, elem2)
    DupedPropName(Span, Span),
}
