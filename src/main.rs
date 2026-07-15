
use bevy::prelude::*;

mod cards;
mod menu;
mod player;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .init_state::<AppState>()
        .add_plugins((menu::menu_plugin, cards::cards_plugin, game::game_plugin))
        .add_systems(Update, manage_app_state)
        .add_systems(OnEnter(AppState::Loading), start_load)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}   


#[derive(States, Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Paused,
    Game,
}


fn manage_app_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state : Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state.get(){
            AppState::Game => next_app_state.set(AppState::Paused),
            AppState::Paused => next_app_state.set(AppState::Game),
            _ => {}
        }
    }
}

fn start_load(mut state : ResMut<NextState<AppState>>){
    state.set(AppState::Menu);
}




