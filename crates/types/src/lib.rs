use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Card {
    ExplodingKitten,
    Defuse,
    Attack,
    TargetedAttack,
    DrawFromTheBottom,
    Favor,
    // Nope,
    Shuffle,
    Skip,
    Reverse,
    SeeTheFuture,
    AlterTheFuture,
    TacoCat,
    WatermelonCat,
    PotatoCat,
    BeardCat,
    RainbowCat,
    FeralCat,
}

impl Card {
    pub fn all_cards() -> impl Iterator<Item = Self> {
        std::array::IntoIter::new([
            Self::ExplodingKitten,
            Self::Defuse,
            Self::Attack,
            Self::TargetedAttack,
            Self::DrawFromTheBottom,
            Self::Favor,
            // Self::Nope,
            Self::Shuffle,
            Self::Skip,
            Self::Reverse,
            Self::SeeTheFuture,
            Self::AlterTheFuture,
            Self::TacoCat,
            Self::WatermelonCat,
            Self::PotatoCat,
            Self::BeardCat,
            Self::RainbowCat,
            Self::FeralCat,
        ])
    }

    pub fn amount_in_deck(self) -> usize {
        match self {
            Self::ExplodingKitten => 5,
            Self::Defuse => 6,
            Self::Attack => 4,
            Self::TargetedAttack => 3,
            Self::DrawFromTheBottom => 4,
            Self::Favor => 4,
            // Self::Nope => 5,
            Self::Shuffle => 4,
            Self::Skip => 4,
            Self::Reverse => 4,
            Self::SeeTheFuture => 5,
            Self::AlterTheFuture => 4,
            Self::TacoCat => 4,
            Self::WatermelonCat => 4,
            Self::PotatoCat => 4,
            Self::BeardCat => 4,
            Self::RainbowCat => 4,
            Self::FeralCat => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Event {
    Draw { bottom: bool },
    Explode,
    Play { card: Card },
    // Nope { player: Player },
    BeginTurn { player: Player },
    EndTurn,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Action {
    Draw,
    Play { cards: Vec<Card> },
    Attack { target: Player },
    Steal { target: Player },
    Take { target: Player, card: Card },
    DiscardTake { card: Card },
    AlterTheFuture { cards: [Card; 3] },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InitialState {
    pub players: Vec<Player>,
}
