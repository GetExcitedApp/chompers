import {
  Recorder,
  RecorderConfigBuilder,
  AudioInputDevice,
  enumerateAudioInputDevices,
} from "../index";
import * as readline from "readline";

async function main(): Promise<void> {
  console.log("OS:", process.platform);
  console.log("Architecture:", process.arch);
  console.log("Application started");

  try {
    // Get list of available audio input devices
    const devices: AudioInputDevice[] = enumerateAudioInputDevices();

    console.log("Available audio input devices:");
    devices.forEach((device, index) => {
      console.log(`${index + 1}. ${device.name}`);
    });

    // Prompt user to select a device
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    const deviceIndex = await new Promise<number>((resolve) => {
      rl.question("Enter device number (or 0 for default): ", (answer) => {
        const index = parseInt(answer.trim()) || 0;
        resolve(index);
      });
    });

    rl.close();

    // Get selected device or undefined for default
    const selectedDevice: AudioInputDevice | undefined =
      deviceIndex > 0 && deviceIndex <= devices.length
        ? devices[deviceIndex - 1]
        : undefined;

    if (selectedDevice) {
      console.log(`Selected device: ${selectedDevice.name}`);
    } else {
      console.log("Using default device");
    }

    // Create a recorder with the selected device using builder pattern
    const config = new RecorderConfigBuilder()
      .fps(30, 1)
      .inputDimensions(1920, 1080)
      .outputDimensions(1920, 1080)
      .captureAudio(true)
      .captureMicrophone(true)
      .microphoneDevice(selectedDevice?.name)
      .outputPath("device.mp4")
      .build();

    // Create and start the recorder
    const recorder = new Recorder(config).withProcessName("Chrome");

    console.log(
      `Starting recording with${
        selectedDevice ? " selected" : " default"
      } microphone device.`
    );

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
