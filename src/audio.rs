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

pub fn sinewave_stream(output_device: &Device, output_config: &StreamConfig, speed: f32) -> Result<(Stream, StreamBufferPassthrough, Arc<RwLock<SineWave>>)> {
    let sine_wave = Arc::new(RwLock::new(SineWave::new(speed)));

    let buffer_passthrough = Arc::new(RwLock::new(Vec::with_capacity(BUFFER_SIZE as usize)));

    let buffer_thread_passthrough = buffer_passthrough.clone();
    let sine_wave_ref = sine_wave.clone();
    let stream = output_device.build_output_stream(
        output_config,
        move |data: &mut [f32], _: &OutputCallbackInfo| SineWave::generate_sinewave(&sine_wave_ref, data, &buffer_thread_passthrough),
        move |_| { },
        None
    )?;

    Ok((stream, buffer_passthrough, sine_wave))
}

pub struct SineWave {
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

    pub const fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    fn generate_sinewave(self_: &Arc<RwLock<Self>>, data: &mut [f32], buffer_passthrough: &StreamBufferPassthrough) {
        let mut buffer_lock = buffer_passthrough.write().unwrap();
        buffer_lock.clear();

        for sample in data {
            let value = self_.read().unwrap().angle.sin();
            *sample = value;
            buffer_lock.push(value);

            let current_speed = self_.read().unwrap().speed;
            self_.write().unwrap().angle += current_speed;
            self_.write().unwrap().angle %= TAU;
        }
    }
}
