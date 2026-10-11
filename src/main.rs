use eframe::egui;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([760.0, 520.0])
            .with_min_inner_size([640.0, 440.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Accessible SDH Music Visualizer",
        native_options,
        Box::new(|cc| {
            setup_cjk_fonts(&cc.egui_ctx);
            Ok(Box::new(SdhApp::new()))
        }),
    )
}

fn setup_cjk_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Check system CJK fonts so Chinese characters render crisply
    let font_paths = [
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
    ];

    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "cjk_fallback".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push("cjk_fallback".to_owned());
            break;
        }
    }

    ctx.set_fonts(fonts);
}

struct SdhApp {
    audio_path: Option<String>,
    srt_path: Option<String>,
    output_path: String,
    status_msg: String,
    is_processing: bool,
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl SdhApp {
    fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            audio_path: None,
            srt_path: None,
            output_path: "accessible_output.mp4".to_string(),
            status_msg: "Ready".to_string(),
            is_processing: false,
            tx,
            rx,
        }
    }
}

impl eframe::App for SdhApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(msg) = self.rx.try_recv() {
            self.status_msg = msg;
            self.is_processing = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("Accessible Music & Visual SDH Pipeline");
            ui.label("Architect: Jesse \"Dylan\" Mahoney (何孟維) | License: GPL-3.0");
            ui.separator();
            ui.add_space(14.0);

            egui::Grid::new("file_grid").spacing([12.0, 14.0]).show(ui, |ui| {
                if ui.add_enabled(!self.is_processing, egui::Button::new("Select Audio File")).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Audio", &["wav", "mp3", "flac", "ogg"])
                        .pick_file()
                    {
                        self.audio_path = Some(path.display().to_string());
                    }
                }
                ui.label(self.audio_path.as_deref().unwrap_or("No audio file selected"));
                ui.end_row();

                if ui.add_enabled(!self.is_processing, egui::Button::new("Select SDH Subtitle (.srt)")).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Subtitles", &["srt"])
                        .pick_file()
                    {
                        self.srt_path = Some(path.display().to_string());
                    }
                }
                ui.label(self.srt_path.as_deref().unwrap_or("No subtitle file selected"));
                ui.end_row();

                ui.label("Output Filename:");
                ui.add_enabled(!self.is_processing, egui::TextEdit::singleline(&mut self.output_path));
                ui.end_row();
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(14.0);

            let can_render = self.audio_path.is_some() && self.srt_path.is_some() && !self.is_processing;
            let render_btn = egui::Button::new("  Render Accessible Video  ");

            if ui.add_enabled(can_render, render_btn).clicked() {
                self.is_processing = true;
                self.status_msg = "Rendering in progress...".to_string();

                let audio = self.audio_path.clone().unwrap();
                let srt = self.srt_path.clone().unwrap();
                let out = self.output_path.clone();
                let tx = self.tx.clone();
                let ctx_clone = ctx.clone();

                thread::spawn(move || {
                    let status = Command::new("./render-accessible-track.sh")
                        .arg(&audio)
                        .arg(&srt)
                        .arg(&out)
                        .status();

                    let msg = match status {
                        Ok(s) if s.success() => format!("Render completed successfully: {}", out),
                        Ok(s) => format!("FFmpeg failed with exit code: {:?}", s.code()),
                        Err(e) => format!("Execution error: {}", e),
                    };

                    let _ = tx.send(msg);
                    ctx_clone.request_repaint();
                });
            }

            ui.add_space(12.0);
            ui.label(format!("Status: {}", self.status_msg));
        });
    }
}
