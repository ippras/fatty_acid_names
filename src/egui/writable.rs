use crate::{
    r#const::{ABBREVIATION, COMMON, ERASE, IUPAC, NAME},
    egui::names::Names,
};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::UiExt as _;
use egui_phosphor::regular::{ERASER, PENCIL};
use typed_builder::TypedBuilder;

/// Writable name widget
#[derive(TypedBuilder)]
pub struct Writable<'a> {
    id: &'a str,
    text: &'a mut String,
    #[builder(default = true)]
    hover: bool,
}

impl Writable<'_> {
    pub fn show(self, ui: &mut Ui) -> Response {
        let mut response = ui.text_edit_singleline(self.text);
        let mut changed = false;
        response.context_menu(|ui| {
            let id = self.id;

            let abbreviation = ui.try_localize(&format!("{id}.{}", ABBREVIATION.to_lowercase()));
            ui.add_enabled_ui(abbreviation.is_some(), |ui| {
                let mut atom = RichText::new(ui.localize(ABBREVIATION));
                if matches!(&abbreviation, Some(abbreviation) if self.text == abbreviation) {
                    atom = atom.strong();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(abbreviation) = abbreviation
                {
                    *self.text = abbreviation;
                    changed = true;
                }
            });

            let common = ui.try_localize(&format!("{id}.{}", COMMON.to_lowercase()));
            ui.add_enabled_ui(common.is_some(), |ui| {
                let mut atom =
                    RichText::new(ui.localize(formatcp!("{COMMON}{NAME}?PluralCategory=one")));
                if matches!(&common, Some(common) if self.text == common) {
                    atom = atom.strong();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(common) = common
                {
                    *self.text = common;
                    changed = true;
                }
            });

            let iupac = ui.try_localize(&format!("{id}.{}", IUPAC.to_lowercase()));
            ui.add_enabled_ui(iupac.is_some(), |ui| {
                let mut atom =
                    RichText::new(ui.localize(formatcp!("{IUPAC}{NAME}?PluralCategory=one")));
                if matches!(&iupac, Some(iupac) if self.text == iupac) {
                    atom = atom.strong();
                }
                if ui.button((PENCIL, atom)).clicked()
                    && let Some(iupac) = iupac
                {
                    *self.text = iupac;
                    changed = true;
                }
            });

            if ui.button((ERASER, ui.localize(ERASE))).clicked() {
                *self.text = String::new();
                changed = true;
            }
        });
        if changed {
            response.mark_changed();
        }
        if self.hover {
            response = response.on_hover_ui(|ui| {
                Names::builder().id(self.id).build().ui(ui);
            });
        }
        response
    }
}

impl Widget for Writable<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

// fn button(ui: &mut Ui, id: &str, attribute: &str, source: &str) -> Option<String> {
//     let target = ui.try_localize(&format!("{id}.{}", attribute.to_lowercase()));
//     ui.add_enabled_ui(target.is_some(), |ui| {
//         let mut atom = RichText::new(ui.localize(attribute));
//         if matches!(&target, Some(target) if source == target) {
//             atom = atom.strong();
//         }
//         if ui.button((PENCIL, atom)).clicked() {
//             return target;
//         }
//         None
//     })
//     .inner
// }
