use expr::Expr;
use ids::CardId;
use meta::WithMeta;

pub mod expr;
pub mod helper;
pub mod meta;
pub mod ty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cards {
    pub cards: Vec<Card>,
    pub file: WithMeta<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: CardId,
    pub hir: WithMeta<Expr>,
}
