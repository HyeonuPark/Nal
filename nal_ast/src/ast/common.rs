pub use codebuf::Node;

pub type Block<T> = Node<[Node<T>]>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident(Box<str>);

impl ::std::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Bool(bool),
    Num(f64),
    Str(String),
}
