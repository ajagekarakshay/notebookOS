use crate::theme::NotebookTheme;
use gpui_kit::component::{
    ActiveTheme as _, Icon, IconName, Selectable as _, Sizable as _, StyledExt as _,
    WindowExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    scroll::ScrollableElement as _,
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, Context, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    Render, Styled as _, WeakEntity, Window, div, rems,
};
use notebookos_domain::{Page, PageId, PageKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Destination {
    Home,
    Notes,
    Boards,
    Calendar,
    Meetings,
}

impl Destination {
    const ALL: [Self; 5] = [
        Self::Home,
        Self::Notes,
        Self::Boards,
        Self::Calendar,
        Self::Meetings,
    ];

    const fn id(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Notes => "notes",
            Self::Boards => "boards",
            Self::Calendar => "calendar",
            Self::Meetings => "meetings",
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Notes => "Notes",
            Self::Boards => "Boards",
            Self::Calendar => "Calendar",
            Self::Meetings => "Meetings",
        }
    }

    const fn icon(self) -> IconName {
        match self {
            Self::Home => IconName::LayoutDashboard,
            Self::Notes => IconName::BookOpen,
            Self::Boards => IconName::Frame,
            Self::Calendar => IconName::Calendar,
            Self::Meetings => IconName::User,
        }
    }
}

pub struct NotebookOs {
    active_destination: Destination,
    pages: Vec<Page>,
    selected_page_id: PageId,
    next_page_id: u128,
}

impl NotebookOs {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        let pages = vec![
            Page::document(PageId::from_u128(1), "Retrieval-augmented synthesis"),
            Page::handwritten(PageId::from_u128(2), "Lab sketches"),
            Page::canvas(PageId::from_u128(3), "Interface map"),
        ];

