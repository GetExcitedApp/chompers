use napi::bindgen_prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use windows_record::{
  AudioInputDevice as WinAudioInputDevice, AudioSource as WinAudioSource,
  Recorder as WindowsRecorder, RecorderConfig as WinRecorderConfig,
  VideoEncoder as WinVideoEncoder, VideoEncoderType as WinVideoEncoderType,
};

// Re-export types from windows-record for use in JavaScript
#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct AudioInputDevice {
  pub id: String,
  pub name: String,
}

impl From<WinAudioInputDevice> for AudioInputDevice {
  fn from(device: WinAudioInputDevice) -> Self {
    Self {
      id: device.id,
      name: device.name,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct VideoEncoder {
  pub name: String,
  pub encoder_type: String,
}

impl From<WinVideoEncoder> for VideoEncoder {
  fn from(encoder: WinVideoEncoder) -> Self {
    Self {
      name: encoder.name,
      encoder_type: format!("{:?}", encoder.encoder_type),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(string_enum)]
pub enum VideoEncoderType {
  H264,
  HEVC,
}

impl From<VideoEncoderType> for WinVideoEncoderType {
  fn from(js_type: VideoEncoderType) -> Self {
    match js_type {
      VideoEncoderType::H264 => WinVideoEncoderType::H264,
      VideoEncoderType::HEVC => WinVideoEncoderType::HEVC,
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(string_enum)]
pub enum AudioSource {
  Desktop,
  ActiveWindow,
}

impl From<AudioSource> for WinAudioSource {
  fn from(js_source: AudioSource) -> Self {
    match js_source {
      AudioSource::Desktop => WinAudioSource::Desktop,
      AudioSource::ActiveWindow => WinAudioSource::ActiveWindow,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct RecorderConfig {
  pub fps_numerator: u32,
  pub fps_denominator: u32,
  pub input_width: Option<u32>,
  pub input_height: Option<u32>,
  pub output_width: u32,
  pub output_height: u32,
  pub capture_audio: bool,
  pub capture_microphone: bool,
  pub audio_source: AudioSource,
  pub microphone_volume: Option<f64>,
  pub system_volume: Option<f64>,
  pub microphone_device: Option<String>,
  pub video_encoder_type: Option<VideoEncoderType>,
  pub video_encoder_name: Option<String>,
  pub capture_cursor: bool,
  pub output_path: String,
  pub debug_mode: bool,
  pub enable_replay_buffer: bool,
  pub replay_buffer_seconds: Option<u32>,
}

impl Default for RecorderConfig {
  fn default() -> Self {
    Self {
      fps_numerator: 30,
      fps_denominator: 1,
      input_width: None,
      input_height: None,
      output_width: 1920,
      output_height: 1080,
      capture_audio: true,
      capture_microphone: false,
      audio_source: AudioSource::Desktop,
      microphone_volume: None,
      system_volume: None,
      microphone_device: None,
      video_encoder_type: None,
      video_encoder_name: None,
      capture_cursor: true,
      output_path: "recording.mp4".to_string(),
      debug_mode: false,
      enable_replay_buffer: false,
      replay_buffer_seconds: None,
    }
  }
}

impl From<RecorderConfig> for WinRecorderConfig {
  fn from(js_config: RecorderConfig) -> Self {
    let mut builder = WinRecorderConfig::builder()
      .fps(js_config.fps_numerator, js_config.fps_denominator)
      .output_dimensions(js_config.output_width, js_config.output_height)
      .capture_audio(js_config.capture_audio)
      .capture_microphone(js_config.capture_microphone)
      .audio_source(js_config.audio_source.into())
      .capture_cursor(js_config.capture_cursor)
      .output_path(PathBuf::from(js_config.output_path))
      .debug_mode(js_config.debug_mode);

    if let Some(input_width) = js_config.input_width {
      if let Some(input_height) = js_config.input_height {
        builder = builder.input_dimensions(input_width, input_height);
      }
    }

    if let Some(volume) = js_config.microphone_volume {
      builder = builder.microphone_volume(volume as f32);
    }

    if let Some(volume) = js_config.system_volume {
      builder = builder.system_volume(volume as f32);
    }

    if let Some(device_name) = js_config.microphone_device {
      builder = builder.microphone_device(Some(device_name));
    }

    if let Some(encoder_type) = js_config.video_encoder_type {
      builder = builder.video_encoder(encoder_type.into());
    }

    if let Some(encoder_name) = js_config.video_encoder_name {
      builder = builder.video_encoder_name(encoder_name);
    }

    if js_config.enable_replay_buffer {
      builder = builder.enable_replay_buffer(true);
      if let Some(seconds) = js_config.replay_buffer_seconds {
        builder = builder.replay_buffer_seconds(seconds);
      }
    }

    builder.build()
  }
}

#[napi]
pub struct Recorder {
  recorder: WindowsRecorder,
}

#[napi]
impl Recorder {
  #[napi(constructor)]
  pub fn new(config: RecorderConfig) -> Result<Self> {
    let recorder_config: WinRecorderConfig = config.into();
    let recorder = WindowsRecorder::new(recorder_config)
      .map_err(|e| Error::from_reason(format!("Failed to create recorder: {}", e)))?;

    Ok(Self { recorder })
  }

  #[napi]
  pub fn with_process_name(&self, process_name: String) -> Result<Recorder> {
    // Create a new recorder with the same config
    let new_recorder = WindowsRecorder::new(self.recorder.config().clone())
      .map_err(|e| Error::from_reason(format!("Failed to create recorder: {}", e)))?
      .with_process_name(&process_name);
    Ok(Recorder {
      recorder: new_recorder,
    })
  }

  #[napi]
  pub fn start_recording(&self) -> Result<()> {
    self
      .recorder
      .start_recording()
      .map_err(|e| Error::from_reason(format!("Failed to start recording: {}", e)))
  }

  #[napi]
  pub fn stop_recording(&self) -> Result<()> {
    self
      .recorder
      .stop_recording()
      .map_err(|e| Error::from_reason(format!("Failed to stop recording: {}", e)))
  }

  #[napi]
  pub fn save_replay(&self, path: String) -> Result<()> {
    self
      .recorder
      .save_replay(&path)
      .map_err(|e| Error::from_reason(format!("Failed to save replay: {}", e)))
  }
}

// Audio device enumeration functions
#[napi]
pub fn enumerate_audio_input_devices() -> Result<Vec<AudioInputDevice>> {
  let devices = windows_record::enumerate_audio_input_devices()
    .map_err(|e| Error::from_reason(format!("Failed to enumerate audio devices: {}", e)))?;

  Ok(devices.into_iter().map(|d| d.into()).collect())
}

// Video encoder enumeration functions
#[napi]
pub fn enumerate_video_encoders() -> Result<Vec<VideoEncoder>> {
  let encoders = windows_record::enumerate_video_encoders()
    .map_err(|e| Error::from_reason(format!("Failed to enumerate video encoders: {}", e)))?;

  Ok(encoders.into_iter().map(|e| e.into()).collect())
}

#[napi]
pub fn get_preferred_video_encoder_by_type(
  encoder_type: VideoEncoderType,
) -> Result<Option<VideoEncoder>> {
  let encoder = windows_record::get_preferred_video_encoder_by_type(encoder_type.into());
  Ok(encoder.map(|e| e.into()))
}

// Builder pattern for easier configuration from JavaScript
#[napi]
pub struct RecorderConfigBuilder {
  config: RecorderConfig,
}

#[napi]
impl RecorderConfigBuilder {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      config: RecorderConfig::default(),
    }
  }

  #[napi]
  pub fn fps(&mut self, numerator: u32, denominator: u32) -> &Self {
    self.config.fps_numerator = numerator;
    self.config.fps_denominator = denominator;
    self
  }

  #[napi]
  pub fn input_dimensions(&mut self, width: u32, height: u32) -> &Self {
    self.config.input_width = Some(width);
    self.config.input_height = Some(height);
    self
  }

  #[napi]
  pub fn output_dimensions(&mut self, width: u32, height: u32) -> &Self {
    self.config.output_width = width;
    self.config.output_height = height;
    self
  }

  #[napi]
  pub fn capture_audio(&mut self, capture: bool) -> &Self {
    self.config.capture_audio = capture;
    self
  }

  #[napi]
  pub fn capture_microphone(&mut self, capture: bool) -> &Self {
    self.config.capture_microphone = capture;
    self
  }

  #[napi]
  pub fn audio_source(&mut self, source: AudioSource) -> &Self {
    self.config.audio_source = source;
    self
  }

  #[napi]
  pub fn microphone_volume(&mut self, volume: f64) -> &Self {
    self.config.microphone_volume = Some(volume);
    self
  }

  #[napi]
  pub fn system_volume(&mut self, volume: f64) -> &Self {
    self.config.system_volume = Some(volume);
    self
  }

  #[napi]
  pub fn microphone_device(&mut self, device_name: Option<String>) -> &Self {
    self.config.microphone_device = device_name;
    self
  }

  #[napi]
  pub fn video_encoder(&mut self, encoder_type: VideoEncoderType) -> &Self {
    self.config.video_encoder_type = Some(encoder_type);
    self
  }

  #[napi]
  pub fn video_encoder_name(&mut self, name: String) -> &Self {
    self.config.video_encoder_name = Some(name);
    self
  }

  #[napi]
  pub fn capture_cursor(&mut self, capture: bool) -> &Self {
    self.config.capture_cursor = capture;
    self
  }

  #[napi]
  pub fn output_path(&mut self, path: String) -> &Self {
    self.config.output_path = path;
    self
  }

  #[napi]
  pub fn debug_mode(&mut self, debug: bool) -> &Self {
    self.config.debug_mode = debug;
    self
  }

  #[napi]
  pub fn enable_replay_buffer(&mut self, enable: bool) -> &Self {
    self.config.enable_replay_buffer = enable;
    self
  }

  #[napi]
  pub fn replay_buffer_seconds(&mut self, seconds: u32) -> &Self {
    self.config.replay_buffer_seconds = Some(seconds);
    self
  }

  #[napi]
  pub fn build(&self) -> RecorderConfig {
    self.config.clone()
  }
}
