
use bevy::prelude::*;

use crate::{AppState, cards::PlayedCards, player::{Cards, Dealer, PPosition, Participant, Player, WonTricks}};


pub fn game_plugin(app : &mut App){
    app
        .init_state::<GameState>()
        .init_resource::<Dealer>()
        .add_systems(OnEnter(GameState::StartRound), start_round)
        .add_systems(OnEnter(AppState::Game), setup_participants);
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum GameState {
    #[default]
    StartRound,
    StartTrick,
    EndTrick,
    EndRound,
}

fn start_round(
    mut dealer : ResMut<Dealer>,
    played_cards : ResMut<PlayedCards>, 
    participants : Query<(&Cards, &PPosition), With<Participant>>,
    mut next_game_state : ResMut<NextState<GameState>>,
){
    // rotate dealer
    dealer.0 = match dealer.0 {
        PPosition::West => PPosition::North,
        PPosition::North => PPosition::East,
        PPosition::East => PPosition::South,
        PPosition::South => PPosition::West,
    };
    
    // distribute cards

    // finish setup
    next_game_state.set(GameState::StartTrick);
}

fn start_trick(){

}

fn setup_participants(mut commands : Commands){
    commands.spawn((
        Player,
        Participant,
        PPosition::South,
        Cards::default(),
        WonTricks::default(),
    ));
    commands.spawn((
        Participant,
        PPosition::West,
        Cards::default(),
        WonTricks::default(),
    ));
    commands.spawn((
        Participant,
        PPosition::North,
        Cards::default(),
        WonTricks::default(),
    ));
    commands.spawn((
        Participant,
        PPosition::East,
        Cards::default(),
        WonTricks::default(),
    ));
}