        Self {
            active_destination: Destination::Notes,
            pages,
            selected_page_id: PageId::from_u128(1),
            next_page_id: 4,
        }
    }

    fn selected_page(&self) -> Option<&Page> {
        self.pages
            .iter()
            .find(|page| page.id() == self.selected_page_id)
    }

    fn select_destination(&mut self, destination: Destination, cx: &mut Context<Self>) {
        self.active_destination = destination;
        cx.notify();
    }

    fn select_page(&mut self, page_id: PageId, cx: &mut Context<Self>) {
        self.active_destination = Destination::Notes;
        self.selected_page_id = page_id;
        cx.notify();
    }

    fn create_page(&mut self, kind: PageKind, cx: &mut Context<Self>) {
        let page_id = PageId::from_u128(self.next_page_id);
        self.next_page_id += 1;

        let page = match kind {
            PageKind::Document => Page::document(page_id, "Untitled document"),
            PageKind::Handwritten => Page::handwritten(page_id, "Untitled handwritten page"),
            PageKind::Canvas => Page::canvas(page_id, "Untitled canvas"),
        };

        self.pages.push(page);
        self.active_destination = Destination::Notes;
        self.selected_page_id = page_id;
        cx.notify();
    }

    fn open_new_page_dialog(window: &mut Window, cx: &mut Context<Self>) {
        let owner = cx.weak_entity();
        window.open_dialog(cx, move |dialog, _, cx| {
            let description_color = cx.theme().muted_foreground;
            dialog.title("New page…").child(
                v_flex()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(description_color)
                            .child("Choose the surface that fits this note."),
                    )
                    .child(page_choice(
                        "new-document",
                        PageKind::Document,
                        IconName::FileText,
                        "Document",
                        "Structured text with optional handwritten blocks",
                        description_color,
                        owner.clone(),
                    ))
                    .child(page_choice(
                        "new-handwritten-page",
                        PageKind::Handwritten,
                        IconName::BookOpen,
                        "Handwritten page",
                        "A full white sheet designed for Apple Pencil",
                        description_color,
                        owner.clone(),
                    ))
                    .child(page_choice(
                        "new-canvas",
                        PageKind::Canvas,
                        IconName::Frame,
                        "Canvas",
                        "A freeform space for text, ink, images, and diagrams",
                        description_color,
                        owner.clone(),
                    )),
            )
        });
    }

    fn render_global_nav(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut navigation = v_flex()
            .w_24()
            .flex_none()
            .items_center()
            .gap_1()
            .p_2()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().border)
            .child(workspace_mark("R", cx))
            .child(div().h_2());

        for destination in Destination::ALL {
            navigation = navigation.child(
                Button::new(format!("destination-{}", destination.id()))
                    .ghost()
                    .selected(self.active_destination == destination)
                    .w_full()
                    .h(rems(3.5))
                    .accessibility_label(destination.label())
                    .child(
                        v_flex()
                            .items_center()
                            .gap_1()
                            .text_xs()
                            .child(Icon::new(destination.icon()).size_4())
                            .child(destination.label()),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select_destination(destination, cx);
                    })),
            );
        }

        navigation
            .child(div().flex_1())
            .child(global_action("search", "Search", IconName::Search))
            .child(global_action("settings", "Settings", IconName::Settings))
    }

    fn render_context_pane(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let heading = h_flex()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .child(self.active_destination.label()),
            )
            .child(div().flex_1())
            .when(self.active_destination == Destination::Notes, |this| {
                this.child(
                    Button::new("new-page")
                        .ghost()
                        .small()
                        .icon(IconName::Plus)
                        .tooltip("New page…")
                        .on_click(cx.listener(|_, _, window, cx| {
                            Self::open_new_page_dialog(window, cx);
                        })),
                )
            });

        let header = v_flex()
            .gap_3()
            .p_4()
            .child(
                h_flex()
                    .gap_2()
                    .child(workspace_mark("R", cx))
                    .child(div().font_semibold().child("Research"))
                    .child(div().flex_1())
                    .child(Icon::new(IconName::ChevronDown).size_4()),
            )
            .child(heading);

        v_flex()
            .w_64()
            .flex_none()
            .min_h_0()
            .bg(cx.theme().sidebar)
            .border_r_1()
            .border_color(cx.theme().border)
            .child(header)
            .child(self.render_context_content(cx))
    }

    fn render_context_content(&self, cx: &mut Context<Self>) -> AnyElement {
        if self.active_destination != Destination::Notes {
            return self.render_destination_summary(cx).into_any_element();
        }

        let mut pages = v_flex()
            .id("page-list")
            .flex_1()
            .min_h_0()
            .gap_1()
            .px_2()
            .pb_4()
            .child(section_label("Library", cx))
            .child(
                Button::new("inbox")
                    .ghost()
                    .small()
                    .w_full()
                    .justify_start()
                    .icon(IconName::Inbox)
                    .label("Inbox"),
            )
            .child(section_label("Research journal", cx));

        for page in &self.pages {
            let page_id = page.id();
            pages = pages.child(
                Button::new(format!("page-{page_id}"))
                    .ghost()
                    .small()
                    .w_full()
                    .justify_start()
                    .selected(page_id == self.selected_page_id)
                    .icon(icon_for_page_kind(page.kind()))
                    .label(page.title().to_owned())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select_page(page_id, cx);
                    })),
            );
        }

        pages
            .overflow_y_scrollbar()
            .id("page-list-scroll")
            .into_any_element()
    }

    fn render_destination_summary(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let details = match self.active_destination {
            Destination::Home => "Recent pages, today's meetings, and quick capture",
            Destination::Notes => "Notebooks, sections, and pages",
            Destination::Boards => "Native boards and connected Trello boards",
            Destination::Calendar => "Agenda and connected calendars",
            Destination::Meetings => "Native and imported meeting notes",
        };

        v_flex()
            .flex_1()
            .min_h_0()
            .p_4()
            .gap_2()
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .child(details)
            .child("This feature will follow after the notes and handwritten-page foundation.")
    }

    fn render_main_region(&self, cx: &mut Context<Self>) -> AnyElement {
        if self.active_destination == Destination::Notes {
            return self.render_notes(cx);
        }

        self.render_destination_placeholder(cx).into_any_element()
    }

    fn render_notes(&self, cx: &mut Context<Self>) -> AnyElement {
        let Some(page) = self.selected_page().cloned() else {
            return v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child("Select or create a page")
                .into_any_element();
        };

        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .child(Self::render_page_toolbar(&page, cx))
            .child(render_page(&page, cx))
            .into_any_element()
    }

    fn render_page_toolbar(page: &Page, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .h_12()
            .flex_none()
            .px_4()
            .gap_2()
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                h_flex()
                    .min_w_0()
                    .gap_2()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child("Research journal")
                    .child(Icon::new(IconName::ChevronRight).size_3())
                    .child(
                        div()
                            .min_w_0()
                            .truncate()
                            .text_color(cx.theme().foreground)
                            .child(page.title().to_owned()),
                    ),
            )
            .child(div().flex_1())
            .child(
                h_flex()
                    .gap_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(div().size_2().rounded_full().bg(cx.theme().primary))
                    .child("Saved locally"),
            )
            .child(
                Button::new("toolbar-new-page")
                    .small()
                    .icon(IconName::Plus)
                    .label("New page…")
                    .on_click(cx.listener(|_, _, window, cx| {
                        Self::open_new_page_dialog(window, cx);
                    })),
            )
    }

    fn render_destination_placeholder(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let (title, description) = match self.active_destination {
            Destination::Home => (
                "Welcome back",
                "Resume recent work, capture a thought, or prepare for the next meeting.",
            ),
            Destination::Notes => ("Notes", "Write, draw, and connect your research."),
            Destination::Boards => (
                "Boards",
                "Plan research and projects with native or connected boards.",
            ),
            Destination::Calendar => (
                "Calendar",
                "See upcoming events and create linked meeting notes.",
            ),
            Destination::Meetings => (
                "Meetings",
                "Keep native notes and Granola imports in one durable collection.",
            ),
        };

        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .items_center()
            .justify_center()
            .gap_3()
            .p_8()
            .child(Icon::new(self.active_destination.icon()).size_8())
            .child(div().text_2xl().font_semibold().child(title))
            .child(
                div()
                    .max_w(rems(28.))
                    .text_center()
                    .text_color(cx.theme().muted_foreground)
                    .child(description),
            )
    }
}

