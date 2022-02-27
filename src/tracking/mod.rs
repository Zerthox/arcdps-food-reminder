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
    ui::{components::item_context_menu, Component},
};
use arcdps::imgui::{
    TabBar, TabItem, TableColumnFlags, TableColumnSetup, TableFlags, TableSortDirection, Ui,
};
use buff::BuffState;
use entry::{Entry, Player};
use std::cmp::Reverse;
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
    pub const fn new() -> Self {
        Self {
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
    fn render_context_menu(ui: &Ui, menu_id: &str, title: &str, buff_id: u32, name: Option<&str>) {
        item_context_menu(menu_id, || {
            ui.text(title);
            if let Some(name) = name {
                if ui.small_button("Copy Name") {
                    ui.set_clipboard_text(name);
                }
                if ui.small_button("Open Wiki") {
                    let _ = open::that(format!(
                        "https://wiki-en.guildwars2.com/wiki/Special:Search/{}",
                        name
                    ));
                }
            }
            if ui.small_button("Copy ID") {
                ui.set_clipboard_text(buff_id.to_string());
            }
        });
    }

    /// Renders a context menu for a food item.
    fn render_food_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
        Self::render_context_menu(
            ui,
            &format!("##food-context-{}", menu_id),
            "Food Options",
            buff_id,
            name,
        )
    }

    /// Renders a context menu for a utility item.
    fn render_util_context_menu(ui: &Ui, menu_id: usize, buff_id: u32, name: Option<&str>) {
        Self::render_context_menu(
            ui,
            &format!("##util-context-{}", menu_id),
            "Utility Options",
            buff_id,
            name,
        )
    }

    /// Renders a player entry in a table.
    fn render_table_entry(
        &self,
        ui: &Ui,
        defs: &Definitions,
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
                if let Some(BuffDef::Food(food)) = defs.get(id) {
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
                if let Some(BuffDef::Util(util)) = defs.get(id) {
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
    fn render_squad_tab(&mut self, ui: &Ui, defs: &Definitions) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            let col_sub = TableColumnSetup::new("Sub");
            col_sub.flags =
                TableColumnFlags::PREFER_SORT_DESCENDING | TableColumnFlags::DEFAULT_SORT;

            let col_player = TableColumnSetup::new("Player");
            col_player.flags = TableColumnFlags::PREFER_SORT_DESCENDING;

            let col_food = TableColumnSetup::new("Food");
            col_food.flags = TableColumnFlags::PREFER_SORT_DESCENDING;

            let col_util = TableColumnSetup::new("Util");
            col_util.flags = TableColumnFlags::PREFER_SORT_DESCENDING;

            if let Some(_table) = ui.begin_table_header_with_flags(
                "##squad-table",
                [col_player],
                TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X | TableFlags::SORTABLE,
            ) {
                // update sorting if necessary
                if let Some(sort_specs) = ui.table_sort_specs_mut() {
                    sort_specs.conditional_sort(|column_specs| {
                        if let Some(sorted_column) = column_specs
                            .iter()
                            .find(|column| column.sort_direction().is_some())
                        {
                            // update sorting state
                            match sorted_column.column_idx() {
                                0 => self.sorting = Sorting::Sub,
                                1 => self.sorting = Sorting::Name,
                                2 => self.sorting = Sorting::Food,
                                3 => self.sorting = Sorting::Util,
                                _ => {}
                            }

                            // ascending is reverse order for us
                            self.reverse = sorted_column.sort_direction().unwrap()
                                == TableSortDirection::Ascending;

                            // refresh sorting
                            self.refresh_sort();
                        }
                    });
                }

                // render table content
                let colors = exports::colors();
                for entry in &self.players {
                    self.render_table_entry(ui, defs, entry.player.id, entry, &colors, true);
                }
            }
        }
    }

    /// Renders the tracker tab for own characters.
    fn render_self_tab(&mut self, ui: &Ui, defs: &Definitions) {
        let current = self.get_self();
        if current.is_none() && self.chars_cache.is_empty() {
            ui.text("No characters found");
        } else if let Some(_table) = ui.begin_table_header_with_flags(
            "##self-table",
            [
                TableColumnSetup::new("Player"),
                TableColumnSetup::new("Food"),
                TableColumnSetup::new("Util"),
            ],
            TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
        ) {
            // render table content
            let colors = exports::colors();
            if let Some(entry) = current {
                self.render_table_entry(ui, defs, usize::MAX, entry, &colors, false);
            }
            for (i, entry) in self.chars_cache.iter().enumerate() {
                self.render_table_entry(ui, defs, i, entry, &colors, false);
            }
        }
    }
}

impl Component for Tracker {
    type Props = Definitions;

    fn render(&mut self, ui: &Ui, defs: &Self::Props) {
        TabBar::new("##tabs").build(ui, || {
            TabItem::new("Squad").build(ui, || {
                self.render_squad_tab(ui, defs);
            });
            TabItem::new("Characters").build(ui, || {
                self.render_self_tab(ui, defs);
            });
        });
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
