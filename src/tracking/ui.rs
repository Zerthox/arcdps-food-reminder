use super::{settings::Color, BuffState, Entry, Sorting, Tracker};
use crate::{
    buff_ui,
    data::{DefKind, Definitions, DIMINISHED, MALNOURISHED},
};
use arc_util::ui::{render, Component, Windowable};
use arcdps::{
    exports::{self, CoreColor},
    imgui::{
        TabBar, TabItem, TableColumnFlags, TableColumnSetup, TableFlags, TableSortDirection, Ui,
    },
};

impl Tracker {
    /// Renders reset buttons for squad & characters.
    pub fn render_reset_buttons(&mut self, ui: &Ui, same_line: bool) {
        const SPACING: f32 = 5.0;

        // reset squad
        if ui.button("Reset squad") {
            for entry in &mut self.players {
                entry.reset_buffs();
            }
        }
        if ui.is_item_hovered() {
            ui.tooltip_text("Reset all buff states for the squad/party.");
        }

        // optional same line
        if same_line {
            ui.same_line_with_spacing(0.0, SPACING);
        }

        // reset characters
        if !self.chars_reset {
            if ui.button("Reset characters") {
                self.chars_reset = true;
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Clear the cache for own characters.");
            }
        } else {
            ui.align_text_to_frame_padding();
            ui.text("Reset characters?");

            ui.same_line();
            if ui.button("Confirm") {
                self.chars_cache.clear();
                self.chars_reset = false;
            }

            ui.same_line_with_spacing(0.0, SPACING);
            if ui.button("Cancel") {
                self.chars_reset = false;
            }
        }
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
        let sub_color = colors
            .sub_base(player.subgroup)
            .map(|color| render::with_alpha(color, 1.0));
        let prof_color = colors
            .prof_base(player.profession)
            .map(|color| render::with_alpha(color, 1.0));
        let red = colors
            .core(CoreColor::LightRed)
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);
        let green = colors
            .core(CoreColor::LightGreen)
            .unwrap_or([0.0, 1.0, 0.0, 1.0]);
        let yellow = colors
            .core(CoreColor::LightYellow)
            .unwrap_or([1.0, 1.0, 0.0, 1.0]);

        // new row for each player
        ui.table_next_row();

        // render subgroup cell
        if sub {
            ui.table_next_column();
            let sub = format!("{:>2}", player.subgroup);
            match (self.settings.color_sub, sub_color, prof_color) {
                (Color::Sub, Some(color), _) => ui.text_colored(color, sub),
                (Color::Prof, _, Some(color)) => ui.text_colored(color, sub),
                _ => ui.text(sub),
            }
        }

