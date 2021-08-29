use eframe::{egui, epi};
use egui::Color32;
mod lst;

pub struct LstApp {
    longitude: String,
}

impl Default for LstApp {
    fn default() -> Self {
        Self {
            longitude: "0.0".to_owned(),
        }
    }
}

impl epi::App for LstApp {
    fn name(&self) -> &str {
        "Local Sidereal Time GUI"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self {longitude } = self;
        egui::CentralPanel::default().show(ctx, |ui| {

            ctx.request_repaint();
            ui.vertical_centered(|ui| {
                ui.heading("Local Sidereal Time");
            });

            ui.separator();

            ui.group(|ui|{
                ui.label("Longitude");
                ui.text_edit_singleline(longitude);
            });
            let lon = match longitude.parse() {
                Ok(lon) => lon,
                Err(_e) => 0.0,
            };

            let utc_string = lst::utc_str();
            let jd_ut1 = lst::jd(&utc_string);
            let era = lst::era(jd_ut1) % 360.;
            let gmst = lst::era(jd_ut1);
            let lst = lst::lst_at_lon(lon, gmst);
            let utc_string = lst::utc_str_simple();

            let (gmst_h, gmst_m, gmst_s) = lst::deg_to_hms(gmst);
            let (tls_h, tls_m, tls_s) = lst::deg_to_hms(lst);

            let gmst = format!("{:0>2.0}:{:0>2.0}:{:0>2.0}", gmst_h, gmst_m, gmst_s);
            let lst = format!("{:0>2.0}:{:0>2.0}:{:0>2.0}", tls_h, tls_m, tls_s);

            egui::Grid::new("data_fields")
            .striped(true)
            .min_col_width(200.)
            .max_col_width(200.)
            .show(ui, |ui| {
                ui.label("UTC"); ui.label(utc_string);
                ui.end_row();
                ui.label("Julian day UTC (JD)"); ui.label(format!("{:.3}", jd_ut1));
                ui.end_row();
                ui.label("ERA (Earth Rotation Angle)"); ui.label(format!("{:.3}", era));
                ui.end_row();
                ui.label("GMST"); ui.label(gmst);
                ui.end_row();
                ui.colored_label(Color32::from_rgb(128, 140, 255), "Local Sidereal Time");
                ui.colored_label(Color32::from_rgb(128, 140, 255), lst);
                ui.end_row();
            });

            ui.separator();
            ui.vertical_centered(|ui| {
                ui.hyperlink("https://github.com/alelouis/");
            });

            egui::warn_if_debug_build(ui);
        });
    }
}
