use eframe::{App, CreationContext, Frame};
use egui::{Button, CentralPanel, Context, Ui};

pub struct GuiElements{
    // Add any fields you need for your GUI elements here
    // a_field: String,
}

impl App for GuiElements {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Hello, world!");
            if ui.add(Button::new("OK")).clicked() {
                println!("Button clicked!");
            }   
        });
    }
}

pub fn run_app() -> eframe::Result<()>{
        eframe::run_native(
        "GUI Elements", 
        eframe::NativeOptions::default(), 
        Box::new(|_cc: &CreationContext<'_>| {
            Ok(Box::new(GuiElements {}))  // Wrap in Ok
        })
        
    )
}