#![windows_subsystem = "windows"]

use serde::{Serialize, Deserialize};
use std::fs;
use eframe::egui;
use chrono::{DateTime,Local, NaiveDate};



#[derive(Serialize, Deserialize, Default, Clone)]
struct Task {
    text: String,  
    completed: bool, 
    created_at: DateTime<Local>, 
    completed_at: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, Default)]
struct MyApp {
    tasks: Vec<Task>,
    new_task: String,
    selected_date: Option<NaiveDate>,
}


impl MyApp {
   fn load_tasks_from_file() -> Self {
        if let Ok(data) = fs::read_to_string("tasks.json") {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&data) {
                return Self {
                    tasks,
                    new_task: String::new(),
                    ..Default::default()
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

    /* need to figure out way to show previous months */
    fn show_calender(&mut self, ctx: &egui::Context) {
        use chrono ::{Datelike, Local, NaiveDate};

              egui::Window::new("📅 Calendar")
            .default_size([200.0, 200.0])
            .collapsible(true)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0]) // fixed top-right
            .show(ctx, |ui| {
                let today = Local::now().date_naive();
                let (year, month) = (today.year(), today.month());

                // Month + Year header
                ui.heading(today.format("%B %Y").to_string()); // format and != format!
                
                // Weekday headers
                ui.horizontal(|ui| {
                    for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
                        ui.label(day);
                        ui.add_space(18.0);
                    }
                });

                  egui::Grid::new("calendar_grid").show(ui, |ui| {
                    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                    let start_weekday = first_day.weekday().num_days_from_monday() as i64;
                    let next_month = if month == 12 {
                        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
                    } else {
                        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
                    };

                    let days_in_month =
                        (next_month - first_day).num_days() as i64;

                    // Empty slots before the 1st
                    for _ in 0..start_weekday {
                        ui.label(" ");
                        
                    }

                    // Fill days
                    for day in 1..=days_in_month {
                        let _text = if day == today.day() as i64{
                            egui::RichText::new(day.to_string())
                                .strong()
                                .color(egui::Color32::LIGHT_BLUE)
                                
                        } else {
                            egui::RichText::new(day.to_string())
                        };
                        
                        //ui.button(text); to stop duplication of days 
                        let date = NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap();
                        if date <= today {
                        if ui.button(day.to_string()).clicked() {
                                self.selected_date = Some(date);
                            }   
                        } else {
                            ui.label(day.to_string());
                        }

                        if (day + start_weekday) % 7 == 0 {
                            ui.end_row();
                        }
                    }

                });

                //saved data on current day, needs ui work
                if let Some(selected) = self.selected_date {
                    ui.add_space(10.0); // optional spacing
                    ui.heading(format!("Tasks completed on {}",selected));

                        let tasks_for_day: Vec<&Task> = self.tasks
                            .iter()
                            .filter(|task| {
                                if let Some(completed_at) = task.completed_at {
                                    completed_at.date_naive() == selected
                                } else {
                                    false
                                }
                            })
                            .collect();

                        if tasks_for_day.is_empty() {
                            ui.label("No tasks completed on this day.");
                        } else {
                            for task in tasks_for_day {
                                ui.label(&task.text); //displays the tasked checked on that day will leave sample to see this.
                            }
                        }
                    }
            });
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
                    self.tasks.push(Task { text: self.new_task.trim().to_string(), completed: false, created_at: Local::now(), completed_at: None });
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

                    let changed = ui.checkbox(&mut task.completed, "").changed();
                    if changed {
                        if task.completed{
                            task.completed_at = Some(Local::now());
                        }else {
                            task.completed_at = None;
                        }
                    }

                    ui.label(&task.text);
                    ui.label(format!("created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S")));

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
           ui.horizontal(|ui| {ui.heading("✅ Completed"); 
            
            if ui.add(egui::Button::new("🗑️")).clicked(){
               self.tasks.retain(|task|!task.completed);
            }
         });
            
            for (i, task) in self.tasks.iter_mut().enumerate().filter(|(_, t)| t.completed) {
                ui.horizontal(|ui| {

                    let changed = ui.checkbox(&mut task.completed,"").changed();

                    if changed {
                        if task.completed{
                            task.completed_at = Some(Local::now());
                        }
                        else{
                            task.completed_at = None;
                        }
                    }

                    ui.label(
                        egui::RichText::new(&task.text)
                            .strikethrough()
                            .color(egui::Color32::LIGHT_GRAY),
                    );

                    if let Some(completed_at) = task.completed_at {
                        ui.label(format!("Completed: {}", completed_at.format("%Y-%m-%d %H:%M:%S")));
                    }
                   
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


        self.show_calender(ctx);
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
