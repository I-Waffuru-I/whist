
use bevy::{color::palettes::css, prelude::*};

use crate::AppState;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn menu_plugin(app: &mut App){
    app
        .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
        .add_systems(OnEnter(AppState::Menu), display_menu);
}

#[derive(Component)]
enum MenuActions {
    NewGame,
    Quit,
    QuitConfirm,
    QuitCancel,
}

fn display_menu(
    mut commands : Commands,
){

    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font_size : FontSize::Px(25.0),
        ..default()
    };

    commands.spawn((
        DespawnOnExit(AppState::Menu),
        Node {
            width : percent(100),
            height : percent(100),
            align_items : AlignItems::Center,
            justify_content : JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction : FlexDirection::Column,
                align_items : AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(css::CRIMSON.into()),
            children![
                (
                    Button,
                    button_node.clone(),
                    MenuActions::NewGame,
                    BackgroundColor(css::GRAY.into()),
                    children![
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    MenuActions::Quit,
                    BackgroundColor(css::GRAY.into()),
                    children![
                        (
                            Text::new("Quit"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
            ]
        )]
    ));
}

fn menu_action (
    interactions : Query<(&Interaction, &MenuActions), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<AppState>>,
){
    for (i, a) in interactions {
        if *i == Interaction::Pressed {
            match *a {
                MenuActions::NewGame => { game_state.set(AppState::Game); },
                MenuActions::Quit => { app_exit_writer.write(AppExit::Success); },
                MenuActions::QuitConfirm => {app_exit_writer.write(AppExit::Success);},
                MenuActions::QuitCancel => { },
            }
        }
    }
}



