
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
    mut participants : Query<(Entity, &mut Cards, &PPosition), With<Participant>>,
    mut next_game_state : ResMut<NextState<GameState>>,
){
    dealer.0 = dealer.0.next();

    // cut deck 

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
    // 2 x 4
    let mut i : usize = 0;
    for _ in 0..=1 {
        for entity in &mut players {
            if let Ok(mut p) = participants.get_mut(*entity) {
                for card in 0..=3 {
                    if let Some(c) = played_cards.0.iter().nth(i+card) {
                        p.1.0.push(c.clone());
                    }
                }
                i += 4;
            }
        }
    }
    // 1 x 5

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
