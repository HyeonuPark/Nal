pub use codebuf::Node;

/// This type represent sequence of elements
/// where parser fails are isolated to its containing line.
///
/// `Ok(Node<T>)` represents parsed line and
/// `Err(Node<()>)` represents parse failed
pub type Block<T> = Node<Vec<Result<Node<T>, Node<()>>>>;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Ident;
