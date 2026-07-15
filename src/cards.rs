use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::AppState;


pub fn cards_plugin(app : &mut App) {
    app
        .init_resource::<PlayedCards>()
        .init_resource::<CurrentTrick>()
        .add_systems(OnEnter(AppState::Game), setup_cards);
}

#[derive(Resource, Default)]
pub struct PlayedCards(pub Vec<Card>);

#[derive(Resource, Default)]
pub struct CurrentTrick(pub Trick);

#[derive(Clone)]
pub struct Card {
    value : u8,
    suit : Suit,
}

#[derive(PartialEq, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Default)]
pub struct Trick {
    cards : Vec<Card>
}
impl Trick {
    /// Returns Some(()) when there's place, None when the 4th card is played
    pub fn add(&mut self, card: Card) -> Option<()> {
        self.cards.push(card);
        if self.cards.len() <= 3 {
            return Some(());
        }
        None
    }

    /// Returns index of the winning card, 0-3.
    /// None if there's an incorrect amount of cards played
    pub fn winner(&self, trump : Suit) -> Option<usize> {
        if self.cards.len() != 4 {
            return None;
        }
        let vals = self.cards.iter().map(|c| {
            if c.suit == trump { (c.value + 13) as usize } else { c.value as usize }
        });
        vals.max()
    }
}



fn setup_cards(mut played_cards : ResMut<PlayedCards>) {
    for v in 2..=14 {
        let h = Card {value : v, suit: Suit::Hearts };
        let d = Card {value : v, suit: Suit::Diamonds };
        let c = Card {value : v, suit: Suit::Clubs};
        let s = Card {value : v, suit: Suit::Spades };
        played_cards.0.push(h);
        played_cards.0.push(d);
        played_cards.0.push(c);
        played_cards.0.push(s);
    }
    played_cards.0.shuffle(&mut rand::rng());
}



