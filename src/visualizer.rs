use std::sync::{Arc, RwLock};

use anyhow::Result;
use cpal::traits::StreamTrait;
use cpal::{Device, Stream, StreamConfig};
use eframe::egui::{Color32, NumExt, pos2, vec2};
use eframe::{App, CreationContext, NativeOptions, egui, run_native};

use crate::audio::{SineWave, sinewave_stream};
use crate::shared::{BUFFER_SIZE, CHANNEL_COUNT, ProgramError, StreamBufferPassthrough};

/// How many frames will be rendered at once.
/// 1 = current frame data only, 2 = current + last frame data, etc.
const FRAMES_BLEDTHROUGH: usize = 2;

pub fn start_visualizer(output_device: &Device, output_config: &StreamConfig) -> Result<()> {
    let native_options = NativeOptions::default();
    run_native(
        "Radio Modulation Visualizer",
        native_options,
        Box::new(|cc| Ok(Box::new(Visualizer::new(cc, output_device, output_config)?)))
    ).map_err(|_| ProgramError::EframeError)?;
    Ok(())
}

struct Visualizer {
    audio_stream: (Stream, StreamBufferPassthrough, Arc<RwLock<SineWave>>),
    sine_speed: f32,
    prev_frame_data: [f32; BUFFER_SIZE * CHANNEL_COUNT],
}

impl Visualizer {
    fn new(cc: &CreationContext<'_>, output_device: &Device, output_config: &StreamConfig) -> Result<Self> {
        cc.egui_ctx.set_visuals(egui::Visuals::default()); // Don't change any styling yet

        // play a sine wave via the `SineWave` struct
        let sine_speed = 0.02;
        let (sinewave_stream, sinewave_buffer, sinewave_generator) = sinewave_stream(output_device, output_config, sine_speed)?;
        sinewave_stream.play()?;

        Ok(Self {
            audio_stream: (sinewave_stream, sinewave_buffer, sinewave_generator),
            sine_speed,
            prev_frame_data: [0.0; BUFFER_SIZE * CHANNEL_COUNT],
        })
    }
}

impl App for Visualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Radio Modulation Visualizer");
                ui.add(egui::Slider::new(&mut self.sine_speed, 0.0..=0.1).step_by(0.0001));
                self.audio_stream.2.write().unwrap().set_speed(self.sine_speed);
                if let Ok(data) = self.audio_stream.1.read().unwrap()[..].try_into() {
                    ui.add(Waveform::new(data, &self.prev_frame_data));
                    self.prev_frame_data.clone_from(data);
                }
            })
        });

        ctx.request_repaint();
    }
}

struct Waveform {
    data: [f32; BUFFER_SIZE * CHANNEL_COUNT],
    prev_frame_data: [f32; BUFFER_SIZE * CHANNEL_COUNT],
}

impl Waveform {
    const fn new(data: &[f32; BUFFER_SIZE * CHANNEL_COUNT], prev_frame_data: &[f32; BUFFER_SIZE * CHANNEL_COUNT]) -> Self {
        Self { data: *data, prev_frame_data: *prev_frame_data}
    }
}

impl egui::Widget for Waveform {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let aspect_ratio: f32 = 0.35;
        let courner_radius = 15.0;

        let width = ui.available_size_before_wrap().x.at_least(250.0);
        let height = width * aspect_ratio;

        let (outer_rect, response) = ui.allocate_exact_size(vec2(width, height), egui::Sense::hover());

        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Panel, true, "Waveform Visualizer")
        });

        if ui.is_rect_visible(response.rect) {
            let visuals = ui.style().visuals.clone();
            let horizontal_padding = outer_rect.width() / 200.0;
            let vertical_padding_percent = 0.95;
            let half_height = outer_rect.height() / 2.0;
            let dot_spacing = (outer_rect.width() - horizontal_padding * 2.0) / (BUFFER_SIZE * CHANNEL_COUNT) as f32;
            ui.painter().rect_filled(outer_rect, courner_radius, visuals.extreme_bg_color);
            let mut segments = Vec::with_capacity(BUFFER_SIZE * CHANNEL_COUNT * FRAMES_BLEDTHROUGH);

            // First pass, current data
            let mut current_data = Vec::with_capacity(BUFFER_SIZE * CHANNEL_COUNT);
            for (index, value) in self.data.into_iter().enumerate() {
                current_data.push(pos2(outer_rect.left() + horizontal_padding + dot_spacing * index as f32, outer_rect.top() + half_height * value.mul_add(vertical_padding_percent, 1.0)));
            }
            segments.extend(current_data.windows(2).map(|points| egui::Shape::line_segment(points[..2].try_into().unwrap(), egui::Stroke::new(2.5, egui::Color32::MAGENTA))));

            // Second pass, last frame's data
            let mut prev_data = Vec::with_capacity(BUFFER_SIZE * CHANNEL_COUNT);
            for (index, value) in self.prev_frame_data.into_iter().enumerate() {
                prev_data.push(pos2(outer_rect.left() + horizontal_padding + dot_spacing * index as f32, outer_rect.top() + half_height * value.mul_add(vertical_padding_percent, 1.0)));
            }
            segments.extend(prev_data.windows(2).map(|points| egui::Shape::line_segment(points[..2].try_into().unwrap(), egui::Stroke::new(5.0, egui::Color32::MAGENTA.additive().blend(Color32::from_black_alpha(255 / 3))))));
            ui.painter().extend(segments);
        }

        response
    }
}