        // render name cell
        ui.table_next_column();
        match (self.settings.color_name, sub_color, prof_color) {
            (Color::Sub, Some(color), _) => ui.text_colored(color, &player.character),
            (Color::Prof, _, Some(color)) => ui.text_colored(color, &player.character),
            _ => ui.text(&player.character),
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
                if let Some(DefKind::Food(food)) = defs.get_buff(id) {
                    let color = match food.id {
                        MALNOURISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &food.display);
                    buff_ui::render_buff_tooltip(ui, food);
                    buff_ui::render_food_context_menu(
                        ui,
                        entry_id,
                        food.id,
                        Some(&food.name),
                        colors,
                    );
                } else {
                    ui.text_colored(yellow, "SOME");
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Food");
                    }
                    buff_ui::render_food_context_menu(ui, entry_id, id, None, colors);
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
                if let Some(DefKind::Util(util)) = defs.get_buff(id) {
                    let color = match util.id {
                        DIMINISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &util.display);
                    buff_ui::render_buff_tooltip(ui, util);
                    buff_ui::render_util_context_menu(
                        ui,
                        entry_id,
                        util.id,
                        Some(&util.name),
                        colors,
                    );
                } else {
                    ui.text_colored(yellow, "SOME");
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Utility");
                    }
                    buff_ui::render_util_context_menu(ui, entry_id, id, None, colors);
                }
            }
        }

        // render reinforced cell
        ui.table_next_column();
        match entry.reinf.state {
            BuffState::Unknown => ui.text("???"),
            BuffState::None => ui.text_colored(red, "No"),
            BuffState::Some(_) => ui.text_colored(green, "Yes"),
        }
    }

    /// Renders the tracker tab for the squad.
    fn render_squad_tab(&mut self, ui: &Ui, defs: &Definitions) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            let col_sub = TableColumnSetup {
                name: "Sub",
                user_id: 0.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING | TableColumnFlags::DEFAULT_SORT,
                init_width_or_weight: 0.0,
            };

            let col_player = TableColumnSetup {
                name: "Player",
                user_id: 1.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            let col_food = TableColumnSetup {
                name: "Food",
                user_id: 2.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            let col_util = TableColumnSetup {
                name: "Util",
                user_id: 3.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            let col_reinf = TableColumnSetup {
                name: "Reinf",
                user_id: 4.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            const TABLE_ID: &str = "##squad-table";
            let table_flags =
                TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X | TableFlags::SORTABLE;

            if let Some(_table) = if self.settings.show_sub {
                ui.begin_table_header_with_flags(
                    TABLE_ID,
                    [col_sub, col_player, col_food, col_util, col_reinf],
                    table_flags,
                )
            } else {
                ui.begin_table_header_with_flags(
                    TABLE_ID,
                    [col_player, col_food, col_util, col_reinf],
                    table_flags,
                )
            } {
                // update sorting if necessary
                if let Some(sort_specs) = ui.table_sort_specs_mut() {
                    sort_specs.conditional_sort(|column_specs| {
                        if let Some(sorted_column) = column_specs
                            .iter()
                            .find(|column| column.sort_direction().is_some())
                        {
                            // update sorting state
                            match sorted_column.column_user_id() {
                                0 => self.sorting = Sorting::Sub,
                                1 => self.sorting = Sorting::Name,
                                2 => self.sorting = Sorting::Food,
                                3 => self.sorting = Sorting::Util,
                                4 => self.sorting = Sorting::Reinf,
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
                    self.render_table_entry(
                        ui,
                        defs,
                        entry.player.id,
                        entry,
                        &colors,
                        self.settings.show_sub,
                    );
                }
            }
        }
    }

    /// Renders the tracker tab for own characters.
    fn render_characters_tab(&mut self, ui: &Ui, defs: &Definitions) {
        let current = self.get_self();

        if current.is_none() && self.chars_cache.is_empty() {
            ui.text("No characters found");
        } else if let Some(_table) = ui.begin_table_header_with_flags(
            "##self-table",
            [
                TableColumnSetup::new("Player"),
                TableColumnSetup::new("Food"),
                TableColumnSetup::new("Util"),
                TableColumnSetup::new("Reinf"),
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

    /// Renders the builds tab for user-defined builds.
    fn render_builds_tab(&mut self, ui: &Ui, defs: &Definitions) {
        let current = self.get_self();
        let prof = current.map(|entry| entry.player.profession);
        let food = current
            .map(|entry| entry.food.state)
            .unwrap_or(BuffState::Unknown);
        let util = current
            .map(|entry| entry.util.state)
            .unwrap_or(BuffState::Unknown);

        self.builds.render(ui, (defs, prof, food, util));
    }
}

impl Component<&Definitions> for Tracker {
    fn render(&mut self, ui: &Ui, defs: &Definitions) {
        TabBar::new("##tabs").build(ui, || {
            TabItem::new("Squad").build(ui, || {
                self.render_squad_tab(ui, defs);
            });

            TabItem::new("Characters").build(ui, || {
                self.render_characters_tab(ui, defs);
            });

            TabItem::new("Builds").build(ui, || {
                self.render_builds_tab(ui, defs);
            })
        });
    }
}

impl Windowable<&Definitions> for Tracker {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, _defs: &&Definitions) {
        let colors = exports::colors();
        let grey = colors
            .core(CoreColor::MediumGrey)
            .unwrap_or([0.5, 0.5, 0.5, 1.0]);

        // hotkey
        render::input_key(ui, "##hotkey", "Hotkey", &mut self.settings.hotkey);

        ui.spacing();

        // reset buttons
        self.render_reset_buttons(ui, false);

        ui.spacing();

        // display options
        ui.menu("Display", || {
            ui.text_colored(grey, "Display");

            // table column checkboxes
            ui.checkbox("Show subgroup", &mut self.settings.show_sub);
            ui.checkbox("Show build notes", &mut self.builds.display_notes);

            const COLORS: &[Color] = &[Color::None, Color::Sub, Color::Prof];
            let input_width = render::ch_width(ui, 16);

            let mut sub_index = COLORS
                .iter()
                .position(|entry| *entry == self.settings.color_sub)
                .unwrap();

            ui.set_next_item_width(input_width);
            if ui.combo("Subgroup color", &mut sub_index, COLORS, |entry| {
                entry.to_string().into()
            }) {
                self.settings.color_sub = COLORS[sub_index];
            }

            let mut name_index = COLORS
                .iter()
                .position(|entry| *entry == self.settings.color_name)
                .unwrap();

            ui.set_next_item_width(input_width);
            if ui.combo("Name color", &mut name_index, COLORS, |entry| {
                entry.to_string().into()
            }) {
                self.settings.color_name = COLORS[name_index];
            }
        });
    }
}
