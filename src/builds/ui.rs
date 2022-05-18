use super::{build::Build, Action, Builds};
use crate::{
    buff_ui,
    defs::{DefKind, Definitions},
    tracking::buff::BuffState,
};
use arc_util::{
    api::CoreColor,
    exports,
    game::Profession,
    ui::{render, Ui},
};
use arcdps_imgui::{StyleVar, TableColumnSetup, TableFlags};
use strum::VariantNames;

impl Builds {
    /// Renders viewing mode contents.
    fn render_view(
        &mut self,
        ui: &Ui,
        defs: &Definitions,
        current_prof: Option<Profession>,
        current_food: BuffState,
        current_util: BuffState,
    ) {
        // render builds table
        if let Some(_table) = ui.begin_table_header_with_flags(
            "##builds-table",
            [
                TableColumnSetup::new("Build"),
                TableColumnSetup::new("Food"),
                TableColumnSetup::new("Util"),
            ],
            TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
        ) {
            let colors = exports::colors();

            for (i, build) in self.entries.iter_mut().enumerate() {
                // filter if necessary
                if !self.filter || current_prof.map(|prof| prof == build.prof).unwrap_or(true) {
                    ui.table_next_row();

                    // name column
                    ui.table_next_column();
                    match colors.prof_base(build.prof) {
                        Some(color) => ui.text_colored(color, &build.name),
                        None => ui.text(&build.name),
                    }

                    let red = colors
                        .core(CoreColor::LightRed)
                        .unwrap_or([1.0, 0.0, 0.0, 1.0]);
                    let green = colors
                        .core(CoreColor::LightGreen)
                        .unwrap_or([0.0, 1.0, 0.0, 1.0]);

                    // food
                    ui.table_next_column();
                    if let Some(DefKind::Food(food)) = defs.get_buff(build.food) {
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
                    if let Some(DefKind::Util(util)) = defs.get_buff(build.util) {
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
        let mut action = Action::None;
        let last = self.entries.len() - 1;

        // render builds table
        if let Some(_table) = ui.begin_table_header_with_flags(
            "##builds-table",
            [
                TableColumnSetup::new("Prof"),
                TableColumnSetup::new("Name"),
                TableColumnSetup::new("Food"),
                TableColumnSetup::new("Util"),
                TableColumnSetup::new(""),
            ],
            TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
        ) {
            for (i, build) in self.entries.iter_mut().enumerate() {
                const INPUT_SIZE: f32 = 100.0;

                ui.table_next_row();

                // prof select
                ui.table_next_column();
                let mut prof = build.prof as usize;
                ui.set_next_item_width(INPUT_SIZE);
                if ui.combo_simple_string(format!("##prof-{}", i), &mut prof, Profession::VARIANTS)
                {
                    build.prof = (prof as u32).into();
                }

                // build name
                ui.table_next_column();
                let mut char_name = String::with_capacity(19);
                char_name.push_str(&build.name);
                ui.set_next_item_width(INPUT_SIZE);
                if ui
                    .input_text(format!("##char-{}", i), &mut char_name)
                    .build()
                {
                    build.name = char_name;
                }

                // food select
                ui.table_next_column();
                ui.set_next_item_width(INPUT_SIZE);
                if let Some(changed) = buff_ui::render_buff_combo(
                    ui,
                    format!("##food-{}", i),
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
                    format!("##util-{}", i),
                    build.util,
                    defs.all_util(),
                ) {
                    build.util = changed.id;
                }

                // buttons
                ui.table_next_column();
                let current_alpha = ui.clone_style().alpha;

                let is_first = i == 0;
                let style =
                    ui.push_style_var(StyleVar::Alpha(if is_first { 0.3 } else { current_alpha }));
                if ui.button(format!("^##{}", i)) && !is_first {
                    action = Action::Up(i);
                }
                style.pop();

                ui.same_line();
                let is_last = i == last;
                let style =
                    ui.push_style_var(StyleVar::Alpha(if is_last { 0.3 } else { current_alpha }));
                if ui.button(format!("v##{}", i)) && !is_last {
                    action = Action::Down(i);
                }
                style.pop();

                ui.same_line();
                if ui.button(format!("X##{}", i)) {
                    action = Action::Remove(i);
                }
            }
        }

        // perform action
        match action {
            Action::None => {}
            Action::Up(i) => {
                self.entries.swap(i - 1, i);
            }
            Action::Down(i) => {
                self.entries.swap(i, i + 1);
            }
            Action::Remove(i) => {
                self.entries.remove(i);
            }
        }

        // add button
        if ui.button("Add") {
            self.entries.push(Build::new(
                Profession::Unknown,
                "My Build",
                defs.all_food().next().unwrap().id,
                defs.all_util().next().unwrap().id,
            ));
        }
    }

    /// Renders the builds UI.
    pub fn render(
        &mut self,
        ui: &Ui,
        defs: &Definitions,
        current_prof: Option<Profession>,
        current_food: BuffState,
        current_util: BuffState,
    ) {
        let _style = render::small_padding(ui);

        ui.checkbox("Current profession", &mut self.filter);
        if ui.is_item_hovered() {
            ui.tooltip_text("Only show builds for current profession");
        }

        ui.same_line_with_spacing(0.0, 10.0);
        if self.edit {
            if ui.button("Done") {
                self.edit = false;
            }

            self.render_edit(ui, defs);
        } else {
            if ui.button("Edit") {
                self.edit = true;
            }

            self.render_view(ui, defs, current_prof, current_food, current_util);
        }
    }
}
