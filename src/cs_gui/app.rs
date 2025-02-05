use super::new_cs_model;
use crate::services::cs_model;

use eframe::egui;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct MyEguiApp {
    show_new_cs_model_window: bool,
    cs_models: BTreeMap<String, Vec<cs_model::CSModelEntry>>,
    new_cs_model: cs_model::AppNewCSModel,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("PNT Capital Structure Tool");
                egui::SidePanel::right("menu_right")
                    .resizable(false)
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Add CS Model").clicked() {
                                self.show_new_cs_model_window = true;
                            }
                        })
                    })
            });

            ui.separator();

            egui_extras::TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .column(egui_extras::Column::initial(100.0).resizable(true))
                .column(egui_extras::Column::initial(300.0).resizable(true))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Ticker");
                    });
                    header.col(|ui| {
                        ui.strong("Formula");
                    });
                })
                .body(|mut body| {
                    for (ticker, model) in &self.cs_models {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(ticker);
                            });
                            row.col(|ui| {
                                ui.label(format!("{:?}", model));
                            });
                        });
                    }
                });
        });

        if self.show_new_cs_model_window {
            egui::Window::new("New CS Model").show(ctx, |ui| {
                new_cs_model::show_new_cs_model_window(
                    ui,
                    &mut self.show_new_cs_model_window,
                    &mut self.cs_models,
                    &mut self.new_cs_model,
                )
            });
        };
    }
}
