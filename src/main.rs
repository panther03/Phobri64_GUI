#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use hidapi::{HidApi, HidDevice};
use std::{time::Instant, f32::consts::E};

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
        stick_x: 0,
        stick_y: 0
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
    stick_x: u16,
    stick_y: u16
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
        let arr: &mut[u8] = &mut [0; 16];
        self.usb_handle.read_timeout(arr, 1).unwrap();
        dbg!(arr[0]);
        if arr[0] == 0x69 {
            if let Some(x) = self.cal_step {
                self.cal_step = Some(x+1);
            }
        } else if arr[0] == 0x68 {
            if let Some(x) = self.cal_step {
                self.cal_step = Some(x-1);
            }
        } else {
            self.stick_x = arr[1] as u16;
            self.stick_y = arr[2] as u16;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.cal_step {
                None => { ui.label("Wait"); }
                Some(x) => { ui.label(format!("Cal Step: {}", x)); }
            }
            ui.label(format!("X: {}", self.stick_x));
            ui.label(format!("Y: {}", self.stick_y));
            if ui.button("Calibration").clicked() {
                self.usb_handle.send_feature_report(&[0x01, 0x69]).unwrap();
            }

            // if ui.button("Get state").clicked() {
            //     let now = Instant::now();
            //     let mut arr: &mut[u8] = &mut [0; 16];
            //     self.usb_handle.get_feature_report(&mut arr).unwrap();
            //     //dbg!(&arr);
            //     self.cal_step = Some(*(arr.get(1).unwrap()) as u32);
            //     let elapsed = now.elapsed();
            //     println!("Elapsed: {:.2?}", elapsed);
            // }

            // if ui.button("Read controller's report").clicked() {
            //     let now = Instant::now();
            //     let mut arr: &mut[u8] = &mut [0; 12];
            //     self.usb_handle.read_timeout(arr, 1).unwrap(); // 1 millisecond
            //     dbg!(&arr);
            //     let elapsed = now.elapsed();
            //     println!("Elapsed: {:.2?}", elapsed);
            // }
            
        });
    }
}

