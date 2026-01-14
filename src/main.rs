#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::significant_drop_tightening)]

mod shared;
mod audio;
mod visualizer;

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};

use crate::shared::{BUFFER_SIZE, ProgramError};
use crate::visualizer::start_visualizer;

fn main() -> Result<()> {
    // set up audio devices and choose the correct configurations for them
    let host = cpal::default_host();
    let output_device = host.default_output_device().ok_or(ProgramError::NoOutputDevice)?;
    let mut output_config = output_device.default_output_config()?.config();
    output_config.buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE);

    start_visualizer(&output_device, &output_config)?;

    Ok(())
}
