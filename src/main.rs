use bevy::{prelude::*, ui::Interaction};
use bevy_mod_picking::*;
use board::{Board, BoardRenderData, Tile};

mod board;
mod fps_counter;
mod hex;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::UiPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(fps_counter::FpsCounter)
        .add_plugins(DefaultPickingPlugins)
        .add_event::<GameStateEvent>()
        .insert_resource(SelectionState { current: None })
        .add_startup_system(setup)
        .add_system(process_game)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert_bundle(PickingCameraBundle::default());
}

struct SelectionState {
    current: Option<usize>,
}

pub enum GameStateEvent {
    FinishTurn,
}

fn process_game(
    mut events: EventReader<PickingEvent>,
    tile_entitys: Query<(&Tile, &Interaction, Entity)>,
    mut selection_state: ResMut<SelectionState>,
    mut board: ResMut<Board>,
    mut game_state_events: EventReader<GameStateEvent>,
    mut board_render_data: ResMut<BoardRenderData>,
) {
    for event in events.iter() {
        if let PickingEvent::Clicked(e) = event {
            for (tile, _, entity) in tile_entitys.iter() {
                if entity == *e {
                    match selection_state.current {
                        None => {
                            if board.owner(tile.index) == board.current_player() {
                                selection_state.current = Some(tile.index);
                                board_render_data.selected = Some(tile.index);
                            }
                        }
                        Some(first) => {
                            if first == tile.index {
                                selection_state.current = None;
                                board_render_data.selected = None;
                                continue;
                            }

                            let second = tile.index;
                            if board.available_moves(first).contains(&second) {
                                board.make_move(first, second);

                                selection_state.current = None;
                                board_render_data.selected = None;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut changed = false;
    for (tile, interaction, _) in tile_entitys.iter() {
        if let Interaction::Hovered = interaction {
            board_render_data.hovered = Some(tile.index);
            changed = true;
            break;
        }
    }
    if !changed {
        board_render_data.hovered = None;
    }

    for game_state_event in game_state_events.iter() {
        match game_state_event {
            GameStateEvent::FinishTurn => {
                board.finish_turn();
            }
        }
    }
}
