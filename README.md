# @getexcited/chompers

High-performance native Node.js addon for Windows screen recording and window enumeration, built with Rust and napi-rs.

## Features

### 🎥 Screen Recording

- **High-performance recording** using Windows Desktop Duplication API
- **No yellow border** during capture (unlike WGC API)
- **Hardware-accelerated encoding** (H.264, HEVC)
- **Audio capture** from desktop and microphone
- **Replay buffer** functionality (similar to ShadowPlay)
- **Process-specific recording** target specific applications
- **Configurable quality** settings and output formats

### 🪟 Window Management

- **Window enumeration** with detailed metadata
- **Multi-monitor support** with monitor information
- **Process information** including executable paths and arguments
- **Focus detection** and window state tracking
- **Game detection** capabilities

## Installation

```bash
npm install @getexcited/chompers
# or
yarn add @getexcited/chompers
```

## Quick Start

### Screen Recording

```typescript
import {
  Recorder,
  RecorderConfig,
  AudioSource,
  VideoEncoderType,
} from "@getexcited/chompers";

// Basic recording configuration
const config: RecorderConfig = {
  fpsNumerator: 30,
  fpsDenominator: 1,
  outputWidth: 1920,
  outputHeight: 1080,
  captureAudio: true,
  captureMicrophone: false,
  audioSource: AudioSource.Desktop,
  videoEncoderType: VideoEncoderType.H264,
  captureCursor: true,
  outputPath: "recording.mp4",
  debugMode: false,
  enableReplayBuffer: false,
};

// Create recorder and target a specific process
const recorder = new Recorder(config).withProcessName("notepad");

// Start recording
recorder.startRecording();

// Record for 10 seconds
setTimeout(() => {
  recorder.stopRecording();
  console.log("Recording completed!");
}, 10000);
```

### Replay Buffer

```typescript
import {
  Recorder,
  RecorderConfigBuilder,
  AudioSource,
} from "@getexcited/chompers";

// Create recorder with replay buffer
const config = new RecorderConfigBuilder()
  .fps(30, 1)
  .outputDimensions(1920, 1080)
  .captureAudio(true)
  .audioSource(AudioSource.Desktop)
  .enableReplayBuffer(true)
  .replayBufferSeconds(30) // Keep last 30 seconds
  .outputPath("main_recording.mp4")
  .build();

const recorder = new Recorder(config).withProcessName("game.exe");

// Start recording with buffer
recorder.startRecording();

// Save replay when something interesting happens
recorder.saveReplay("highlight.mp4");

// Continue recording or stop
recorder.stopRecording();
```

### Window Enumeration

```typescript
import { getAllWindows } from "@getexcited/chompers";

// Get all windows with detailed information
const windows = getAllWindows();

windows.forEach((window) => {
  console.log(`Title: ${window.title}`);
  console.log(`Process: ${window.executable}`);
  console.log(`PID: ${window.pid}`);
  console.log(`Focused: ${window.focused}`);
  console.log(
    `Monitor: ${window.monitorDimensions.width}x${window.monitorDimensions.height}`
  );
  console.log("---");
});
```

## API Reference

### Recording Classes

#### `Recorder`

Main recording class for capturing screen content.

```typescript
class Recorder {
  constructor(config: RecorderConfig);
  withProcessName(processName: string): Recorder;
  startRecording(): void;
  stopRecording(): void;
  saveReplay(path: string): void;
}
```

#### `RecorderConfigBuilder`

Fluent builder for creating recorder configurations.

```typescript
class RecorderConfigBuilder {
  fps(numerator: number, denominator: number): this;
  inputDimensions(width: number, height: number): this;
  outputDimensions(width: number, height: number): this;
  captureAudio(capture: boolean): this;
  captureMicrophone(capture: boolean): this;
  audioSource(source: AudioSource): this;
  microphoneVolume(volume: number): this;
  systemVolume(volume: number): this;
  microphoneDevice(deviceName?: string): this;
  videoEncoder(encoderType: VideoEncoderType): this;
  videoEncoderName(name: string): this;
  captureCursor(capture: boolean): this;
  outputPath(path: string): this;
  debugMode(debug: boolean): this;
  enableReplayBuffer(enable: boolean): this;
  replayBufferSeconds(seconds: number): this;
  build(): RecorderConfig;
}
```

### Configuration Types

#### `RecorderConfig`

