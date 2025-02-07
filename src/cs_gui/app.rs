use super::new_cs_model;
use crate::services::cs_model::{AppNewCSModel, CSModel};

use crossbeam_channel::Sender;
use dashmap::DashMap;
use eframe::egui;
use std::sync::Arc;

pub struct MyEguiApp {
    show_new_cs_model_window: bool,
    cs_models: Arc<DashMap<String, CSModel>>,
    new_cs_model: AppNewCSModel,
    cs_model_tx: Arc<Sender<CSModel>>,
}

impl MyEguiApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        cs_models: Arc<DashMap<String, CSModel>>,
        cs_model_tx: Arc<Sender<CSModel>>,
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            show_new_cs_model_window: false,
            cs_models,
            new_cs_model: AppNewCSModel::default(),
            cs_model_tx,
        }
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
                    for row in self.cs_models.iter() {
                        let (ticker, model) = (row.key(), row.value());
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
                    self.cs_models.clone(),
                    &mut self.new_cs_model,
                    self.cs_model_tx.clone(),
                )
            });
        };
    }
}
