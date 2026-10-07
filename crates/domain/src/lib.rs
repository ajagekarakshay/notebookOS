//! Domain concepts shared by every `NotebookOS` platform.

use std::fmt;

/// Stable identity for a page, independent of its title or position.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PageId(u128);

impl PageId {
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_u128(self) -> u128 {
        self.0
    }
}

impl fmt::Display for PageId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:032x}", self.0)
    }
}

/// The editing surface owned by a page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageKind {
    /// Vertically ordered content blocks with optional embedded ink blocks.
    Document,
    /// A paper-first surface whose entire content is editable vector ink.
    Handwritten,
    /// A spatial surface containing positioned text, ink, images, and files.
    Canvas,
}

impl PageKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Document => "Document",
            Self::Handwritten => "Handwritten page",
            Self::Canvas => "Canvas",
        }
    }
}

/// Background rendered beneath strokes on a handwritten page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaperStyle {
    PlainLight,
    Ruled,
    Grid,
    DotGrid,
    PlainDark,
}

impl PaperStyle {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PlainLight => "Plain white",
            Self::Ruled => "Ruled",
            Self::Grid => "Grid",
            Self::DotGrid => "Dot grid",
            Self::PlainDark => "Plain dark",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTypeError;

impl fmt::Display for PageTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("paper style is only available for handwritten pages")
    }
}

impl std::error::Error for PageTypeError {}

/// A page and the persistent settings that define its editing surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    id: PageId,
    title: String,
    kind: PageKind,
    paper_style: Option<PaperStyle>,
}

impl Page {
    #[must_use]
    pub fn document(id: PageId, title: impl Into<String>) -> Self {
        Self::new(id, title, PageKind::Document, None)
    }

    #[must_use]
    pub fn handwritten(id: PageId, title: impl Into<String>) -> Self {
        Self::new(
            id,
            title,
            PageKind::Handwritten,
            Some(PaperStyle::PlainLight),
        )
    }

    #[must_use]
    pub fn canvas(id: PageId, title: impl Into<String>) -> Self {
        Self::new(id, title, PageKind::Canvas, None)
    }

    fn new(
        id: PageId,
        title: impl Into<String>,
        kind: PageKind,
        paper_style: Option<PaperStyle>,
    ) -> Self {
        let title = title.into();
        Self {
            id,
            title: normalized_title(&title),
            kind,
            paper_style,
        }
    }

    #[must_use]
    pub const fn id(&self) -> PageId {
        self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn kind(&self) -> PageKind {
        self.kind
    }

    #[must_use]
    pub const fn paper_style(&self) -> Option<PaperStyle> {
        self.paper_style
    }

    pub fn rename(&mut self, title: impl Into<String>) {
        let title = title.into();
        self.title = normalized_title(&title);
    }

    /// Changes the paper rendered beneath a handwritten page.
    ///
    /// # Errors
    ///
    /// Returns [`PageTypeError`] when this page is not handwritten.
    pub fn set_paper_style(&mut self, paper_style: PaperStyle) -> Result<(), PageTypeError> {
        if self.kind != PageKind::Handwritten {
            return Err(PageTypeError);
        }

        self.paper_style = Some(paper_style);
        Ok(())
    }
}

fn normalized_title(title: &str) -> String {
    let title = title.trim();
    if title.is_empty() {
        "Untitled".to_owned()
    } else {
        title.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handwritten_page_starts_as_plain_white_paper() {
        let page = Page::handwritten(PageId::from_u128(42), "Field notes");

        assert_eq!(page.kind(), PageKind::Handwritten);
        assert_eq!(page.paper_style(), Some(PaperStyle::PlainLight));
    }

    #[test]
    fn document_cannot_acquire_handwritten_paper_settings() {
        let mut page = Page::document(PageId::from_u128(7), "Sources");

        assert_eq!(page.set_paper_style(PaperStyle::Grid), Err(PageTypeError));
        assert_eq!(page.paper_style(), None);
    }

    #[test]
    fn blank_titles_have_a_stable_visible_fallback() {
        let page = Page::canvas(PageId::from_u128(9), "   ");

        assert_eq!(page.title(), "Untitled");
    }
}
