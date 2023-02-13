use super::{build::Build, Builds};
use crate::{
    action::Action,
    buff_ui,
    data::{DefinitionKind, Definitions, PROFESSIONS},
    tracking::buff::BuffState,
};
use arc_util::ui::{render, Component, Ui};
use arcdps::{
    exports::{self, CoreColor},
    imgui::{TableColumnSetup, TableFlags},
    Profession,
};

pub type Props<'p> = (
    &'p Definitions,
    Option<Profession>,
    BuffState<u32>,
    BuffState<u32>,
);

impl Builds {
    /// Renders viewing mode contents.
    fn render_view(
        &mut self,
        ui: &Ui,
        defs: &Definitions,
        current_prof: Option<Profession>,
        current_food: BuffState<u32>,
        current_util: BuffState<u32>,
    ) {
        // render builds table
        let name = "##builds-table";
        let flags = TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X;
        let table = if self.display_notes {
            ui.begin_table_header_with_flags(
                name,
                [
                    TableColumnSetup::new("Build"),
                    TableColumnSetup::new("Notes"),
                    TableColumnSetup::new("Food"),
                    TableColumnSetup::new("Util"),
                ],
                flags,
            )
        } else {
            ui.begin_table_header_with_flags(
                name,
                [
                    TableColumnSetup::new("Build"),
                    TableColumnSetup::new("Food"),
                    TableColumnSetup::new("Util"),
                ],
                flags,
            )
        };
        if let Some(_table) = table {
            let colors = exports::colors();

            for (i, build) in self.entries.iter_mut().enumerate() {
                // check if filters match
                let prof_matches =
                    !self.filter_prof || current_prof.map_or(true, |prof| prof == build.prof);

                if build.visible && prof_matches {
                    ui.table_next_row();

                    let red = colors
                        .core(CoreColor::LightRed)
                        .unwrap_or([1.0, 0.0, 0.0, 1.0]);
                    let green = colors
                        .core(CoreColor::LightGreen)
                        .unwrap_or([0.0, 1.0, 0.0, 1.0]);

                    // name
                    ui.table_next_column();
                    match colors.prof_base(build.prof) {
                        Some(color) => ui.text_colored(render::with_alpha(color, 1.0), &build.name),
                        None => ui.text(&build.name),
                    }

                    // notes as column or tooltip
                    if self.display_notes {
                        ui.table_next_column();
                        ui.text(&build.notes);
                    } else if !build.notes.is_empty() && ui.is_item_hovered() {
                        ui.tooltip_text(&build.notes);
                    }

                    // food
                    ui.table_next_column();
                    if let Some(DefinitionKind::Food(food)) = defs.definition(build.food) {
                        match current_food {
                            BuffState::Unknown => ui.text(&food.display),
                            BuffState::Some(id) if id == food.id => {
                                ui.text_colored(green, &food.display)
                            }
                            _ => ui.text_colored(red, &food.display),
                        }

                        buff_ui::render_buff_tooltip(ui, food);
                        buff_ui::render_food_context_menu(
                            ui,
                            i,
                            food.id,
                            Some(&food.name),
                            &colors,
                        );
                    }

                    // util
                    ui.table_next_column();
                    if let Some(DefinitionKind::Util(util)) = defs.definition(build.util) {
                        match current_util {
                            BuffState::Unknown => ui.text(&util.display),
                            BuffState::Some(id) if id == util.id => {
                                ui.text_colored(green, &util.display)
                            }
                            _ => ui.text_colored(red, &util.display),
                        }

                        buff_ui::render_buff_tooltip(ui, util);
                        buff_ui::render_util_context_menu(
                            ui,
                            i,
                            util.id,
                            Some(&util.name),
                            &colors,
                        );
                    }
                }
            }
        }
    }

    /// Renders edit mode contents.
    fn render_edit(&mut self, ui: &Ui, defs: &Definitions) {
        // render builds table
        if let Some(_table) = ui.begin_table_header_with_flags(
            "##builds-table",
            [
                TableColumnSetup::new("Profession"),
                TableColumnSetup::new("Name"),
                TableColumnSetup::new("Notes"),
                TableColumnSetup::new("Food"),
                TableColumnSetup::new("Utility"),
                TableColumnSetup::new(""),
            ],
            TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
        ) {
            let mut action = Action::None;
            let len = self.entries.len();

            for (i, build) in self.entries.iter_mut().enumerate() {
                const INPUT_SIZE: f32 = 100.0;

                ui.table_next_row();

                // prof select
                ui.table_next_column();
                let mut index = PROFESSIONS
                    .iter()
                    .position(|prof| *prof == build.prof)
                    .unwrap();
                ui.set_next_item_width(INPUT_SIZE);
                if ui.combo(format!("##prof-{i}"), &mut index, PROFESSIONS, |prof| {
                    <&str>::from(prof).into()
                }) {
                    build.prof = PROFESSIONS[index];
                }

                // name input
                ui.table_next_column();
                ui.set_next_item_width(INPUT_SIZE);
                ui.input_text(format!("##name-{i}"), &mut build.name)
                    .build();

                // notes input
                ui.table_next_column();
                ui.set_next_item_width(INPUT_SIZE);
                ui.input_text(format!("##notes-{i}"), &mut build.notes)
                    .build();

                // food select
                ui.table_next_column();
                ui.set_next_item_width(INPUT_SIZE);
                if let Some(changed) = buff_ui::render_buff_combo(
                    ui,
                    format!("##food-{i}"),
                    build.food,
                    defs.all_food(),
                ) {
                    build.food = changed.id;
                }

                // util select
                ui.table_next_column();
                ui.set_next_item_width(INPUT_SIZE);
                if let Some(changed) = buff_ui::render_buff_combo(
                    ui,
                    format!("##util-{i}"),
                    build.util,
                    defs.all_util(),
                ) {
                    build.util = changed.id;
                }

                // buttons
                ui.table_next_column();
                action.render_buttons(ui, i, len);
            }
            action.perform(&mut self.entries);
        }

        // add button
        if ui.button("Add") {
            self.entries.push(Build::empty());
        }
    }
}

impl<'p> Component<Props<'p>> for Builds {
    /// Renders the builds UI.
    fn render(&mut self, ui: &Ui, (defs, current_prof, current_food, current_util): Props<'p>) {
        let _style = render::small_padding(ui);

        // profession filter
        ui.checkbox("Current profession", &mut self.filter_prof);
        if ui.is_item_hovered() {
            ui.tooltip_text("Only show builds for current profession");
        }

        // edit mode button
        ui.same_line_with_spacing(0.0, 10.0);
        if self.edit {
            if ui.button("Done") {
                self.edit = false;
                self.refresh_search();
            }
        } else if ui.button("Edit") {
            self.edit = true;
        }

        // search field
        if ui.input_text("##search", &mut self.search).build() {
            self.refresh_search();
        }

        // contents
        if self.edit {
            self.render_edit(ui, defs);
        } else {
            self.render_view(ui, defs, current_prof, current_food, current_util);
        }
    }
}
