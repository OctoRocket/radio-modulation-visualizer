use std::sync::{Arc, RwLock};

use thiserror::Error;

pub type StreamBufferPassthrough = Arc<RwLock<Vec<f32>>>;
pub const BUFFER_SIZE: usize = 1024;
pub const CHANNEL_COUNT: usize = 2;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("there's no available output device")]
    NoOutputDevice,
    #[error("there was an eframe error (maybe this should be more descriptive)")]
    EframeError
}
