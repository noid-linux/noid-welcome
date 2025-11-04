#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Context, FontFamily, TextStyle, FontId};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 400.0]).with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Welcome to Noid Linux",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_styles(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(20.0);
                
                // Top section with image and text
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/noid.png")).fit_to_exact_size(egui::vec2(256.0, 256.0)),
                    );
                    ui.vertical(|ui| {
                        ui.add_space(64.0);
                        ui.heading("Welcome to Noid Linux");
                        ui.label("This tool provides quick access to software installation, system updates, and setup scripts for virtualization and Rust-based tools.");
                        ui.add_space(8.0);
                        ui.label("Have questions or feedback? Don't hesitate to reach out.");
                    });
                });
                
                ui.add_space(20.0);
                
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Software Management");
                        ui.button("Get Software");
                        ui.button("Update System");
                    });
                    ui.add_space(16.0);
                    ui.vertical(|ui| {
                        ui.label("System Tweaks");
                        ui.button("Install/Configure virt-manager");
                        ui.button("Oxidize Your System");
                    });
                    ui.add_space(16.0);
                    ui.vertical(|ui| {
                        ui.label("Links");
                        ui.button("Source Code");
                        ui.button("Matrix Support Channel");
                    });
                });

                ui.horizontal(|ui| {
                    ui.add_space(640.0);
                    ui.checkbox(&mut true, "Show On Startup")
                })
            });
        });
    }
}

fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (TextStyle::Heading, FontId::new(48.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
    ].into();

    ctx.set_style(style);
}
