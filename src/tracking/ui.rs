use super::{buff::Buffs, settings::Color, BuffState, Sorting, Tracker};
use crate::{
    assets::{FOOD_ICON, UNKNOWN_ICON, UTIL_ICON},
    buff_ui,
    colors::{GREEN, RED, YELLOW},
    combo_ui::render_enum_combo,
    data::{
        DefinitionKind, Definitions, DIMINISHED, MALNOURISHED, NO_BUFF_TEXT, UNKNOWN_BUFF_TEXT,
        UNKNOWN_STATE_TEXT,
    },
    reminder::custom::CustomReminder,
};
use arc_util::{
    tracking::Entry,
    ui::{
        render::{self, TableIconColumn},
        Component, Windowable,
    },
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::{TabBar, TabItem, TableColumnFlags, TableFlags, TableSortDirection, Ui},
    Profession,
};

pub type Props<'p> = (&'p Definitions, &'p [CustomReminder]);

impl Tracker {
    /// Renders reset buttons for squad & characters.
    pub fn render_reset_buttons(&mut self, ui: &Ui, same_line: bool) {
        const SPACING: f32 = 5.0;

        // reset squad
        if ui.button("Reset squad") {
            for entry in self.players.iter_mut() {
                entry.data.reset_buffs();
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
        if render::reset_button(ui, "Reset characters", &mut self.chars_reset) {
            self.players.clear_cache();
        }
        if !self.chars_reset && ui.is_item_hovered() {
            ui.tooltip_text("Clear the cache for own characters.");
        }
    }

    /// Renders a player entry in a table.
    fn render_table_entry(
        &self,
        ui: &Ui,
        (defs, custom): Props,
        colors: &exports::Colors,
        entry: TableEntry,
        show_sub: bool,
    ) {
        let sub_color = colors
            .sub_base(entry.subgroup)
            .map(|color| render::with_alpha(color, 1.0));
        let prof_color = colors
            .prof_base(entry.profession)
            .map(|color| render::with_alpha(color, 1.0));
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
        let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
        let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

        // new row for each player
        ui.table_next_row();

        // render subgroup cell
        if show_sub {
            ui.table_next_column();
            let sub = format!("{:>2}", entry.subgroup);
            match (self.settings.color_sub, sub_color, prof_color) {
                (Color::Sub, Some(color), _) => ui.text_colored(color, sub),
                (Color::Prof, _, Some(color)) => ui.text_colored(color, sub),
                _ => ui.text(sub),
            }
        }

        // render name cell
        ui.table_next_column();
        match (self.settings.color_name, sub_color, prof_color) {
            (Color::Sub, Some(color), _) => ui.text_colored(color, entry.character),
            (Color::Prof, _, Some(color)) => ui.text_colored(color, entry.character),
            _ => ui.text(entry.character),
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(entry.account);
        }

        let TableEntry { buffs, .. } = entry;

        // render food cell
        ui.table_next_column();
        match buffs.food.state {
            BuffState::Unknown => {
                ui.text(UNKNOWN_STATE_TEXT);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Uncertain");
                }
            }
            BuffState::None => {
                ui.text_colored(red, NO_BUFF_TEXT);
                if ui.is_item_hovered() {
                    ui.tooltip_text("No Food");
                }
            }
            BuffState::Some(buff_id) => {
                if let Some(DefinitionKind::Food(food)) = defs.definition(buff_id) {
                    let color = match food.id {
                        MALNOURISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &food.display);
                    buff_ui::render_buff_tooltip(ui, food);
                    buff_ui::render_food_context_menu(
                        ui,
                        entry.id,
                        food.id,
                        Some(&food.name),
                        colors,
                    );
                } else {
                    ui.text_colored(yellow, UNKNOWN_BUFF_TEXT);
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Food");
                    }
                    buff_ui::render_food_context_menu(ui, entry.id, buff_id, None, colors);
                }
            }
        }

        // render util cell
        ui.table_next_column();
        match buffs.util.state {
            BuffState::Unknown => {
                ui.text(UNKNOWN_STATE_TEXT);
                if ui.is_item_hovered() {
                    ui.tooltip_text("Uncertain");
                }
            }
            BuffState::None => {
                ui.text_colored(red, NO_BUFF_TEXT);
                if ui.is_item_hovered() {
                    ui.tooltip_text("No Utility");
                }
            }
            BuffState::Some(buff_id) => {
                if let Some(DefinitionKind::Util(util)) = defs.definition(buff_id) {
                    let color = match util.id {
                        DIMINISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &util.display);
                    buff_ui::render_buff_tooltip(ui, util);
                    buff_ui::render_util_context_menu(
                        ui,
                        entry.id,
                        util.id,
                        Some(&util.name),
                        colors,
                    );
                } else {
                    ui.text_colored(yellow, UNKNOWN_BUFF_TEXT);
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Utility");
                    }
                    buff_ui::render_util_context_menu(ui, entry.id, buff_id, None, colors);
                }
            }
        }

