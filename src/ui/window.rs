use super::Component;
use arcdps::imgui::{ImString, Ui, Window as ImGuiWindow};
use std::ops::{Deref, DerefMut};

/// Window component.
#[derive(Debug)]
pub struct Window<T>
where
    T: Component,
{
    pub props: WindowProps,
    pub inner: T,
    pub shown: bool,
}

impl<T> Window<T>
where
    T: Component,
{
    /// Toggles the visibility of the window.
    pub fn toggle_visibility(&mut self) {
        self.shown = !self.shown;
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
    type Props = (WindowProps, T::Props);

    fn create(props: Self::Props) -> Self {
        let (props, inner_props) = props;
        let shown = props.visible;
        Self {
            props,
            inner: T::create(inner_props),
            shown,
        }
    }

    fn render(&mut self, ui: &Ui) {
        if self.shown {
            let name = ImString::new(&self.props.name);
            let inner = &mut self.inner;
            ImGuiWindow::new(&name)
                .title_bar(true)
                .draw_background(true)
                .collapsible(false)
                .always_auto_resize(self.props.auto_resize)
                .resizable(self.props.resize)
                .scroll_bar(self.props.scroll)
                .scrollable(self.props.scroll)
                .focus_on_appearing(false)
                .opened(&mut self.shown)
                .build(ui, || inner.render(ui))
        }
    }
}

/// Window props.
#[derive(Debug)]
pub struct WindowProps {
    name: String,
    visible: bool,
    resize: bool,
    auto_resize: bool,
    scroll: bool,
}

impl WindowProps {
    /// Creates new props for a [`Window`].
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            visible: true,
            resize: true,
            auto_resize: false,
            scroll: true,
        }
    }

    /// Sets whether the window is initially visible.
    pub fn visible(mut self, value: bool) -> Self {
        self.visible = value;
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
}
