#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use hidapi::{HidApi, HidDevice};
use std::time::Instant;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let api = HidApi::new().unwrap();
    let dev = api.open(0x1337, 0x4004).unwrap();
    
    let my_app = MyApp {
        cal_step: None,
        usb_handle: dev,
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(my_app)),
    )
}

struct MyApp {
    cal_step: Option<u32>,
    usb_handle: HidDevice,
}

// impl Default for MyApp {
//     fn default() -> Self {
//         Self {
//             cal_step: None
//         }
//     }
// }

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.cal_step {
                None => { ui.label("Wait"); }
                Some(x) => { ui.label(format!("Cal Step: {}", x)); }
            }
            if ui.button("Calibration").clicked() {
                self.usb_handle.send_feature_report(&[0x01, 0x69]).unwrap();
            }

            if ui.button("Get state").clicked() {
                let now = Instant::now();
                let mut arr: &mut[u8] = &mut [0; 1024];
                self.usb_handle.get_feature_report(&mut arr).unwrap();
                //dbg!(&arr);
                self.cal_step = Some(*(arr.get(1).unwrap()) as u32);
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);
            }
        });
    }
}
