import { Recorder, RecorderConfig, AudioSource } from "../index";
import * as readline from "readline";

async function main(): Promise<void> {
  console.log("=== Replay Buffer Example ===");
  console.log("This example demonstrates the replay buffer functionality.");
  console.log("- It will start a recording with the replay buffer enabled.");
  console.log(
    "- Press 'S' and 'return' to save the last 30 seconds as a replay."
  );
  console.log("- Press 'Q' and 'return' to quit the program.");

  // Get process name to record
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  const processName = await new Promise<string>((resolve) => {
    rl.question(
      "Enter process name to record (e.g., 'notepad'): ",
      (answer) => {
        resolve(answer.trim());
      }
    );
  });

  if (!processName) {
    console.log("No process name provided. Exiting.");
    rl.close();
    return;
  }

  try {
    // Create recorder with replay buffer enabled
    const config: RecorderConfig = {
      fpsNumerator: 30,
      fpsDenominator: 1,
      inputWidth: undefined,
      inputHeight: undefined,
      outputWidth: 1920,
      outputHeight: 1080,
      captureAudio: true,
      captureMicrophone: true,
      audioSource: AudioSource.Desktop,
      microphoneVolume: undefined,
      systemVolume: undefined,
      microphoneDevice: undefined,
      videoEncoderType: undefined,
      videoEncoderName: undefined,
      captureCursor: true,
      outputPath: "recording.mp4",
      debugMode: false,
      enableReplayBuffer: true,
      replayBufferSeconds: 30, // Keep last 30 seconds in buffer
    };

    const recorder = new Recorder(config).withProcessName(processName);

    console.log("Starting recording with replay buffer enabled...");
    recorder.startRecording();

    console.log("Recording started! Press 'S' to save replay, 'Q' to quit.");

    // Setup input handling for keypresses
    let replayCount = 0;
    let running = true;

    const handleInput = (input: string) => {
      const command = input.trim().toUpperCase();

      switch (command) {
        case "S":
          replayCount++;
          const replayPath = `replay_${replayCount}.mp4`;
          try {
            recorder.saveReplay(replayPath);
            console.log(`Replay saved to: ${replayPath}`);
          } catch (error) {
            console.error("Failed to save replay:", error);
          }
          break;

        case "Q":
          console.log("Stopping recording...");
          try {
            recorder.stopRecording();
            console.log("Recording stopped successfully.");
          } catch (error) {
            console.error("Error stopping recording:", error);
          }
          running = false;
          rl.close();
          break;

        default:
          console.log(
            "Unknown command. Press 'S' to save replay, 'Q' to quit."
          );
          break;
      }
    };

    rl.on("line", handleInput);

    // Keep the process alive until user quits
    while (running) {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
  } catch (error) {
    console.error("Error during recording:", error);
    rl.close();
    process.exit(1);
  }
}

// Run the example
if (require.main === module) {
  main().catch(console.error);
}

export { main };