```typescript
interface RecorderConfig {
  fpsNumerator: number; // Frame rate numerator
  fpsDenominator: number; // Frame rate denominator
  inputWidth?: number; // Input resolution width (auto-detected if not set)
  inputHeight?: number; // Input resolution height (auto-detected if not set)
  outputWidth: number; // Output resolution width
  outputHeight: number; // Output resolution height
  captureAudio: boolean; // Enable system audio capture
  captureMicrophone: boolean; // Enable microphone capture
  audioSource: AudioSource; // Desktop or ActiveWindow
  microphoneVolume?: number; // Microphone volume (0.0-1.0)
  systemVolume?: number; // System audio volume (0.0-1.0)
  microphoneDevice?: string; // Specific microphone device name
  videoEncoderType?: VideoEncoderType; // H264 or HEVC
  videoEncoderName?: string; // Specific encoder name
  captureCursor: boolean; // Include cursor in recording
  outputPath: string; // Output file path
  debugMode: boolean; // Enable debug logging
  enableReplayBuffer: boolean; // Enable replay buffer feature
  replayBufferSeconds?: number; // Replay buffer duration in seconds
}
```

#### `WindowInfo`

```typescript
interface WindowInfo {
  className: string; // Window class name
  executable: string; // Executable name
  title: string; // Window title
  pid: number; // Process ID
  productName?: string; // Product name from executable
  hwnd: number; // Window handle
  fullExe: string; // Full executable path
  monitorDimensions: MonitorDimensions; // Monitor information
  intersectsMultiple: boolean; // Spans multiple monitors
  focused: boolean; // Currently focused window
  arguments: Array<string>; // Process arguments
}
```

### Enums

#### `AudioSource`

```typescript
enum AudioSource {
  Desktop = "Desktop", // Capture all system audio
  ActiveWindow = "ActiveWindow", // Capture audio from target window only
}
```

#### `VideoEncoderType`

```typescript
enum VideoEncoderType {
  H264 = "H264", // H.264/AVC encoding
  HEVC = "HEVC", // H.265/HEVC encoding
}
```

### Utility Functions

#### Audio Device Management

```typescript
// Enumerate available audio input devices
function enumerateAudioInputDevices(): AudioInputDevice[];

// Get preferred video encoder by type
function getPreferredVideoEncoderByType(
  encoderType: VideoEncoderType
): VideoEncoder | null;

// Enumerate available video encoders
function enumerateVideoEncoders(): VideoEncoder[];
```

#### Window Management

```typescript
// Get all windows with detailed information
function getAllWindows(): WindowInfo[];
```

## Examples

The `examples/` directory contains comprehensive TypeScript examples:

- **`basic_recording.ts`** - Basic screen recording setup
- **`replay_buffer.ts`** - Replay buffer functionality
- **`select_audio_device.ts`** - Audio device enumeration and selection
- **`select_video_encoder.ts`** - Video encoder enumeration and selection

Run examples with:

```bash
# Show available examples
yarn examples

# Run specific examples
yarn example:basic
yarn example:replay
yarn example:audio
yarn example:encoder
```

## System Requirements

- **Operating System**: Windows 10/11 (x64)
- **Node.js**: 16.0.0 or higher
- **Hardware**: GPU with hardware encoding support (recommended)
- **Drivers**: Up-to-date GPU drivers for optimal performance

## Performance Considerations

### Optimal Settings

- **Resolution**: 1920x1080 or lower for best performance
- **Frame Rate**: 30 FPS provides good quality/performance balance
- **Encoder**: H.264 for compatibility, HEVC for better compression
- **Audio**: Disable microphone if not needed

### Hardware Acceleration

The library automatically uses hardware-accelerated encoding when available:

- **NVIDIA**: NVENC (H.264/HEVC)
- **AMD**: VCE (H.264/HEVC)
- **Intel**: Quick Sync Video (H.264/HEVC)

## Troubleshooting

### Common Issues

1. **Recording fails to start**

   - Ensure target process exists
   - Check output path permissions
   - Verify sufficient disk space

2. **No audio captured**

   - Check Windows audio permissions
   - Verify target application produces audio
   - Try different audio sources

3. **Poor performance**

   - Lower output resolution
   - Reduce frame rate
   - Update GPU drivers
   - Close unnecessary applications

4. **No video encoders found**
   - Update GPU drivers
   - Install Windows Media Feature Pack
   - Try different encoder types

### Debug Mode

Enable debug logging for troubleshooting:

```typescript
const config: RecorderConfig = {
  // ... other options
  debugMode: true,
};
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/GetExcitedApp/chompers.git
cd chompers

# Install dependencies
yarn install

# Build the native addon
yarn build

# Run tests
yarn test
```

## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to the main repository.

## License

This project is licensed under the GPL-3.0-only License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [napi-rs](https://napi.rs/) for Node.js native addon development
- Uses [windows-record](https://github.com/judehek/windows-record) for core recording functionality
- Powered by Windows Desktop Duplication API for efficient screen capture

---

**Note**: This library is Windows-only and requires Windows 10 or later. For cross-platform solutions, consider other recording libraries.
