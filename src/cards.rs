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
struct PlayedCards(Vec<Card>);



pub struct Card {
    value : u8,
    suit : Suit,
}

pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

/// Identifies the actual player from the bots
#[derive(Component)]
struct Player;

/// welke cards van de player zijn
#[derive(Component)]
struct Cards;


#[derive(Resource, Default)]
struct CurrentTrick(Trick);

#[derive(Component)]
struct WonTricks();

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
}

#[derive(Component)]
struct Participant;

#[derive(Component)]
struct ParticipantIndex(u8);



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



