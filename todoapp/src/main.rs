use eframe::egui;

#[derive(Default)]
struct MyApp {
    tasks: Vec<String>,
    new_task: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(28.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
        ].into();
        style.spacing.item_spacing = egui::vec2(12.0, 12.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("✅ Modern To-Do App");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_sized([300.0, 30.0], egui::TextEdit::singleline(&mut self.new_task));
                if ui.add_sized([80.0, 30.0], egui::Button::new("➕ Add")).clicked()
                    && !self.new_task.trim().is_empty()
                {
                    self.tasks.push(self.new_task.trim().to_string());
                    self.new_task.clear();
                }
            });

            ui.separator();

            // Collect indexes to delete
            let mut to_delete: Option<usize> = None;

            for (i, task) in self.tasks.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(task);
                    if ui.add(egui::Button::new("❌").fill(egui::Color32::DARK_RED)).clicked() {
                        to_delete = Some(i);
                    }
                });
            }

            // Actually remove *after* loop to avoid index errors
            if let Some(i) = to_delete {
                self.tasks.remove(i);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ToDo App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
