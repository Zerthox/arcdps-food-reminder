use arcdps::imgui::{StyleVar, Ui};

/// An action for ordered lists in UI.
#[derive(Debug, Clone)]
pub enum Action {
    None,
    Remove(usize),
    Up(usize),
    Down(usize),
}

impl Action {
    /// Renders action buttons.
    pub fn render_buttons(&mut self, ui: &Ui, index: usize, len: usize) {
        let current_alpha = ui.clone_style().alpha;

        let is_first = index == 0;
        let style = ui.push_style_var(StyleVar::Alpha(if is_first { 0.3 } else { current_alpha }));
        if ui.button(format!("^##{index}")) && !is_first {
            *self = Self::Up(index);
        }
        style.pop();

        ui.same_line();
        let is_last = index == len - 1;
        let style = ui.push_style_var(StyleVar::Alpha(if is_last { 0.3 } else { current_alpha }));
        if ui.button(format!("v##{index}")) && !is_last {
            *self = Self::Down(index);
        }
        style.pop();

        ui.same_line();
        if ui.button(format!("X##{index}")) {
            *self = Self::Remove(index);
        }
    }

    /// Performs the action on a [`Vec`].
    pub fn perform<T>(&self, vec: &mut Vec<T>) {
        match *self {
            Action::None => {}
            Action::Up(i) => vec.swap(i - 1, i),
            Action::Down(i) => vec.swap(i, i + 1),
            Action::Remove(i) => {
                vec.remove(i);
            }
        }
    }
}
