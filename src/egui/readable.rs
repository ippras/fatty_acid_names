use crate::egui::names::Names;
use egui::{Label, Response, Ui, Widget};
use lipid::r#struct::fatty_acid::FattyAcid;
use typed_builder::TypedBuilder;

/// Readable name widget
#[derive(TypedBuilder)]
pub struct Readable<'a> {
    fatty_acid: &'a FattyAcid,
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
                Names::builder().fatty_acid(self.fatty_acid).build().ui(ui);
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
