use egui::{CentralPanel, Context, Ui};
use crate::detector;
use crate::downloader;
use crate::installer;

pub struct App {
    detected_gpu: Option<detector::GpuInfo>,
    status: String,
    error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            detected_gpu: None,
            status: "Ready".to_string(),
            error: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Universal GPU Driver");
            
            if ui.button("Detect GPU").clicked() {
                match detector::detect_gpu() {
                    Ok(gpu) => self.detected_gpu = Some(gpu),
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
            
            if let Some(ref gpu) = self.detected_gpu {
                ui.label(format!("Detected: {} {}", gpu.vendor, gpu.model));
                
                if ui.button("Install Driver").clicked() {
                    self.status = "Downloading...".to_string();
                    let gpu_clone = gpu.clone();
                    let ctx = ctx.clone();
                    
                    tokio::spawn(async move {
                        match downloader::download_driver(&gpu_clone).await {
                            Ok(path) => {
                                installer::install_driver(&path).unwrap();
                                ctx.request_repaint();
                            }
                            Err(e) => {
                                ctx.request_repaint();
                            }
                        }
                    });
                }
            }
            
            ui.label(&self.status);
            
            if let Some(ref error) = self.error {
                ui.colored_label(egui::Color32::RED, error);
            }
        });
    }
}