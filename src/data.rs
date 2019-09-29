pub use linked_hash_map::LinkedHashMap;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    start: Position,
    end: Position,
    value: T,
}

impl<T> Spanned<T> {
    pub fn new(value: T, start: Position, end: Position) -> Self {
        if end < start {
            panic!("end position cannot come before start position");
        }
        Self {
            start: start,
            end: end,
            value: value,
        }
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }

    pub fn span(&self) -> (Position, Position) {
        (self.start, self.end)
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Spanned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> AsRef<T> for Spanned<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for Spanned<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> std::borrow::Borrow<T> for Spanned<T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T> std::borrow::BorrowMut<T> for Spanned<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<SpannedValue>),
    Map(LinkedHashMap<Spanned<String>, SpannedValue>),
}

pub type SpannedValue = Spanned<Value>;
