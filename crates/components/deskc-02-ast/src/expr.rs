pub use ids::LinkName;
use uuid::Uuid;

use crate::{
    span::Spanned,
    ty::{CommentPosition, Type},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Int(i64),
    Rational(i64, i64),
    Float(f64),
}

// Literal::Float should not be NaN
impl Eq for Literal {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handler {
    pub input: Spanned<Type>,
    pub output: Spanned<Type>,
    pub handler: Spanned<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(Literal),
    Let {
        ty: Spanned<Type>,
        definition: Box<Spanned<Self>>,
        body: Box<Spanned<Self>>,
    },
    Perform {
        input: Box<Spanned<Self>>,
        output: Spanned<Type>,
    },
    Continue {
        input: Box<Spanned<Self>>,
        output: Option<Spanned<Type>>,
    },
    Handle {
        expr: Box<Spanned<Self>>,
        handlers: Vec<Handler>,
    },
    Apply {
        function: Spanned<Type>,
        link_name: LinkName,
        arguments: Vec<Spanned<Self>>,
    },
    Product(Vec<Spanned<Self>>),
    Match {
        of: Box<Spanned<Self>>,
        cases: Vec<MatchCase>,
    },
    Typed {
        ty: Spanned<Type>,
        item: Box<Spanned<Self>>,
    },
    Hole,
    Function {
        parameters: Vec<Spanned<Type>>,
        body: Box<Spanned<Self>>,
    },
    Vector(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
    Import {
        ty: Spanned<Type>,
        uuid: Option<Uuid>,
    },
    Export {
        ty: Spanned<Type>,
    },
    Attribute {
        attr: Box<Spanned<Self>>,
        item: Box<Spanned<Self>>,
    },
    Brand {
        brands: Vec<String>,
        item: Box<Spanned<Self>>,
    },
    Label {
        label: String,
        item: Box<Spanned<Self>>,
    },
    NewType {
        ident: String,
        ty: Spanned<Type>,
        expr: Box<Spanned<Self>>,
    },
    Comment {
        position: CommentPosition,
        text: String,
        item: Box<Spanned<Self>>,
    },
    Card {
        uuid: Uuid,
        item: Box<Spanned<Self>>,
        next: Option<Box<Spanned<Self>>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub ty: Spanned<Type>,
    pub expr: Spanned<Expr>,
}