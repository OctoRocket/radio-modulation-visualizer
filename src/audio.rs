use std::{f32::consts::TAU, sync::{Arc, RwLock}};

use anyhow::Result;

use cpal::{
    traits::DeviceTrait,
    Device,
    OutputCallbackInfo,
    Stream,
    StreamConfig
};

use crate::shared::{StreamBufferPassthrough, BUFFER_SIZE};

pub fn sinewave_stream(output_device: &Device, output_config: &StreamConfig, speed: f32) -> Result<(Stream, StreamBufferPassthrough)> {
    let mut sine_wave = SineWave::new(speed);

    let buffer_passthrough = Arc::new(RwLock::new(Vec::with_capacity(BUFFER_SIZE as usize)));

    let buffer_thread_passthrough = buffer_passthrough.clone();
    let stream = output_device.build_output_stream(
        output_config,
        move |data: &mut [f32], _: &OutputCallbackInfo| sine_wave.generate_sinewave(data, &buffer_thread_passthrough),
        move |_| { },
        None
    )?;

    Ok((stream, buffer_passthrough))
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

    fn generate_sinewave(&mut self, data: &mut [f32], buffer_passthrough: &StreamBufferPassthrough) {
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
