pub mod buff;
pub mod entry;

use crate::data::Boss;
use arc_util::{
    api::CoreColor,
    exports,
    game::Player,
    settings::HasSettings,
    ui::{components::item_context_menu, Component, WindowProps, Windowed},
};
use arcdps::imgui::{im_str, sys as ig, ImStr, TableColumnFlags, TableFlags, Ui};
use buff::{BuffState, Food, Utility};
use entry::Entry;
use std::{cmp::Reverse, ptr, slice};
use windows::System::VirtualKey;

/// Player tracker.
#[derive(Debug)]
pub struct Tracker {
    /// Currently tracked players.
    players: Vec<Entry>,

    /// Current sorting.
    sorting: Sorting,

    /// Current sorting direction.
    reverse: bool,

    /// Cache for buffs on characters of local player (self).
    self_cache: Vec<Entry>,

    /// Current ongoing encounter.
    encounter: Encounter,
}

#[allow(unused)]
impl Tracker {
    /// Default hotkey for tracker.
    pub const HOTKEY: usize = VirtualKey::F.0 as usize;

    /// Creates a new tracker.
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            sorting: Sorting::Sub,
            reverse: false,
            self_cache: Vec::new(),
            encounter: Encounter::None,
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player) {
        let mut added = Entry::new(player);

        // check for self
        if added.player.is_self {
            // check cache
            self.self_cache
                .iter()
                .position(|entry| entry.player.character == added.player.character)
                .map(|index| {
                    // use cached buffs
                    let removed = self.self_cache.remove(index);
                    added.food = removed.food;
                    added.util = removed.util;
                });
        }

        // insert entry
        self.players.push(added);

        // refresh sorting
        self.refresh_sort();
    }

    /// Removes a tracked player, returning the removed entry if they were tracked.
    pub fn remove_player(&mut self, id: usize) -> Option<Entry> {
        self.players
            .iter()
            .position(|entry| entry.player.id == id)
            .map(|index| {
                // remove entry, sorting will be preserved
                let removed = self.players.remove(index);

                // check for self
                if removed.player.is_self {
                    // cache own character buffs in case we play it again later
                    self.self_cache.push(removed.clone());
                }

                // return removed entry
                removed
            })
    }

    /// Returns a reference to the local player (self).
    pub fn get_self(&self) -> Option<&Entry> {
        self.players.iter().find(|entry| entry.player.is_self)
    }

    /// Returns a mutable reference to the local player (self).
    pub fn get_self_mut(&mut self) -> Option<&mut Entry> {
        self.players.iter_mut().find(|entry| entry.player.is_self)
    }

    /// Returns a reference to a tracked player entry.
    pub fn player(&self, id: usize) -> Option<&Entry> {
        self.players.iter().find(|entry| entry.player.id == id)
    }

    /// Returns a mutable reference to a tracked player entry.
    pub fn player_mut(&mut self, id: usize) -> Option<&mut Entry> {
        self.players.iter_mut().find(|entry| entry.player.id == id)
    }

    /// Returns an iterator over all tracked player entries.
    pub fn all_players(&self) -> impl Iterator<Item = &Entry> {
        self.players.iter()
    }

    /// Returns a mutable iterator over all tracked player entries.
    pub fn all_players_mut(&mut self) -> impl Iterator<Item = &mut Entry> {
        self.players.iter_mut()
    }

    /// Returns the number of tracked players.
    pub fn len(&self) -> usize {
        self.players.len()
    }

    /// Returns `true` if there is no tracked players.
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    /// Starts a boss encounter.
    pub fn start_encounter(&mut self, boss: Option<Boss>) {
        self.encounter = boss.map(Encounter::Boss).unwrap_or(Encounter::Unknown);
    }

    /// Ends the current boss encounter.
    pub fn end_encounter(&mut self) {
        self.encounter = Encounter::None;
    }

    /// Returns the encounter state.
    pub fn encounter(&self) -> Encounter {
        self.encounter
    }

    /// Returns `true` if there is an ongoing boss encounter.
    pub fn in_encounter(&self) -> bool {
        self.encounter != Encounter::None
    }

    /// Sorts the players in the tracker table.
    fn refresh_sort(&mut self) {
        match (self.sorting, self.reverse) {
            (Sorting::Sub, false) => self.players.sort_by_key(|entry| entry.player.subgroup),
            (Sorting::Sub, true) => self
                .players
                .sort_by_key(|entry| Reverse(entry.player.subgroup)),

            (Sorting::Name, false) => self
                .players
                .sort_by(|a, b| a.player.character.cmp(&b.player.character)),
            (Sorting::Name, true) => self
                .players
                .sort_by(|a, b| Reverse(&a.player.character).cmp(&Reverse(&b.player.character))),

            (Sorting::Food, false) => self.players.sort_by_key(|entry| entry.food.state),
            (Sorting::Food, true) => self.players.sort_by_key(|entry| Reverse(entry.food.state)),

            (Sorting::Util, false) => self.players.sort_by_key(|entry| entry.util.state),
            (Sorting::Util, true) => self.players.sort_by_key(|entry| Reverse(entry.util.state)),
        }
    }

    /// Renders a context menu for an item.
    fn render_context_menu(
        ui: &Ui,
        menu_id: &ImStr,
        title: &ImStr,
        buff_id: u32,
        name: Option<&str>,
    ) {
        item_context_menu(menu_id, || {
            ui.text(title);
            if let Some(name) = name {
                if ui.small_button(im_str!("Copy Name")) {
                    ui.set_clipboard_text(&im_str!("{}", name));
                }
                if ui.small_button(im_str!("Open Wiki")) {
                    open::that(format!(
                        "https://wiki-en.guildwars2.com/wiki/Special:Search/{}",
                        name
                    ));
                }
            }
            if ui.small_button(im_str!("Copy ID")) {
                ui.set_clipboard_text(&im_str!("{}", buff_id));
            }
        });
    }

    /// Renders a context menu for a food item.
    fn render_food_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
        Self::render_context_menu(
            ui,
            &im_str!("##food-context-{}", menu_id),
            im_str!("Food Options"),
            buff_id,
            name,
        )
    }

    /// Renders a context menu for a utility item.
    fn render_util_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
        Self::render_context_menu(
            ui,
            &im_str!("##util-context-{}", menu_id),
            im_str!("Utility Options"),
            buff_id,
            name,
        )
    }

    /// Renders a player entry in a table.
    fn render_table_entry(ui: &Ui, entry: &Entry, colors: &exports::Colors, sub: bool) {
        let player = &entry.player;
        let red = colors
            .core(CoreColor::LightRed)
            .map(|vec| vec.into())
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);
        let green = colors
            .core(CoreColor::LightGreen)
            .map(|vec| vec.into())
            .unwrap_or([0.0, 1.0, 0.0, 1.0]);
        let yellow = colors
            .core(CoreColor::LightYellow)
            .map(|vec| vec.into())
            .unwrap_or([1.0, 1.0, 0.0, 1.0]);

        // new row for each player
        ui.table_next_row();

        // render subgroup cell
        if sub {
            ui.table_next_column();
            let sub = format!("{:>2}", player.subgroup);
            match colors.sub_base(player.subgroup) {
                Some(color) => ui.text_colored(color.into(), sub),
                None => ui.text(sub),
            }
        }

        // render name cell
        ui.table_next_column();
        match colors.prof_base(player.profession) {
            Some(color) => ui.text_colored(color.into(), &player.character),
            None => ui.text(&player.character),
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(&player.account);
        }

        // render food cell
        ui.table_next_column();
        match entry.food.state {
            BuffState::Unset => {
                ui.text("???");
                if ui.is_item_hovered() {
                    ui.tooltip_text("Uncertain");
                }
            }
            BuffState::None => {
                ui.text_colored(red, "NONE");
                if ui.is_item_hovered() {
                    ui.tooltip_text("No Food");
                }
            }
            BuffState::Unknown(id) => {
                ui.text_colored(yellow, "SOME");
                if ui.is_item_hovered() {
                    ui.tooltip_text("Unknown Food");
                }
                Self::render_food_context_menu(ui, player.id, id, None);
            }
            BuffState::Known(food) => {
                let color = match food {
                    Food::Malnourished => red,
                    _ => green,
                };
                ui.text_colored(color, food.category().unwrap_or("SOME"));
                if ui.is_item_hovered() {
                    ui.tooltip_text(format!("{}\n{}", food.name(), food.stats().join("\n")));
                }
                Self::render_food_context_menu(ui, player.id, food.into(), Some(food.name()));
            }
        }

        // render util cell
        ui.table_next_column();
        match entry.util.state {
            BuffState::Unset => {
                ui.text("???");
                if ui.is_item_hovered() {
                    ui.tooltip_text("Uncertain");
                }
            }
            BuffState::None => {
                ui.text_colored(red, "NONE");
                if ui.is_item_hovered() {
                    ui.tooltip_text("No Utility");
                }
            }
            BuffState::Unknown(id) => {
                ui.text_colored(yellow, "SOME");
                if ui.is_item_hovered() {
                    ui.tooltip_text("Unknown Utility");
                }
                Self::render_util_context_menu(ui, player.id, id, None);
            }
            BuffState::Known(util) => {
                let color = match util {
                    Utility::Diminished => red,
                    _ => green,
                };
                ui.text_colored(color, util.category().unwrap_or("SOME"));
                if ui.is_item_hovered() {
                    ui.tooltip_text(format!("{}\n{}", util.name(), util.stats().join("\n")));
                }
                Self::render_util_context_menu(ui, player.id, util.into(), Some(util.name()));
            }
        }
    }

    /// Renders the buff table for the squad.
    fn render_squad_tab(&mut self, ui: &Ui) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            if ui.begin_table_with_flags(
                im_str!("##squad-table"),
                4,
                TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X | TableFlags::SORTABLE,
            ) {
                // columns
                ui.table_setup_column_with_flags(
                    im_str!("Sub"),
                    TableColumnFlags::PREFER_SORT_DESCENDING | TableColumnFlags::DEFAULT_SORT,
                );
                ui.table_setup_column_with_flags(
                    im_str!("Player"),
                    TableColumnFlags::PREFER_SORT_DESCENDING,
                );
                ui.table_setup_column_with_flags(
                    im_str!("Food"),
                    TableColumnFlags::PREFER_SORT_DESCENDING,
                );
                ui.table_setup_column_with_flags(
                    im_str!("Util"),
                    TableColumnFlags::PREFER_SORT_DESCENDING,
                );
                ui.table_headers_row();

                // sorting
                if let Some(sort_specs) = unsafe { ig::igTableGetSortSpecs().as_mut() } {
                    // check for changes
                    if sort_specs.SpecsDirty {
                        let column_specs = unsafe {
                            slice::from_raw_parts(sort_specs.Specs, sort_specs.SpecsCount as usize)
                        };
                        if let Some(sorted_column) = column_specs.iter().find(|column| {
                            column.SortDirection() as u32 != ig::ImGuiSortDirection_None
                        }) {
                            // update sorting state
                            match sorted_column.ColumnIndex {
                                0 => self.sorting = Sorting::Sub,
                                1 => self.sorting = Sorting::Name,
                                2 => self.sorting = Sorting::Food,
                                3 => self.sorting = Sorting::Util,
                                _ => {}
                            }

                            // ascending is reverse order for us
                            self.reverse = sorted_column.SortDirection() as u32
                                == ig::ImGuiSortDirection_Ascending;

                            // refresh sorting
                            self.refresh_sort();
                        }
                    }
                }

                // render table content
                let colors = exports::colors();
                for entry in &self.players {
                    Self::render_table_entry(ui, entry, &colors, true);
                }

                ui.end_table();
            }
        }
    }

    fn render_self_tab(&mut self, ui: &Ui) {
        let current = self.get_self();
        if current.is_none() && self.self_cache.is_empty() {
            ui.text("No characters found");
        } else {
            if ui.begin_table_with_flags(
                im_str!("##self-table"),
                4,
                TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
            ) {
                // columns
                ui.table_setup_column(im_str!("Player"));
                ui.table_setup_column(im_str!("Food"));
                ui.table_setup_column(im_str!("Util"));
                ui.table_headers_row();

                // render table content
                let colors = exports::colors();
                if let Some(entry) = current {
                    Self::render_table_entry(ui, entry, &colors, false);
                }
                for entry in &self.self_cache {
                    Self::render_table_entry(ui, entry, &colors, false);
                }

                ui.end_table();
            }
        }
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Tracker {
    fn render(&mut self, ui: &Ui) {
        if unsafe {
            ig::igBeginTabBar(im_str!("##tabs").as_ptr(), ig::ImGuiTabBarFlags_None as i32)
        } {
            if unsafe {
                ig::igBeginTabItem(
                    im_str!("Squad").as_ptr(),
                    ptr::null_mut(),
                    ig::ImGuiTabItemFlags_None as i32,
                )
            } {
                self.render_squad_tab(ui);
                unsafe { ig::igEndTabItem() };
            }
            if unsafe {
                ig::igBeginTabItem(
                    im_str!("Characters").as_ptr(),
                    ptr::null_mut(),
                    ig::ImGuiTabItemFlags_None as i32,
                )
            } {
                self.render_self_tab(ui);
                unsafe { ig::igEndTabItem() };
            }
            unsafe { ig::igEndTabBar() };
        }
    }
}

impl Windowed for Tracker {
    fn window_props() -> WindowProps {
        WindowProps::new("Food Tracker")
            .hotkey(Tracker::HOTKEY)
            .visible(false)
            .auto_resize(true)
    }
}

// required to save window settings
impl HasSettings for Tracker {
    type Settings = ();

    const SETTINGS_ID: &'static str = "tracker";

    fn current_settings(&self) -> Self::Settings {}

    fn load_settings(&mut self, _: Self::Settings) {}
}

/// Possible encounter states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Encounter {
    // No ongoing encounter.
    None,

    // Ongoing unknown encounter.
    Unknown,

    // Ongoing encounter with known boss.
    Boss(Boss),
}

/// Current column sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sorting {
    Sub,
    Name,
    Food,
    Util,
}
