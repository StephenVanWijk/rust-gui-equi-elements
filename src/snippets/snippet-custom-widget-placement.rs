fn custom_ui(ui: &mut egui::Ui) {
    // Place a button at (50, 50)
    ui.put(
        Pos2::new(50.0, 50.0),
        egui::Button::new("Click me!")
    );

    // Draw a label at mouse position on click
    if ui.button("Show label at mouse").clicked() {
        if let Some(pos) = ui.ctx().pointer_latest_pos() {
            ui.put(pos, egui::Label::new("Hello!"));
        }
    }
}