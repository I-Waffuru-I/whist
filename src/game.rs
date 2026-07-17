
use bevy::prelude::*;

use crate::{AppState, cards::{Card, PlayedCards}, player::{Cards, Dealer, PPosition, Participant, Player, WonTricks}};


pub fn game_plugin(app : &mut App){
    app
        .init_state::<GameState>()
        .init_resource::<Dealer>()
        .add_systems(OnEnter(GameState::StartRound), start_round)
        .add_systems(OnEnter(AppState::Game), setup_participants);
}

fn setup(mut next_game_state : ResMut<NextState<GameState>>){
    next_game_state.set(GameState::StartRound);
}

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum GameState {
    #[default]
    Idle,
    StartRound,
    StartTrick,
    EndTrick,
    EndRound,
}

fn start_round(
    mut dealer : ResMut<Dealer>,
    mut played_cards : ResMut<PlayedCards>, 
    mut participants : Query<(Entity, &mut Cards, &PPosition), With<Participant>>,
    mut next_game_state : ResMut<NextState<GameState>>,
){
    dealer.0 = dealer.0.next();

    dbg!(&played_cards.0);
    // cut deck 
    played_cards.cut_random();
    dbg!(&played_cards.0);

    // distribute cards
    let mut players : Vec<Entity> = vec!();
    let mut ppos = dealer.0.clone();
    for _ in 0..=3 {
        if let Some(p) = participants.iter().find(|e| e.2 == &ppos) {
            players.push(p.0);
        }
        ppos = ppos.next();
    }

    // distribute cards to players
    let mut i : usize = 0;
    let rounds : [usize;3] = [4, 4, 5];
    for r in rounds {
        for card in 0..=r {
            for entity in &mut players {
                if let Ok(mut p) = participants.get_mut(*entity) {
                    if let Some(c) = played_cards.0.iter().nth(i+card) {
                        p.1.0.push(c.clone());
                    }
                    i += 4;
                }
            }
        }
    }

    // finish setup
    next_game_state.set(GameState::StartTrick);
}

fn start_trick(){

}

fn setup_participants(mut commands : Commands, mut next_game_state : ResMut<NextState<GameState>>){
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

    next_game_state.set(GameState::StartRound);
}


