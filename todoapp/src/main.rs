#![windows_subsystem = "windows"]

use serde::{Serialize, Deserialize};
use std::fs;
use eframe::egui;


#[derive(Serialize, Deserialize, Default, Clone)]
struct Task {
    text: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct MyApp {
    tasks: Vec<Task>,
    new_task: String,
}

impl MyApp {
   fn load_tasks_from_file() -> Self {
        if let Ok(data) = fs::read_to_string("tasks.json") {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&data) {
                return Self {
                    tasks,
                    new_task: String::new(),
                };
            }
        }
        Self::default()
    }

    fn save_tasks_to_file(&self) {
       
        //saves both finished and nonfinished but not deleted
        if let Ok(json) = serde_json::to_string_pretty(&self.tasks){
            let _ = fs::write("tasks.json",json);
        } 
      
    }
  
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
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("✅ Modern To-Do App");
            ui.add_space(10.0);

            // Input row for new task
            ui.horizontal(|ui| {
                ui.add_sized([300.0, 30.0], egui::TextEdit::singleline(&mut self.new_task));
                if ui.add_sized([80.0, 30.0], egui::Button::new("➕ Add")).clicked()
                    && !self.new_task.trim().is_empty()
                {
                    self.tasks.push(Task { text: self.new_task.trim().to_string(), completed: false });
                    self.new_task.clear();
                    self.save_tasks_to_file();
                   
                }
            });

            ui.separator();

            // Track tasks to remove
            let mut to_delete: Option<usize> = None;

             // -----------------
            // Incomplete tasks
            // -----------------
            ui.heading("📝 To Do");
            for (i, task) in self.tasks.iter_mut().enumerate().filter(|(_, t)| !t.completed) {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.completed, "");
                    ui.label(&task.text);

                    if ui.add(egui::Button::new("❌").fill(egui::Color32::DARK_RED)).clicked() {
                        to_delete = Some(i);
                    }
                });
            }

            ui.add_space(15.0);
            ui.separator();

            // -----------------
            // Completed tasks
            // -----------------
            ui.heading("✅ Completed");
            for (i, task) in self.tasks.iter_mut().enumerate().filter(|(_, t)| t.completed) {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.completed, "");
                    ui.label(
                        egui::RichText::new(&task.text)
                            .strikethrough()
                            .color(egui::Color32::LIGHT_GRAY),
                    );

                    if ui.add(egui::Button::new("❌").fill(egui::Color32::DARK_RED)).clicked() {
                        to_delete = Some(i);
                    }
                });
            }
            

// Remove only deleted tasks
        if let Some(i) = to_delete {
         self.tasks.remove(i);
        
        }

// Save all tasks including completed ones
        self.save_tasks_to_file();

        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ToDo App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::load_tasks_from_file()))),
    )
}