impl Render for NotebookOs {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_w_0()
            .min_h_0()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                h_flex()
                    .h_11()
                    .flex_none()
                    .px_4()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().sidebar)
                    .child(div().font_semibold().child("NotebookOS"))
                    .child(div().flex_1())
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child("Research"),
                    ),
            )
            .child(
                h_flex()
                    .items_stretch()
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .child(self.render_global_nav(cx))
                    .child(self.render_context_pane(cx))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .min_h_0()
                            .child(self.render_main_region(cx)),
                    ),
            )
    }
}

fn workspace_mark(label: &'static str, cx: &App) -> impl IntoElement {
    div()
        .size_9()
        .rounded_lg()
        .flex()
        .items_center()
        .justify_center()
        .font_semibold()
        .bg(cx.theme().primary)
        .text_color(cx.theme().primary_foreground)
        .child(label)
}

fn global_action(id: &'static str, label: &'static str, icon: IconName) -> impl IntoElement {
    Button::new(id)
        .ghost()
        .w_full()
        .h(rems(3.5))
        .accessibility_label(label)
        .child(
            v_flex()
                .items_center()
                .gap_1()
                .text_xs()
                .child(Icon::new(icon).size_4())
                .child(label),
        )
}

fn section_label(label: &'static str, cx: &App) -> impl IntoElement {
    div()
        .mt_3()
        .px_2()
        .text_xs()
        .font_semibold()
        .text_color(cx.theme().muted_foreground)
        .child(label)
}

