pub use codebuf::Span;

/// This type represent sequence of elements
/// where parser fails are isolated to its containing line.
///
/// `Ok(Span<T>)` represents parsed line and
/// `Err(Span<()>)` represents parse failed
pub type Block<T> = Span<Vec<Result<Span<T>, Span<()>>>>;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Ident;
