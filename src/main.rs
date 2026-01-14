#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::significant_drop_tightening)]

use std::{f32::consts::TAU, sync::{Arc, RwLock}, thread, time::Duration};

use thiserror::Error;
use anyhow::Result;

use cpal::{OutputCallbackInfo, traits::{DeviceTrait, HostTrait, StreamTrait}};

const BUFFER_SIZE: u32 = 1024;

fn main() -> Result<()> {
    let host = cpal::default_host();
    let output = host.default_output_device().ok_or(ProgramError::NoOutputDevice)?;
    let mut config = output.default_output_config()?.config();
    config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE);

    let mut sine_wave = SineWave::new(0.019);

    let buffer_passthrough = Arc::new(RwLock::new(Vec::with_capacity(1024)));

    let buffer_thread_passthrough = buffer_passthrough.clone();
    let stream = output.build_output_stream(
        &config,
        move |data: &mut [f32], _: &OutputCallbackInfo| sine_wave.generate_sinewave(data, &buffer_thread_passthrough),
        move |_| { },
        None
    )?;

    stream.play()?;

    loop {
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", buffer_passthrough.read().unwrap().chunks(16).next().unwrap());
    };
}

#[derive(Debug, Error)]
enum ProgramError {
    #[error("there's no available output device")]
    NoOutputDevice,
}

struct SineWave {
    angle: f32,
    speed: f32,
}

impl SineWave {
    const fn new(speed: f32) -> Self {
        Self {
            angle: 0.0,
            speed
        }
    }

    fn generate_sinewave(&mut self, data: &mut [f32], buffer_passthrough: &Arc<RwLock<Vec<f32>>>) {
        let mut lock = buffer_passthrough.write().unwrap();
        lock.clear();
        for sample in data {
            let value = self.angle.sin();
            *sample = value;
            lock.push(value);
            self.angle += self.speed;
            self.angle %= TAU;
        }
    }
}
