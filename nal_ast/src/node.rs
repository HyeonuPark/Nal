
#[derive(Debug, Clone)]
pub struct Node<T: ?Sized>(Box<T>, Span);

pub type Block<T> = Node<[Node<T>]>;

impl<T: ?Sized> Node<T> {
    pub fn new<U: Into<Box<T>>>(span: Span, data: U) -> Self {
        Node(data.into(), span)
    }

    pub fn dummy<U: Into<Box<T>>>(data: U) -> Self {
        Self::new(Default::default(), data)
    }

    pub fn val(&self) -> &T {
        &*self.0
    }

    pub fn span(&self) -> Span {
        Span
    }
}

impl<T> Node<T> {
    pub fn into(self) -> T {
        *self.0
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Span;

impl ::std::ops::Add for Span {
    type Output = Self;

    fn add(self, _: Self) -> Self {
        Span
    }
}
