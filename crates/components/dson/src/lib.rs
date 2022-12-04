use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Float(pub f64);

// Float must not be NaN or infinity.
impl Eq for Float {}

impl std::hash::Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_be_bytes().hash(state);
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Integer(i64),
    Rational(i64, i64),
    Float(Float),
    Hole,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Dson {
    Literal(Literal),
    Product(Vec<Self>),
    Vector(Vec<Self>),
    Map(Vec<MapElem>),
    Attributed { attr: Box<Self>, expr: Box<Self> },
    Labeled { label: Box<Self>, expr: Box<Self> },
    Typed { ty: Type, expr: Box<Self> },
    Comment { text: String, expr: Box<Self> },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MapElem {
    pub key: Dson,
    pub value: Dson,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Type {
    Brand {
        brand: Box<Dson>,
        item: Box<Self>,
    },
    Number,
    String,
    Product(Vec<Self>),
    Sum(Vec<Self>),
    Vector(Box<Self>),
    Map {
        key: Box<Self>,
        value: Box<Self>,
    },
    Attributed {
        attr: Box<Dson>,
        ty: Box<Self>,
    },
    Comment {
        text: String,
        item: Box<Self>,
    },
    Let {
        variable: String,
        definition: Box<Self>,
        body: Box<Self>,
    },
    Variable(String),
}

impl <T: ToString> From<T> for Dson {
    fn from(s: T) -> Self {
        Dson::Literal(Literal::String(s.to_string()))
    }
}
