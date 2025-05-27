#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

// Module declarations
mod recorder;
mod window_info;

// Re-export from window_info module
pub use window_info::{get_all_windows, MonitorDimensions, WindowInfo};

// Re-export from recorder module
pub use recorder::{
  enumerate_audio_input_devices, enumerate_video_encoders, get_preferred_video_encoder_by_type,
  AudioInputDevice, AudioSource, Recorder, RecorderConfig, RecorderConfigBuilder, VideoEncoder,
  VideoEncoderType,
};
