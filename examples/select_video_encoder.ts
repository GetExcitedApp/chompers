import {
  Recorder,
  RecorderConfigBuilder,
  VideoEncoder,
  VideoEncoderType,
  enumerateVideoEncoders,
  getPreferredVideoEncoderByType,
} from "../index";
import * as readline from "readline";

async function main(): Promise<void> {
  try {
    // Get list of available video encoders
    const encoders: VideoEncoder[] = enumerateVideoEncoders();

    console.log("Available video encoders:");
    encoders.forEach((encoder, index) => {
      console.log(`${index + 1}. ${encoder.name} (${encoder.encoderType})`);
    });

    // Prompt user to select an encoder
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    const encoderIndex = await new Promise<number>((resolve) => {
      rl.question("Enter encoder number (or 0 for default): ", (answer) => {
        const index = parseInt(answer.trim()) || 0;
        resolve(index);
      });
    });

    rl.close();

    // Get selected encoder or default
    let selectedEncoder: VideoEncoder | null = null;

    if (encoderIndex > 0 && encoderIndex <= encoders.length) {
      selectedEncoder = encoders[encoderIndex - 1];
    } else {
      // Try to get a preferred encoder, otherwise error out
      const preferredH264 = getPreferredVideoEncoderByType(
        VideoEncoderType.H264
      );
      const preferredHEVC = getPreferredVideoEncoderByType(
        VideoEncoderType.HEVC
      );

      selectedEncoder = preferredH264 || preferredHEVC;

      if (!selectedEncoder) {
        console.error("No suitable video encoders found on the system");
        process.exit(1);
      }
    }

    console.log(
      `Selected encoder: ${selectedEncoder.name} (${selectedEncoder.encoderType})`
    );

    // Parse encoder type from string
    const encoderType =
      selectedEncoder.encoderType === "H264"
        ? VideoEncoderType.H264
        : VideoEncoderType.HEVC;

    // Create a recorder with the selected encoder using builder pattern
    const config = new RecorderConfigBuilder()
      .fps(30, 1)
      .inputDimensions(1920, 1080)
      .captureAudio(true)
      .captureMicrophone(false)
      .videoEncoder(encoderType) // Use the encoder type from selected encoder
      .videoEncoderName(selectedEncoder.name) // Specify encoder by name
      .outputPath("encoder_test.mp4")
      .build();

    // Create and start the recorder
    const recorder = new Recorder(config).withProcessName("Chrome");

    console.log(`Starting recording with ${selectedEncoder.name} encoder.`);

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

    // Record for 30 seconds
    console.log("Recording for 30 seconds...");
    await new Promise((resolve) => setTimeout(resolve, 30000));

    // Stop recording
    console.log("Stopping recording");
    recorder.stopRecording();

    console.log("Recording completed successfully");
  } catch (error) {
    console.error("Error during recording:", error);
    process.exit(1);
  }
}

// Run the example
if (require.main === module) {
  main().catch(console.error);
}

export { main };
