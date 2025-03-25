use anyhow::Result;
use gpui::{
    actions, div, prelude::*, Action, AsyncWindowContext, Context, Entity, EventEmitter,
    FocusHandle, Focusable, FontWeight, Hsla, IntoElement, Render, WeakEntity,
};
use library::{DocumentType, Model};
use project::Project;
use ui::{px, ActiveTheme, App, IconName, Label, Pixels, Window};
use workspace::{
    dock::{DockPosition, PanelEvent},
    Panel, Workspace,
};

pub struct LibraryPanel {
    project: Entity<Project>,
    library: Model,
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    width: Option<Pixels>,
}

#[derive(Debug)]
pub enum Event {
    Focus,
}

actions!(library_panel, [ToggleFocus]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<LibraryPanel>(window, cx);
        });
    })
    .detach();
}

impl LibraryPanel {
    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let library = Model::new();
        // cx.update_entity(&library, |&model, cx| {
        //     model.add_document(
        //         "Research".to_string(),
        //         DocumentType::PDF,
        //         "research.pdf".into(),
        //         None,
        //     );
        // });

        let project = workspace.project().clone();
        let library_panel = cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            cx.on_focus(&focus_handle, window, Self::focus_in).detach();
            cx.on_focus_out(&focus_handle, window, |this, _, window, cx| {
                this.focus_out(window, cx);
                // this.hide_scrollbar(window, cx);
            })
            .detach();

            // todo: subscribe stuff, also make `this` mutable
            let this = Self {
                project: project.clone(),
                workspace: workspace.weak_handle(),
                library,
                width: None,
                focus_handle,
            };

            this
        });

        // cx.subscribe_in(&library_panel, window, {}).detach();

        library_panel
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = LibraryPanel::new(workspace, window, cx);
            // if let Some(serialized_panel) = serialized_panel {
            //     panel.update(cx, |panel, cx| {
            //         panel.width = serialized_panel.width.map(|px| px.round());
            //         cx.notify();
            //     });
            // }
            panel
        })
    }

    fn render_document_tree(
        &self,
        cx: &mut Context<Self>,
        document_id: usize,
        depth: usize,
    ) -> impl IntoElement {
        let document = self.library.get_document(document_id).unwrap().clone();
        let is_selected = self.library.selected_document_id == Some(document_id);
        let has_children = !document.children.is_empty();

        let _indent = depth * 16; // 16px indentation per level

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // .pl(DefiniteLength::from(indent))
                    .py_1()
                    .bg(if is_selected {
                        Hsla::blue()
                    } else {
                        Hsla::transparent_black()
                    })
                    .rounded_md()
                    .text_color(if is_selected {
                        Hsla::white()
                    } else {
                        Hsla::red()
                    }), // .text_color(if is_selected { "white" } else { "gray-800" })
                        // .hover(|style| style.bg("blue-100"))
                        // .on_click(cx.listener(move |this, _event, _, cx| {
                        //     cx.update_model(&this.library, |model, cx| {
                        //         model.set_selected_document(Some(document_id));
                        //     });
                        // })),
            )
            .children(if has_children {
                document
                    .children
                    .iter()
                    .map(|&child_id| self.render_document_tree(cx, child_id, depth + 1))
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            })
    }

    fn _document_icon(&self, doc_type: &DocumentType, _: &mut Context<Self>) -> impl IntoElement {
        let icon = match doc_type {
            DocumentType::PDF => "📄",
            DocumentType::URL => "🌐",
            DocumentType::Text => "📝",
            DocumentType::Image => "🖼️",
            DocumentType::Other(name) if name == "Folder" => "📁",
            DocumentType::Other(_) => "📎",
        };

        div().child(Label::new(format!("{}", icon)))
    }

    fn focus_in(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.focus_handle.contains_focused(window, cx) {
            cx.emit(Event::Focus);
        }
    }

    fn focus_out(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        if self.focus_handle.is_focused(window) {
            // self.confirm(&Confirm, window, cx);
        }
    }
}

impl Focusable for LibraryPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LibraryPanel {
    fn render(&mut self, _: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors().surface_background)
            .p_2()
            .gap_2()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .child("Research Library"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .overflow_x_hidden()
                    .p_1()
                    .size_full()
                    .bg(cx.theme().colors().panel_background)
                    .border(px(1.))
                    .border_color(cx.theme().colors().border)
                    .children(
                        self.library
                            .root_documents
                            .iter()
                            .map(|&id| self.render_document_tree(cx, id, 0)),
                    ),
            )
            .child(
                div().flex().gap_2().child(
                    div()
                        .px_3()
                        .py_1()
                        // .bg("blue-600")
                        .text_color(Hsla::white())
                        .rounded_md()
                        .hover(|style| {
                            style
                            // .bg(Hsla::blue())
                        })
                        .child("Add Document"),
                ),
            )
    }
}

impl EventEmitter<Event> for LibraryPanel {}
impl EventEmitter<PanelEvent> for LibraryPanel {}

impl Panel for LibraryPanel {
    fn position(&self, _: &Window, _cx: &App) -> DockPosition {
        DockPosition::Left
        // match cx.config().library_panel_position {
        //     LibraryPanelPosition::Left => DockPosition::Left,
        //     LibraryPanelPosition::Right => DockPosition::Right,
        // }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _position: DockPosition, _: &mut Window, _cx: &mut Context<Self>) {
        // cx.config_mut().library_panel_position = position;
    }

    fn size(&self, _: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or_else(|| Pixels(240.0))
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        cx.notify();
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Library)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Library Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "Library Panel"
    }

    fn activation_priority(&self) -> u32 {
        0
    }
}
