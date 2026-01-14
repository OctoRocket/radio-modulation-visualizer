use std::sync::{Arc, RwLock};

use anyhow::Result;
use cpal::traits::StreamTrait;
use cpal::{Device, Stream, StreamConfig};
use eframe::{App, CreationContext, NativeOptions, egui, run_native};

use crate::audio::{SineWave, sinewave_stream};
use crate::shared::{ProgramError, StreamBufferPassthrough};

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
                ui.label(format!("{:?}", self.audio_stream.1.read().unwrap()))
            })
        });

        ctx.request_repaint();
    }
}
