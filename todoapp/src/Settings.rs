



use eframe::egui;
use crate::{AppPage, MyApp};


pub fn show_settings_page(app: &mut MyApp, ctx: &egui::Context) {

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("⚙ Settings");

        ui.separator();


       
        if ui.button("⬅ Back").clicked() {
            app.current_page = AppPage::ToDo;
        }


        ui.add_space(20.0);


        ui.heading("Appearance");


        if ui.button("🖼 Choose Background Image").clicked() {
            
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg"])
                .pick_file()
            {

                app.background_path = Some(
                    path.to_string_lossy().to_string()
                );


                // force reload next frame
                app.background_texture = None;

    }
            

        }


        ui.add_space(10.0);


        ui.checkbox(
            &mut app.dark_mode,
            "Dark Mode"
        );


    });
}