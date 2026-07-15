
use bevy::prelude::*;

use crate::{cards::{Card, Trick}};

#[derive(Component, Default)]
pub enum PPosition {
    #[default]
    North, 
    East,
    South,
    West
}

/// Separates the actual player from the bots
#[derive(Component)]
pub struct Player;

/// tag to query bots & player 
#[derive(Component)]
pub struct Participant;

#[derive(Resource, Default)]
pub struct Dealer(pub PPosition);

/// welke cards van de player zijn
#[derive(Component, Default)]
pub struct Cards(Vec<Card>);

/// the tricks the user won this round
#[derive(Component, Default)]
pub struct WonTricks(Vec<Trick>);

