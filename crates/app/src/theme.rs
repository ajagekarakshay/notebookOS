use gpui_kit::{Hsla, rgb};

/// Product-owned colors for content whose color is part of its meaning.
pub struct NotebookTheme;

impl NotebookTheme {
    /// The physical-paper color used by the default handwritten page style.
    #[must_use]
    pub fn paper_light() -> Hsla {
        rgb(0x00fc_fbf8).into()
    }

    /// The neutral ink color used for the initial handwriting placeholder.
    #[must_use]
    pub fn ink() -> Hsla {
        rgb(0x0026_312d).into()
    }

    /// A subdued ruled-paper line used only inside paper content.
    #[must_use]
    pub fn paper_rule() -> Hsla {
        rgb(0x00dd_e3df).into()
    }
}
