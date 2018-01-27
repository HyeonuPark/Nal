pub use codebuf::Node;

pub type Block<T> = Node<[Node<T>]>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident(pub Box<str>);

impl ::std::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}