fn icon_for_page_kind(kind: PageKind) -> IconName {
    match kind {
        PageKind::Document => IconName::FileText,
        PageKind::Handwritten => IconName::BookOpen,
        PageKind::Canvas => IconName::Frame,
    }
}

#[allow(clippy::too_many_arguments)]
fn page_choice(
    id: &'static str,
    kind: PageKind,
    icon: IconName,
    title: &'static str,
    description: &'static str,
    description_color: Hsla,
    owner: WeakEntity<NotebookOs>,
) -> impl IntoElement {
    v_flex()
        .gap_1()
        .child(
            Button::new(id)
                .w_full()
                .justify_start()
                .icon(icon)
                .label(title)
                .on_click(move |_, window, cx| {
                    let _ = owner.update(cx, |this, cx| this.create_page(kind, cx));
                    window.close_dialog(cx);
                }),
        )
        .child(
            div()
                .px_2()
                .text_xs()
                .text_color(description_color)
                .child(description),
        )
}

fn render_page(page: &Page, cx: &App) -> AnyElement {
    match page.kind() {
        PageKind::Document => render_document_page(page, cx),
        PageKind::Handwritten => render_handwritten_page(page, cx),
        PageKind::Canvas => render_canvas_page(page, cx),
    }
}

fn render_document_page(page: &Page, cx: &App) -> AnyElement {
    v_flex()
        .id(format!("document-page-{}", page.id()))
        .size_full()
        .min_h_0()
        .overflow_y_scrollbar()
        .id(format!("document-scroll-{}", page.id()))
        .child(
            v_flex()
                .w_full()
                .max_w(rems(48.))
                .mx_auto()
                .px_8()
                .py_10()
                .gap_6()
                .child(page_heading(page, "October 7, 2026 · Research journal", cx))
                .child(
                    div().text_lg().child(
                        "The central question is how a notebook can preserve the shape of evidence while ideas are written, connected, and revised.",
                    ),
                )
                .child(
                    v_flex()
                        .gap_2()
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().muted)
                        .child(div().font_semibold().child("Working hypothesis"))
                        .child(
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child("Synthesis improves when claims retain stable links to source passages and the notes that transformed them."),
                        ),
                )
                .child(div().text_2xl().font_semibold().child("Evidence path"))
                .child(
                    div().child(
                        "A source enters the workspace as an attachment or reference. Highlights become stable targets, working notes link to those targets, and the final synthesis links to the working notes.",
                    ),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .p_6()
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().border)
                        .child(evidence_node("Source", "page 14", cx))
                        .child(Icon::new(IconName::ArrowRight).size_5())
                        .child(evidence_node("Working note", "claim A", cx))
                        .child(Icon::new(IconName::ArrowRight).size_5())
                        .child(evidence_node("Synthesis", "linked", cx)),
                )
                .child(div().text_2xl().font_semibold().child("Handwritten block"))
                .child(
                    v_flex()
                        .min_h(rems(16.))
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(NotebookTheme::paper_light())
                        .p_4()
                        .text_color(NotebookTheme::ink())
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Button::new("block-pen").small().icon(IconName::FileText))
                                .child(Button::new("block-undo").small().icon(IconName::Undo)),
                        )
                        .child(div().flex_1())
                        .child(
                            div()
                                .text_sm()
                                .child("Embedded ink blocks live inside an ordinary document."),
                        ),
                ),
        )
        .into_any_element()
}

