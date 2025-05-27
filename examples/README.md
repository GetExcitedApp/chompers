# Windows Record Examples

This directory contains TypeScript examples demonstrating how to use the `@getexcited/chompers` library for screen recording on Windows. These examples are TypeScript ports of the original Rust examples from the `windows-record` library.

## Prerequisites

- Windows operating system
- Node.js 16 or higher
- TypeScript (installed as dev dependency)
- Built native addon (run `yarn build` first)

## Building the Project

Before running any examples, make sure to build the native addon:

```bash
yarn build
```

## Examples

### 1. Basic Recording (`basic_recording.ts`)

Demonstrates the fundamental screen recording functionality.

**Features:**

- Basic screen recording setup
- Audio capture from desktop
- 10-second recording duration
- Process-specific recording (targets "League of Legends")

**Usage:**

```bash
npx tsx examples/basic_recording.ts
```

**Configuration highlights:**

- 30 FPS recording
- 1920x1080 output resolution
- Desktop audio capture
- H.264 video encoding
- Debug mode enabled

### 2. Replay Buffer (`replay_buffer.ts`)

Shows how to use the replay buffer feature for continuous background recording with on-demand saving.

**Features:**

- Continuous background recording
- 30-second replay buffer
- Interactive controls (S to save, Q to quit)
- Multiple replay saves

**Usage:**

```bash
npx tsx examples/replay_buffer.ts
```

**Interactive commands:**

- `S` + Enter: Save the last 30 seconds as a replay file
- `Q` + Enter: Stop recording and quit

**Use cases:**

- Gaming highlights capture
- Security monitoring
- Event-based recording

### 3. Audio Device Selection (`select_audio_device.ts`)

Demonstrates how to enumerate and select specific audio input devices.

**Features:**

- Audio device enumeration
- Interactive device selection
- Microphone recording with selected device
- Builder pattern configuration

**Usage:**

```bash
npx tsx examples/select_audio_device.ts
```

**Workflow:**

1. Lists all available audio input devices
2. Prompts user to select a device (or use default)
3. Records with the selected microphone device
4. Targets Chrome browser window

### 4. Video Encoder Selection (`select_video_encoder.ts`)

Shows how to enumerate and select specific video encoders for recording.

**Features:**

- Video encoder enumeration
- Interactive encoder selection
- Fallback to preferred encoders
- 30-second recording duration

**Usage:**

```bash
npx tsx examples/select_video_encoder.ts
```

**Workflow:**

1. Lists all available video encoders (H.264, HEVC)
2. Prompts user to select an encoder
3. Falls back to system preferred encoder if none selected
4. Records with the selected encoder

## Common Configuration Options

### RecorderConfig Interface

```typescript
interface RecorderConfig {
  fpsNumerator: number; // Frame rate numerator (e.g., 30)
  fpsDenominator: number; // Frame rate denominator (e.g., 1)
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

### Builder Pattern

You can also use the `RecorderConfigBuilder` for a more fluent configuration:

```typescript
const config = new RecorderConfigBuilder()
  .fps(30, 1)
  .outputDimensions(1920, 1080)
  .captureAudio(true)
  .captureMicrophone(false)
  .outputPath("my_recording.mp4")
  .build();
```

## Audio Sources

- `AudioSource.Desktop`: Captures all system audio
- `AudioSource.ActiveWindow`: Captures audio only from the target window

## Video Encoder Types

- `VideoEncoderType.H264`: Standard H.264 encoding (widely compatible)
- `VideoEncoderType.HEVC`: H.265/HEVC encoding (better compression, newer)

## Error Handling

All examples include proper error handling for common scenarios:

- Failed to start recording
- No suitable encoders found
- Audio device enumeration failures
- Recording interruption

## Performance Tips

1. **Hardware Acceleration**: The library uses hardware-accelerated encoding when available
2. **Resolution**: Lower output resolutions improve performance
3. **Frame Rate**: 30 FPS is a good balance between quality and performance
4. **Audio**: Disable microphone capture if not needed to reduce CPU usage
5. **Debug Mode**: Disable debug mode in production for better performance

## Troubleshooting

### Common Issues

1. **"No suitable video encoders found"**

   - Ensure your GPU drivers are up to date
   - Try different encoder types (H.264 vs HEVC)

2. **"Failed to start recording"**

   - Check if the target process/window exists
   - Ensure sufficient disk space
   - Verify output path is writable

3. **Audio not captured**

   - Check Windows audio permissions
   - Verify the target application is producing audio
   - Try different audio sources (Desktop vs ActiveWindow)

4. **Poor performance**
   - Lower the output resolution
   - Reduce frame rate
   - Disable cursor capture if not needed
   - Close unnecessary applications

### Debug Mode

Enable debug mode in the configuration to get detailed logging:

```typescript
const config: RecorderConfig = {
  // ... other options
  debugMode: true,
};
```

## License

These examples are part of the `@getexcited/chompers` project and are licensed under GPL-3.0-only.
