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
impl PlayedCards {

    pub fn cut_random(&mut self) {
        let pos = rand::random_range(1..self.0.len()) as usize;
        self.cut(pos);
    }

    pub fn cut(&mut self, pos: usize) {
        let mut base = 0usize;
        let mut len = self.0.len();
        let mut k = pos;

        while k != 0 && k != len {
            if k < len - k {
                // rotation amount is smaller: swap first k with last k,
                // then shrink the window from the back
                self.swap_range(base, base + k, base + len - k, base + len);
                len -= k;
            } else {
                // swap start and move base
                let m = len - k;
                self.swap_range(base, base + m, base + m, base + 2 * m);
                base += m;
                len = k;
                k -= m;
            }
        }
    }

    fn swap_range(&mut self, s1: usize, e1: usize, s2: usize, e2: usize) {
        // dbg!(s1,e1,s2,e2);
        debug_assert!(e1 <= s2, "ranges must be non-overlapping and in order");

        let (left, right) = self.0.split_at_mut(s2);
        let a = &mut left[s1..e1];
        let b = &mut right[..e2 - s2];
        a.swap_with_slice(b);
    }
}

#[derive(Resource, Default)]
pub struct CurrentTrick(pub Trick);

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    value : u8,
    suit : Suit,
}

#[derive(PartialEq, Clone, Debug)]
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
    insert_cards(&mut played_cards.0);
}
fn insert_cards(list : &mut Vec<Card>){
    for value in 2..=14 {
        let h = Card {value, suit: Suit::Hearts };
        let d = Card {value, suit: Suit::Diamonds };
        let c = Card {value, suit: Suit::Clubs};
        let s = Card {value, suit: Suit::Spades };
        list.push(h);
        list.push(d);
        list.push(c);
        list.push(s);
    }
    if cfg!(not(test)) {
        list.shuffle(&mut rand::rng());
    }
}



#[test]
fn test_cut(){
    let mut played = PlayedCards::default();
    let mut clone = PlayedCards::default();
    insert_cards(&mut played.0);

    assert!(&played.0.len() == &52usize);

    clone.0 = played.0.clone();
    played.cut(5);
    dbg!(&played.0[45..]);
    dbg!(&clone.0[45..]);

    assert!(played.0.iter().nth(51) == clone.0.iter().nth(4));
    assert!(&played.0.len() == &52usize);

}



