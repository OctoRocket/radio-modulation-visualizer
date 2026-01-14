#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::significant_drop_tightening)]

mod shared;
mod audio;

use std::{thread, time::Duration};

use anyhow::Result;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::audio::sinewave_stream;
use crate::shared::{BUFFER_SIZE, ProgramError};

fn main() -> Result<()> {
    // set up audio devices and choose the correct configurations for them
    let host = cpal::default_host();
    let output_device = host.default_output_device().ok_or(ProgramError::NoOutputDevice)?;
    let mut output_config = output_device.default_output_config()?.config();
    output_config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE);

    // play a sine wave via the `SineWave` struct
    let (sine_stream, sine_buffer) = sinewave_stream(&output_device, &output_config, 0.02)?;
    sine_stream.play()?;

    loop {
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", sine_buffer.read().unwrap().chunks(16).next().unwrap());
    };
}
