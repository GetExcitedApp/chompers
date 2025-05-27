import {
  Recorder,
  RecorderConfig,
  AudioSource,
  VideoEncoderType,
} from "../index";

async function main(): Promise<void> {
  console.log("=== Basic Recording Example ===");
  console.log(
    "This example demonstrates basic screen recording functionality."
  );

  try {
    // Create recorder configuration
    const config: RecorderConfig = {
      fpsNumerator: 30,
      fpsDenominator: 1,
      // Input dimensions will be auto-detected from monitor resolution
      inputWidth: undefined,
      inputHeight: undefined,
      outputWidth: 1920,
      outputHeight: 1080,
      captureAudio: true,
      captureMicrophone: false,
      audioSource: AudioSource.Desktop,
      microphoneVolume: 1.0,
      systemVolume: 1.0,
      microphoneDevice: undefined,
      videoEncoderType: VideoEncoderType.H264,
      videoEncoderName: undefined,
      captureCursor: false,
      outputPath: "output.mp4",
      debugMode: true,
      enableReplayBuffer: false,
      replayBufferSeconds: undefined,
    };

    // Create the recorder with your target window name
    const recorder = new Recorder(config).withProcessName("Minecraft");

    // Short delay before starting recording
    console.log("Waiting 1 second before starting recording...");
    await new Promise((resolve) => setTimeout(resolve, 1000));

    console.log("Starting recording");

    // Start recording
    try {
      recorder.startRecording();
      console.log("Recording started successfully");
    } catch (error) {
      console.error("Failed to start recording:", error);
      return;
    }

    // Record for 10 seconds
    console.log("Recording for 10 seconds...");
    await new Promise((resolve) => setTimeout(resolve, 10000));

    // Stop recording
    console.log("Stopping recording");
    recorder.stopRecording();

    console.log("Application finished - all resources properly cleaned up");
  } catch (error) {
    console.error("Error during recording:", error);
    if (typeof process !== "undefined") {
      process.exit(1);
    }
  }
}

// Run the example
if (
  typeof require !== "undefined" &&
  typeof module !== "undefined" &&
  require.main === module
) {
  main().catch(console.error);
}

export { main };
