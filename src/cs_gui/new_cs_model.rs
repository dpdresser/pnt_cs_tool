use crate::services::cs_model;

use eframe::egui;
use std::collections::BTreeMap;

pub fn show_new_cs_model_window(
    ui: &mut egui::Ui,
    show_new_cs_model_window: &mut bool,
    cs_models: &mut BTreeMap<String, Vec<cs_model::CSModelEntry>>,
    new_cs_model: &mut cs_model::AppNewCSModel,
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
                    cs_model::CSModelEntryType::Debt,
                    "Debt",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    cs_model::CSModelEntryType::Preferred,
                    "Preferred",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    cs_model::CSModelEntryType::NonControllingInterest,
                    "Noncontrolling Interest",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    cs_model::CSModelEntryType::Cash,
                    "Cash",
                );
                ui.selectable_value(
                    &mut new_cs_model.entry_type,
                    cs_model::CSModelEntryType::Shares,
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
            new_cs_model.entries.push(cs_model::CSModelEntry {
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
        .column(egui_extras::Column::initial(200.0).resizable(false))
        .column(egui_extras::Column::initial(200.0).resizable(false))
        .column(egui_extras::Column::initial(200.0).resizable(false))
        .column(egui_extras::Column::initial(50.0).resizable(false))
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
            header.col(|ui| {
                ui.strong("Remove");
            });
        })
        .body(|mut body| {
            for (i, entry) in new_cs_model.entries.clone().iter().enumerate() {
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
                    row.col(|ui| {
                        if ui.button("X").clicked() {
                            new_cs_model.entries.remove(i);
                        }
                    });
                });
            }
        });

    ui.separator();

    ui.horizontal(|ui| {
        if ui.button("Save").clicked() {
            let save_cs_model = cs_model::CSModel {
                ticker: new_cs_model.ticker.clone(),
                entries: new_cs_model.entries.clone(),
            };
            println!("{}", save_cs_model.db_string());
            cs_models.insert(save_cs_model.ticker.clone(), save_cs_model.entries.clone());
        }

        if ui.button("Close").clicked() {
            *show_new_cs_model_window = false;
        }
    });

    ui.separator();
}
