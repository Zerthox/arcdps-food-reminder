use arcdps::imgui::{StyleVar, Ui};

/// An action for ordered lists in UI.
#[derive(Debug, Default, Clone)]
pub enum Action {
    #[default]
    None,
    Remove(usize),
    Up(usize),
    Down(usize),
}

impl Action {
    /// Renders action buttons.
    pub fn render_buttons(&mut self, ui: &Ui, index: usize, len: usize) {
        let is_first = index == 0;
        let is_last = index == len - 1;
        let current_alpha = ui.clone_style().alpha;

        let style = ui.push_style_var(StyleVar::Alpha(if is_first { 0.3 } else { current_alpha }));
        if ui.button(format!("^##{index}")) && !is_first {
            *self = Self::Up(index);
        }
        style.pop();

        ui.same_line();
        let style = ui.push_style_var(StyleVar::Alpha(if is_last { 0.3 } else { current_alpha }));
        if ui.button(format!("v##{index}")) && !is_last {
            *self = Self::Down(index);
        }
        style.pop();

        ui.same_line();
        if ui.button(format!("x##{index}")) {
            *self = Self::Remove(index);
        }
    }

    /// Performs the action on a [`Vec`].
    pub fn perform<T>(&self, vec: &mut Vec<T>) {
        match *self {
            Self::None => {}
            Self::Up(i) => vec.swap(i - 1, i),
            Self::Down(i) => vec.swap(i, i + 1),
            Self::Remove(i) => {
                vec.remove(i);
            }
        }
    }
}
