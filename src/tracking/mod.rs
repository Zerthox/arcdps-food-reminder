pub mod buff;
pub mod player;

use crate::{
    tracking::buff::{Categorize, Food, Utility},
    ui::{color, Component},
};
use arcdps::imgui::{im_str, TableColumnFlags, TableFlags, Ui};
use buff::Buff;
use player::Player;
use std::collections::HashMap;

/// Player tracker.
#[derive(Debug)]
pub struct Tracker {
    /// Currently tracked players mapped by their id.
    players: HashMap<usize, Player>,
}

impl Tracker {
    /// Creates a new tracker.
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }

    /// Removes a tracked player, returning the removed player if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> Option<Player> {
        self.players.remove(&id)
    }

    /// Returns a reference to a tracked player.
    pub fn get_player(&self, id: usize) -> Option<&Player> {
        self.players.get(&id)
    }

    /// Returns a mutable reference to a tracked player.
    pub fn get_player_mut(&mut self, id: usize) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    /// Returns an unsorted iterator over all tracked players.
    pub fn get_players(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }

    /// Returns an unsorted mutable iterator over all tracked players.
    pub fn get_players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.players.values_mut()
    }

    /// Returns all tracked players sorted by subgroup.
    pub fn get_players_by_sub(&self) -> Vec<&Player> {
        let mut players = self.get_players().collect::<Vec<_>>();
        players.sort_by_key(|player| player.subgroup);
        players
    }
}

impl Component for Tracker {
    type Props = ();

    fn create(_props: Self::Props) -> Self {
        Self::new()
    }

    fn render(&mut self, ui: &Ui) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            // create table
            if ui.begin_table_with_flags(
                im_str!("food-reminder-tracker-table"),
                4,
                TableFlags::NONE,
            ) {
                // declare columns
                ui.table_setup_column_with_flags(im_str!("Sub"), TableColumnFlags::DEFAULT_SORT);
                ui.table_setup_column_with_flags(im_str!("Player"), TableColumnFlags::NO_SORT);
                ui.table_setup_column_with_flags(im_str!("Food"), TableColumnFlags::NO_SORT);
                ui.table_setup_column_with_flags(im_str!("Util"), TableColumnFlags::NO_SORT);

                // render header
                ui.table_headers_row();

                // TODO: sorting, imgui-rs currently has no wrapping, need to use imgui::sys directly

                // iterate over tracked players
                for player in self.get_players_by_sub() {
                    // new row for each player
                    ui.table_next_row();

                    // render subgroup cell
                    ui.table_next_column();
                    ui.text(format!("{:>2}", player.subgroup));

                    // render name cell
                    ui.table_next_column();
                    ui.text(&player.character);
                    if ui.is_item_hovered() {
                        ui.tooltip_text(&player.account);
                    }

                    // render food cell
                    ui.table_next_column();
                    match player.food {
                        Buff::Unset => {
                            ui.text("???");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Uncertain");
                            }
                        }
                        Buff::None | Buff::Known(Food::Malnourished) => {
                            ui.text_colored(color::RED.to_rgba_f32s(), "NONE");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("No Food");
                            }
                        }
                        Buff::Unknown => {
                            ui.text_colored(color::YELLOW.to_rgba_f32s(), "SOME");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Unknown Food");
                            }
                        }
                        Buff::Known(food) => {
                            ui.text_colored(color::GREEN.to_rgba_f32s(), food.categorize());
                            if ui.is_item_hovered() {
                                ui.tooltip_text(format!(
                                    "{}\n{}",
                                    food.name(),
                                    food.stats().join("\n")
                                ));
                            }
                        }
                    }

                    // render util cell
                    ui.table_next_column();
                    match player.util {
                        Buff::Unset => {
                            ui.text("???");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Uncertain");
                            }
                        }
                        Buff::None | Buff::Known(Utility::Diminished) => {
                            ui.text_colored(color::RED.to_rgba_f32s(), "NONE");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("No Utility");
                            }
                        }
                        Buff::Unknown => {
                            ui.text_colored(color::YELLOW.to_rgba_f32s(), "SOME");
                            if ui.is_item_hovered() {
                                ui.tooltip_text("Unknown Utility");
                            }
                        }
                        Buff::Known(util) => {
                            ui.text_colored(color::GREEN.to_rgba_f32s(), util.categorize());
                            if ui.is_item_hovered() {
                                ui.tooltip_text(format!(
                                    "{}\n{}",
                                    util.name(),
                                    util.stats().join("\n")
                                ));
                            }
                        }
                    }
                }

                ui.end_table();
            }
        }
    }
}
