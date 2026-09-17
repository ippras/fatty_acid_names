use crate::r#const::{ABBREVIATION, COMMON, IUPAC, NAMES, PREFIX};
use const_format::formatcp;
use egui::{Grid, Response, Ui, Widget};
use egui_l10n::ContextExt as _;
use lipid::r#struct::fatty_acid::FattyAcid;
use typed_builder::TypedBuilder;

/// Names widget
#[derive(TypedBuilder)]
pub struct Names<'a> {
    fatty_acid: &'a FattyAcid,
}

impl Widget for Names<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.heading(ui.localize(formatcp!("{PREFIX}_{NAMES}")));
        Grid::new(ui.next_auto_id())
            .show(ui, |ui| {
                let id = self.fatty_acid.id().to_string();
                if let Some(abbreviation) = ui.try_localize(&format!("{id}.abbreviation")) {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{ABBREVIATION}")));
                    ui.label(abbreviation);
                    ui.end_row();
                }
                if let Some(common) = ui.try_localize(&format!("{id}.common")) {
                    ui.label(ui.localize(formatcp!("{PREFIX}_{COMMON}")));
                    if let Some(synonyms) = ui.try_localize(&format!("{id}.synonyms")) {
                        let names = format!("{common}; {}", synonyms.replace(";", "; "));
                        ui.label(names);
                    } else {
                        ui.label(common);
                    }
                    ui.end_row();
                }
                ui.label(ui.localize(formatcp!("{PREFIX}_{IUPAC}")));
                ui.label(self.fatty_acid.iupac().to_string());
                ui.end_row();
            })
            .response
    }
}
