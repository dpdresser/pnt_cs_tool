use super::app::NewCSModel;
use crate::services::fs_api::{self, CSModelEntry};

use eframe::egui;
use std::collections::BTreeMap;

pub fn show_new_cs_model_window(
    ui: &mut egui::Ui,
    show_new_cs_model_window: &mut bool,
    cs_models: &mut BTreeMap<String, Vec<fs_api::CSModelEntry>>,
    new_cs_model: &mut NewCSModel,
) {
    ui.horizontal(|ui| {
        ui.label("Ticker for CS Model:\t");
        ui.text_edit_singleline(&mut new_cs_model.ticker);
    });

    ui.horizontal(|ui| {
        ui.label("New Formula:\t");
        ui.text_edit_singleline(&mut new_cs_model.formula);
    });

    ui.horizontal(|ui| {
        ui.label("Formula Type:\t");
        egui::ComboBox::from_id_salt("Type")
            .selected_text(format!("{:?}", &new_cs_model.entry_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    fs_api::CSModelEntryType::Debt,
                    "Debt",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    fs_api::CSModelEntryType::Preferred,
                    "Preferred",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    fs_api::CSModelEntryType::NonControllingInterest,
                    "Noncontrolling Interest",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    fs_api::CSModelEntryType::Cash,
                    "Cash",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    fs_api::CSModelEntryType::Shares,
                    "Shares",
                );
            });
    });

    ui.horizontal(|ui| {
        ui.label("Display Name:\t");
        ui.text_edit_singleline(&mut new_cs_model.display_name);
    });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("Add").clicked() {
            new_cs_model.entries.push(CSModelEntry {
                formula: new_cs_model.formula.clone(),
                entry_type: new_cs_model.entry_type,
                display_name: new_cs_model.display_name.clone(),
            })
        }
    });

    ui.separator();

    ui.label("Model Entries");

    egui_extras::TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .column(egui_extras::Column::initial(100.0).resizable(true))
        .column(egui_extras::Column::initial(100.0).resizable(true))
        .column(egui_extras::Column::initial(100.0).resizable(true))
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Formula");
            });
            header.col(|ui| {
                ui.strong("Type");
            });
            header.col(|ui| {
                ui.strong("Display Name");
            });
        })
        .body(|mut body| {
            for entry in &*new_cs_model.entries {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(&entry.formula);
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", entry.entry_type));
                    });
                    row.col(|ui| {
                        ui.label(&entry.formula);
                    });
                });
            }
        });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("Save").clicked() {
            cs_models.insert(new_cs_model.ticker.clone(), new_cs_model.entries.clone());
        }

        if ui.button("Close").clicked() {
            *show_new_cs_model_window = false;
        }
    });

    ui.separator();
}
