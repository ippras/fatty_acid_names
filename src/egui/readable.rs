use crate::egui::names::Names;
use egui::{Label, Response, Ui, Widget};
use typed_builder::TypedBuilder;

/// Readable name widget
#[derive(TypedBuilder)]
pub struct Readable<'a> {
    id: &'a str,
    text: &'a str,
    #[builder(default = true)]
    hover: bool,
    #[builder(default)]
    truncate: bool,
}

impl Readable<'_> {
    pub fn show(self, ui: &mut Ui) -> Response {
        let mut label = Label::new(self.text);
        if self.truncate {
            label = label.truncate();
        }
        let mut response = label.ui(ui);
        if self.hover {
            response = response.on_hover_ui(|ui| {
                Names::builder().id(self.id).build().ui(ui);
            });
        }
        response
    }
}

impl Widget for Readable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}