        // render custom buffs cell
        ui.table_next_column();
        ui.group(|| {
            for remind in custom {
                let short = remind.short();
                match buffs.custom_state(remind.id) {
                    BuffState::Unknown => ui.text(short),
                    BuffState::None => ui.text_colored(red, short),
                    BuffState::Some(_) => ui.text_colored(green, short),
                }
                ui.same_line_with_spacing(0.0, 0.0);
            }
        });
        if ui.is_item_hovered() {
            ui.tooltip(|| {
                for remind in custom {
                    let name = remind.display_name();
                    match buffs.custom_state(remind.id) {
                        BuffState::Unknown => ui.text(name),
                        BuffState::None => ui.text_colored(red, name),
                        BuffState::Some(_) => ui.text_colored(green, name),
                    }
                }
            });
        }
    }

    /// Renders the tracker tab for the squad.
    fn render_squad_tab(&mut self, ui: &Ui, props: Props) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            let show_sub = self.settings.show_sub;
            let columns = [
                TableIconColumn::with_flags(
                    "Sub",
                    None,
                    TableColumnFlags::PREFER_SORT_DESCENDING | TableColumnFlags::DEFAULT_SORT,
                ),
                TableIconColumn::with_flags(
                    "Player",
                    None,
                    TableColumnFlags::PREFER_SORT_DESCENDING,
                ),
                TableIconColumn::with_flags(
                    "Food",
                    FOOD_ICON.as_ref(),
                    TableColumnFlags::PREFER_SORT_DESCENDING,
                ),
                TableIconColumn::with_flags(
                    "Util",
                    UTIL_ICON.as_ref(),
                    TableColumnFlags::PREFER_SORT_DESCENDING,
                ),
                TableIconColumn::with_flags(
                    "Buffs",
                    UNKNOWN_ICON.as_ref(),
                    TableColumnFlags::NO_SORT,
                ),
            ];

            if let Some(_table) = render::table_with_icons(
                ui,
                "##squad-table",
                if show_sub { &columns } else { &columns[1..] },
                TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X | TableFlags::SORTABLE,
                self.settings.show_icons,
            ) {
                // update sorting if necessary
                if let Some(sort_specs) = ui.table_sort_specs_mut() {
                    sort_specs.conditional_sort(|column_specs| {
                        let column = column_specs.iter().next().unwrap();
                        if let Some(dir) = column.sort_direction() {
                            // increase index by 1 if no sub column
                            let index = column.column_idx() + if show_sub { 0 } else { 1 };

                            // update sorting state
                            self.sorting = match index {
                                0 => Sorting::Sub,
                                1 => Sorting::Name,
                                2 => Sorting::Food,
                                3 => Sorting::Util,
                                _ => unreachable!("column sort spec index out of range"),
                            };

                            // ascending is reverse order for us
                            self.reverse = dir == TableSortDirection::Ascending;

                            // refresh sorting
                            self.refresh_sort();
                        }
                    });
                }

                // render table content
                let colors = exports::colors();
                for entry in self.players.iter() {
                    self.render_table_entry(
                        ui,
                        props,
                        &colors,
                        TableEntry::from_entry(entry.player.id, entry),
                        show_sub,
                    );
                }
            }
        }
    }

    /// Renders the tracker tab for own characters.
    fn render_characters_tab(&mut self, ui: &Ui, props: Props) {
        let current = self.players.get_self();

        if current.is_none() && !self.players.cached() {
            ui.text("No characters found");
        } else if let Some(_table) = render::table_with_icons(
            ui,
            "##self-table",
            &[
                TableIconColumn::new("Player", None),
                TableIconColumn::new("Food", FOOD_ICON.as_ref()),
                TableIconColumn::new("Util", UTIL_ICON.as_ref()),
                TableIconColumn::new("Buffs", UNKNOWN_ICON.as_ref()),
            ],
            TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
            self.settings.show_icons,
        ) {
            // render table content
            let colors = exports::colors();
            if let Some(entry) = current {
                self.render_table_entry(
                    ui,
                    props,
                    &colors,
                    TableEntry::from_entry(usize::MAX, entry),
                    false,
                );
            }
            for (i, (player, buffs)) in self.players.cache_iter().enumerate() {
                self.render_table_entry(
                    ui,
                    props,
                    &colors,
                    TableEntry {
                        id: i,
                        account: &player.account,
                        character: &player.character,
                        profession: player.profession,
                        buffs,
                        subgroup: 0,
                    },
                    false,
                );
            }
        }
    }

    /// Renders the builds tab for user-defined builds.
    fn render_builds_tab(&mut self, ui: &Ui, (defs, ..): Props) {
        let current = self.players.get_self();
        let prof = current.map(|entry| entry.player.profession);
        let food = current
            .map(|entry| entry.data.food.state)
            .unwrap_or(BuffState::Unknown);
        let util = current
            .map(|entry| entry.data.util.state)
            .unwrap_or(BuffState::Unknown);

        self.builds
            .render(ui, (defs, prof, food, util, self.settings.show_icons));
    }
}

