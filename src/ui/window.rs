use super::Component;
use arcdps::imgui::{ImString, Ui, Window as ImGuiWindow};
use std::ops::{Deref, DerefMut};

/// Window component.
#[derive(Debug)]
pub struct Window<T>
where
    T: Component,
{
    inner: T,
    pub shown: bool,
    name: String,
    resize: bool,
    auto_resize: bool,
    scroll: bool,
}

impl<T> Window<T>
where
    T: Component,
{
    /// Creates a new window with a given inner [`Component`].
    pub fn with_inner<S>(name: S, inner: T) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            resize: true,
            auto_resize: false,
            scroll: true,
            inner,
            shown: true,
        }
    }

    /// Sets whether the window is visible.
    pub fn visible(mut self, value: bool) -> Self {
        self.shown = value;
        self
    }

    /// Sets whether the window is resizable.
    pub fn resize(mut self, value: bool) -> Self {
        self.resize = value;
        self
    }

    /// Sets whether the window automatically resizes.
    pub fn auto_resize(mut self, value: bool) -> Self {
        self.auto_resize = value;
        self
    }

    /// Sets whether the window is scrollable.
    pub fn scroll(mut self, value: bool) -> Self {
        self.scroll = value;
        self
    }

    /// Toggles the visibility of the window.
    pub fn toggle_visibility(&mut self) {
        self.shown = !self.shown;
    }
}

impl<T> Window<T>
where
    T: Component + Default,
{
    /// Creates a new window with the [`Default`] of the inner [`Component`].
    pub fn with_default<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self::with_inner(name, T::default())
    }
}

impl<T> Deref for Window<T>
where
    T: Component,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Window<T>
where
    T: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Component for Window<T>
where
    T: Component,
{
    fn render(&mut self, ui: &Ui) {
        if self.shown {
            let name = ImString::new(&self.name);
            let inner = &mut self.inner;
            ImGuiWindow::new(&name)
                .title_bar(true)
                .draw_background(true)
                .collapsible(false)
                .always_auto_resize(self.auto_resize)
                .resizable(self.resize)
                .scroll_bar(self.scroll)
                .scrollable(self.scroll)
                .focus_on_appearing(false)
                .opened(&mut self.shown)
                .build(ui, || inner.render(ui))
        }
    }
}
