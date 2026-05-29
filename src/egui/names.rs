use crate::r#const::{ABBREVIATION, COMMON, IUPAC, NAMES};
use egui::{Grid, Response, Ui, Widget};
use egui_l10n::UiExt as _;
use typed_builder::TypedBuilder;

/// Names widget
#[derive(TypedBuilder)]
pub struct Names<'a> {
    id: &'a str,
}

impl Widget for Names<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let id = self.id;
        ui.heading(ui.localize(NAMES));
        Grid::new(ui.next_auto_id())
            .show(ui, |ui| {
                if let Some(abbreviation) = ui.try_localize(&format!("{id}.abbreviation")) {
                    ui.label(ui.localize(ABBREVIATION));
                    ui.label(abbreviation);
                    ui.end_row();
                }
                if let Some(common) = ui.try_localize(&format!("{id}.common")) {
                    ui.label(ui.localize(COMMON));
                    if let Some(synonyms) = ui.try_localize(&format!("{id}.synonyms")) {
                        let names = format!("{common}; {}", synonyms.replace(";", "; "));
                        ui.label(names);
                    } else {
                        ui.label(common);
                    }
                    ui.end_row();
                }
                if let Some(iupac) = ui.try_localize(&format!("{id}.iupac")) {
                    ui.label(ui.localize(IUPAC));
                    ui.label(iupac);
                    ui.end_row();
                }
            })
            .response
    }
}