impl Component<Props<'_>> for Tracker {
    fn render(&mut self, ui: &Ui, props: Props) {
        TabBar::new("##tabs").build(ui, || {
            TabItem::new("Squad").build(ui, || {
                self.render_squad_tab(ui, props);
            });

            TabItem::new("Characters").build(ui, || {
                self.render_characters_tab(ui, props);
            });

            TabItem::new("Builds").build(ui, || {
                self.render_builds_tab(ui, props);
            })
        });
    }
}

impl Windowable<Props<'_>> for Tracker {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, _: &(&Definitions, &[CustomReminder])) {
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
            ui.checkbox("Show icons", &mut self.settings.show_icons);
            ui.checkbox("Show subgroup", &mut self.settings.show_sub);
            ui.checkbox("Show build notes", &mut self.builds.display_notes);

            let input_width = render::ch_width(ui, 16);

            ui.set_next_item_width(input_width);
            render_enum_combo(ui, "Subgroup color", &mut self.settings.color_sub);

            ui.set_next_item_width(input_width);
            render_enum_combo(ui, "Name color", &mut self.settings.color_name);
        });
    }
}

#[derive(Debug)]
struct TableEntry<'a> {
    id: usize,
    account: &'a str,
    character: &'a str,
    profession: Profession,
    subgroup: usize,
    buffs: &'a Buffs,
}

impl<'a> TableEntry<'a> {
    fn from_entry(id: usize, entry: &'a Entry<Buffs>) -> Self {
        Self {
            id,
            account: &entry.player.account,
            character: &entry.player.character,
            profession: entry.player.profession,
            subgroup: entry.player.subgroup,
            buffs: &entry.data,
        }
    }
}