fn render_handwritten_page(page: &Page, cx: &App) -> AnyElement {
    v_flex()
        .size_full()
        .min_h_0()
        .bg(cx.theme().muted)
        .child(handwriting_toolbar(page, cx))
        .child(
            v_flex()
                .id(format!("handwritten-page-{}", page.id()))
                .flex_1()
                .min_h_0()
                .overflow_y_scrollbar()
                .id(format!("handwritten-scroll-{}", page.id()))
                .p_6()
                .child(
                    v_flex()
                        .w_full()
                        .max_w(rems(58.))
                        .min_h(rems(64.))
                        .mx_auto()
                        .p_8()
                        .rounded_lg()
                        .border_1()
                        .border_color(NotebookTheme::paper_rule())
                        .bg(NotebookTheme::paper_light())
                        .text_color(NotebookTheme::ink())
                        .child(
                            h_flex()
                                .child(handwritten_page_heading(page))
                                .child(div().flex_1())
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(NotebookTheme::ink().opacity(0.45))
                                        .child("Apple Pencil surface"),
                                ),
                        )
                        .child(div().flex_1())
                        .child(
                            v_flex()
                                .items_center()
                                .gap_2()
                                .pb_8()
                                .text_color(NotebookTheme::ink().opacity(0.38))
                                .child(Icon::new(IconName::BookOpen).size_8())
                                .child("Write anywhere on this sheet"),
                        ),
                ),
        )
        .into_any_element()
}

fn handwriting_toolbar(page: &Page, cx: &App) -> impl IntoElement {
    h_flex()
        .h_12()
        .flex_none()
        .px_4()
        .gap_2()
        .border_b_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().background)
        .child(
            Button::new("pen")
                .small()
                .selected(true)
                .icon(IconName::FileText)
                .label("Pen"),
        )
        .child(
            Button::new("eraser")
                .small()
                .icon(IconName::Delete)
                .label("Eraser"),
        )
        .child(
            Button::new("lasso")
                .small()
                .icon(IconName::Inspector)
                .label("Lasso"),
        )
        .child(div().flex_1())
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(page.paper_style().map_or("Paper", |style| style.label())),
        )
        .child(
            Button::new("undo")
                .small()
                .icon(IconName::Undo)
                .tooltip("Undo"),
        )
        .child(
            Button::new("redo")
                .small()
                .icon(IconName::Redo)
                .tooltip("Redo"),
        )
}

fn render_canvas_page(page: &Page, cx: &App) -> AnyElement {
    v_flex()
        .size_full()
        .min_h_0()
        .bg(cx.theme().muted)
        .p_6()
        .child(
            div()
                .relative()
                .size_full()
                .min_h_0()
                .rounded_lg()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().background)
                .child(
                    v_flex()
                        .absolute()
                        .top_8()
                        .left_8()
                        .gap_2()
                        .child(div().text_2xl().font_semibold().child(page.title().to_owned()))
                        .child(
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child("Freeform text, images, links, and ink share this spatial surface."),
                        ),
                )
                .child(
                    v_flex()
                        .absolute()
                        .right_8()
                        .bottom_8()
                        .w_64()
                        .gap_2()
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().sidebar)
                        .child(div().font_semibold().child("Linked idea"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("Canvas objects retain stable IDs and can link back to document blocks."),
                        ),
                ),
        )
        .into_any_element()
}

fn page_heading(page: &Page, metadata: &'static str, cx: &App) -> impl IntoElement {
    v_flex()
        .gap_2()
        .child(
            div()
                .text_3xl()
                .font_semibold()
                .child(page.title().to_owned()),
        )
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(metadata),
        )
}

fn handwritten_page_heading(page: &Page) -> impl IntoElement {
    v_flex()
        .gap_2()
        .child(
            div()
                .text_2xl()
                .font_semibold()
                .child(page.title().to_owned()),
        )
        .child(
            div()
                .text_sm()
                .text_color(NotebookTheme::ink().opacity(0.55))
                .child("Full-sheet handwritten page"),
        )
}

fn evidence_node(title: &'static str, detail: &'static str, cx: &App) -> impl IntoElement {
    v_flex()
        .items_center()
        .gap_1()
        .p_3()
        .rounded_lg()
        .bg(cx.theme().muted)
        .child(div().text_sm().font_semibold().child(title))
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(detail),
        )
}
