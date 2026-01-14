use std::sync::{Arc, RwLock};

use thiserror::Error;

pub type StreamBufferPassthrough = Arc<RwLock<Vec<f32>>>;
pub const BUFFER_SIZE: u32 = 1024;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("there's no available output device")]
    NoOutputDevice,
    #[error("there was an eframe error (maybe this should be more descriptive)")]
    EframeError
}
