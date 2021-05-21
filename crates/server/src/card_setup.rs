use nanorand::{WyRand, RNG};
use num::Integer;
use types::Card;

pub(crate) fn set_up_cards(num_players: usize, rng: &mut WyRand) -> (Deck, Vec<Hand>) {
    let mut card_setup = CardSetup::new(num_players, rng);
    let hands = (0..num_players).map(|_| card_setup.deal_hand()).collect();

    (card_setup.finalize_deck(rng), hands)
}

pub(crate) struct Deck {
    pub(crate) cards: Vec<Card>,
}

pub(crate) struct Hand {
    pub(crate) cards: Vec<Card>,
}

impl Hand {
    const NUM_INITIAL_CARDS: usize = 4;
}

struct CardSetup {
    cards: Vec<Card>,
    exploding_kittens: Vec<Card>,
    defuses: Vec<Card>,
    num_players: usize,
}

impl CardSetup {
    fn new(num_players: usize, rng: &mut WyRand) -> Self {
        let num_decks = num_players.div_ceil(&Card::ExplodingKitten.amount_in_deck());
        let mut cards = Vec::new();

        for card in Card::all_cards() {
            if card == Card::ExplodingKitten || card == Card::Defuse {
                continue;
            }

            for _ in 0..card.amount_in_deck() {
                cards.push(card);
            }
        }

        cards = cards.repeat(num_decks);

        rng.shuffle(&mut cards);

        let exploding_kittens = vec![Card::ExplodingKitten; num_players - 1];
        let defuses = vec![Card::Defuse; Card::Defuse.amount_in_deck()];

        Self {
            cards,
            exploding_kittens,
            defuses,
            num_players,
        }
    }

    fn deal_hand(&mut self) -> Hand {
        let mut cards = self
            .cards
            .split_off(self.cards.len() - Hand::NUM_INITIAL_CARDS);

        cards.push(self.defuses.pop().unwrap());

        Hand { cards }
    }

    fn finalize_deck(mut self, rng: &mut WyRand) -> Deck {
        self.cards.append(&mut self.exploding_kittens);
        self.cards.append(&mut self.defuses);

        rng.shuffle(&mut self.cards);

        Deck { cards: self.cards }
    }
}
