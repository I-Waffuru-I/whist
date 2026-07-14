
use bevy::prelude::*;

use crate::cards::Trick;


pub fn players_plugin(app : &mut App) {


}

/// Separates the actual player from the bots
#[derive(Component)]
struct Player;

/// welke cards van de player zijn
#[derive(Component)]
struct Cards;

/// the tricks the user won this round
#[derive(Component)]
struct WonTricks(Vec<Trick>);

#[derive(Component)]
struct Participant;

#[derive(Component)]
struct ParticipantIndex(u8);
