use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ToDo App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())), // <-- FIX
    )
}

struct MyApp {
    task_input: String,
    tasks: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            task_input: String::new(),
            tasks: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ToDo Application");

            // Input field for typing a new task
            ui.text_edit_singleline(&mut self.task_input);

            if ui.button("Add Task").clicked() {
                if !self.task_input.trim().is_empty() {
                    self.tasks.push(self.task_input.trim().to_string());
                    self.task_input.clear();
                }
            }

            ui.separator();

            // Show the task list
            for (i, task) in self.tasks.iter().enumerate() {
                ui.label(format!("{}. {}", i + 1, task));
            }
        });
    }
}
