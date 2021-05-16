use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event {
    Draw { bottom: bool },
    Explode,
    Play { card: Card },
    // Nope { user: User },
    BeginTurn { user: User },
    EndTurn,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    Draw,
    Play { cards: Vec<Card> },
    Attack { target: User },
    Steal { target: User },
    Take { target: User, card: Card },
    DiscardTake { card: Card },
    AlterTheFuture { cards: [Card; 3] },
}