use super::board::Board;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_system);
    }
}

fn ui_system(mut egui_context: ResMut<EguiContext>, mut board: ResMut<Board>) {
    egui::Window::new("Window").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Spread Dice").clicked() {
            for i in 0..board.teritories.len() {
                let mut dice = board.teritories[i].dice;
                for j in 0..board.teritories[i].connections.len() {
                    if dice == 0 {
                        break;
                    }
                    let connection = board.teritories[i].connections[j];
                    board.teritories[connection].dice += 1;
                    dice -= 1;
                }
                board.teritories[i].dice = dice;
            }
        }
    });
}
