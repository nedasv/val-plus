use eframe::egui;
use eframe::egui::Ui;
use poll_promise::Promise;
use crate::{converter, MyApp, State};

pub fn show_settings(app: &mut MyApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Auto Refresh: ");
        ui.checkbox(&mut app.settings.auto_refresh, "");
    });

    ui.vertical(|ui| ui.add(egui::widgets::Separator::default().spacing(10.0)));

    if ui.button("Import VRY data").clicked() {
        app.import_promise = Some(Promise::spawn_thread("import_data", || {
            if let Some(dir) = directories_next::ProjectDirs::from("", "", "vry") {
                let path = dir.clone().data_dir().parent().unwrap().join("stats.json");
                let new_path = path.as_path();

                println!("{:?}", new_path.as_os_str().to_string_lossy());

                let cv = converter::Converter::get_file(new_path.clone());
                let (success, fail, total) = converter::Converter::convert_vry_history(cv);

                return (success, fail, total)
            }

            return (0, 0, 0)
        }));

        app.state = State::CheckPromise;
    }
}