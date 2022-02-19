pub mod buff;
pub mod entry;
pub mod settings;

use crate::{
    data::{DIMINISHED, MALNOURISHED},
    defs::{BuffDef, Definitions},
};
use arc_util::{
    api::CoreColor,
    exports,
    ui::{components::item_context_menu, Component, WindowProps, Windowed},
};
use arcdps::imgui::{im_str, sys as ig, ImStr, TabBar, TabItem, TableColumnFlags, TableFlags, Ui};
use buff::BuffState;
use entry::{Entry, Player};
use std::{cmp::Reverse, slice};
use windows::System::VirtualKey;

/// Player tracker.
#[derive(Debug)]
pub struct Tracker {
    /// Buff definitions.
    defs: Definitions,

    /// Currently tracked players.
    players: Vec<Entry>,

    /// Current sorting.
    sorting: Sorting,

    /// Current sorting direction.
    reverse: bool,

    /// Cache for buffs on own characters of local player (self).
    chars_cache: Vec<Entry>,

    /// Whether to save the buffs on own characters.
    pub save_chars: bool,

    /// Current ongoing encounter.
    encounter: Option<usize>,
}

#[allow(dead_code)]
impl Tracker {
    /// Default hotkey for tracker.
    pub const HOTKEY: usize = VirtualKey::F.0 as usize;

    /// Creates a new tracker.
    pub const fn new(defs: Definitions) -> Self {
        Self {
            defs,
            players: Vec::new(),
            sorting: Sorting::Sub,
            reverse: false,
            chars_cache: Vec::new(),
            save_chars: true,
            encounter: None,
        }
    }

    /// Adds a new tracked player.
    pub fn add_player(&mut self, player: Player) {
        let mut added = Entry::new(player);

        // check for self
        if added.player.is_self {
            // check cache
            if let Some(index) = self
                .chars_cache
                .iter()
                .position(|entry| entry.player.character == added.player.character)
            {
                // use cached buffs
                let removed = self.chars_cache.remove(index);
                added.food = removed.food;
                added.util = removed.util;
            }
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
                    self.chars_cache.push(removed.clone());
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

    /// Starts an encounter.
    pub fn start_encounter(&mut self, target_id: usize) {
        self.encounter = Some(target_id);
    }

    /// Ends the current encounter.
    pub fn end_encounter(&mut self) {
        self.encounter = None;
    }

    /// Returns the encounter state.
    pub fn encounter(&self) -> Option<usize> {
        self.encounter
    }

    /// Returns `true` if there is an ongoing boss encounter.
    pub fn in_encounter(&self) -> bool {
        self.encounter.is_some()
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
                    let _ = open::that(format!(
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
    fn render_table_entry(
        &self,
        ui: &Ui,
        entry_id: usize,
        entry: &Entry,
        colors: &exports::Colors,
        sub: bool,
    ) {
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
            BuffState::Unknown => {
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
            BuffState::Some(id) => {
                if let Some(BuffDef::Food(food)) = self.defs.get(id) {
                    let color = match food.id {
                        MALNOURISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &food.display);
                    if ui.is_item_hovered() {
                        ui.tooltip_text(format!("{}\n{}", food.name, food.stats.join("\n")));
                    }
                    Self::render_food_context_menu(ui, entry_id, food.id, Some(&food.name));
                } else {
                    ui.text_colored(yellow, "SOME");
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Food");
                    }
                    Self::render_food_context_menu(ui, entry_id, id, None);
                }
            }
        }

        // render util cell
        ui.table_next_column();
        match entry.util.state {
            BuffState::Unknown => {
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
            BuffState::Some(id) => {
                if let Some(BuffDef::Util(util)) = self.defs.get(id) {
                    let color = match util.id {
                        DIMINISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &util.display);
                    if ui.is_item_hovered() {
                        ui.tooltip_text(format!("{}\n{}", util.name, util.stats.join("\n")));
                    }
                    Self::render_util_context_menu(ui, entry_id, util.id, Some(&util.name));
                } else {
                    ui.text_colored(yellow, "SOME");
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Utility");
                    }
                    Self::render_util_context_menu(ui, entry_id, id, None);
                }
            }
        }
    }

    /// Renders the tracker tab for the squad.
    fn render_squad_tab(&mut self, ui: &Ui) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else if ui.begin_table_with_flags(
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
                    if let Some(sorted_column) = column_specs
                        .iter()
                        .find(|column| column.SortDirection() as u32 != ig::ImGuiSortDirection_None)
                    {
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
                self.render_table_entry(ui, entry.player.id, entry, &colors, true);
            }

            ui.end_table();
        }
    }

    /// Renders the tracker tab for own characters.
    fn render_self_tab(&mut self, ui: &Ui) {
        let current = self.get_self();
        if current.is_none() && self.chars_cache.is_empty() {
            ui.text("No characters found");
        } else if ui.begin_table_with_flags(
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
                self.render_table_entry(ui, usize::MAX, entry, &colors, false);
            }
            for (i, entry) in self.chars_cache.iter().enumerate() {
                self.render_table_entry(ui, i, entry, &colors, false);
            }

            ui.end_table();
        }
    }
}

impl Component for Tracker {
    fn render(&mut self, ui: &Ui) {
        TabBar::new(im_str!("##tabs")).build(ui, || {
            TabItem::new(im_str!("Squad")).build(ui, || {
                self.render_squad_tab(ui);
            });
            TabItem::new(im_str!("Characters")).build(ui, || {
                self.render_self_tab(ui);
            });
        });
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

/// Current column sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sorting {
    Sub,
    Name,
    Food,
    Util,
}
