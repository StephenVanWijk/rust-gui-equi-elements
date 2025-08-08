use std::process;

use eframe::{App, CreationContext, Frame};
use egui::{Button, CentralPanel, Context, Grid, MenuBar, Rect, Sense, TopBottomPanel, Ui, Vec2};

pub struct GuiElements{
    // Add any fields you need for your GUI elements here
    // a_field: String,
}

impl App for GuiElements {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::bottom("top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        //functionality
                    }
                    if ui.button("Quit").clicked() {
                        process::exit(0);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        //functionality
                    }
                    if ui.button("Copy").clicked() {
                        //functionality
                    }
                    if ui.button("Paste").clicked() {
                        //funtionality
                    }
                })
            });
        });
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            Grid::new("manual_spacing_grid").show(ui, |ui| {
                ui.label("");
                ui.end_row();
                // Empty column (placeholder)
                ui.label("");        
                // Your widget
                // ui.button("Click Me");
                if ui.add(Button::new("OK")).clicked() {
                   println!("Button clicked!");
                }       
                // Another empty column
                ui.label("");       
                ui.end_row();   
            });
            Grid::new("manual_spacing_grid").show(ui, |ui| {
                let width = ui.available_width() / 3.0;

                // Empty column (left spacer)
                ui.allocate_rect(Rect::from_min_size(ui.cursor().min, Vec2::new(width, 0.0)), Sense);

                // Centered element
                ui.label("Perfectly Centered");
                // Empty column (right spacer)
                ui.allocate_rect(Rect::from_min_size(ui.cursor().min, Vec2::new(width, 0.0)), Sense::hover());
                ui.end_row();
            });
        });
    }
}    

pub fn run_app() -> eframe::Result<()>{
    eframe::run_native(
    "GUI Elements", 
    eframe::NativeOptions::default(), 
    Box::new(|_cc: &CreationContext<'_>| {
        Ok(Box::new(GuiElements {}))  // Wrap in Ok
    }))
}