import { getAllWindows, WindowInfo } from "../index";

async function main(): Promise<void> {
  console.log("=== Get All Windows Example ===");
  console.log("This example demonstrates window enumeration functionality.");
  console.log();

  try {
    // Get all windows with detailed information
    const windows: WindowInfo[] = getAllWindows();

    console.log(`Found ${windows.length} windows:`);
    console.log("=".repeat(80));

    // Display all windows with their information
    windows.forEach((window, index) => {
      console.log(`${index + 1}. ${window.title || "(No Title)"}`);
      console.log(`   Class Name: ${window.className}`);
      console.log(`   Executable: ${window.executable}`);
      console.log(`   Full Path: ${window.fullExe}`);
      console.log(`   Process ID: ${window.pid}`);
      console.log(`   Window Handle: ${window.hwnd}`);
      console.log(`   Product Name: ${window.productName || "N/A"}`);
      console.log(`   Focused: ${window.focused ? "Yes" : "No"}`);
      console.log(
        `   Monitor: ${window.monitorDimensions.width}x${window.monitorDimensions.height}`
      );
      console.log(
        `   Spans Multiple Monitors: ${
          window.intersectsMultiple ? "Yes" : "No"
        }`
      );

      if (window.arguments && window.arguments.length > 0) {
        console.log(`   Arguments: ${window.arguments.join(" ")}`);
      } else {
        console.log(`   Arguments: None`);
      }

      console.log("-".repeat(80));
    });

    // Filter and display focused windows
    const focusedWindows = windows.filter((w) => w.focused);
    console.log(`\nFocused Windows (${focusedWindows.length}):`);
    focusedWindows.forEach((window) => {
      console.log(`  • ${window.title} (${window.executable})`);
    });

    // Filter and display windows by common applications
    const commonApps = [
      "chrome",
      "firefox",
      "notepad",
      "code",
      "discord",
      "steam",
    ];
    console.log("\nCommon Applications Found:");

    commonApps.forEach((appName) => {
      const appWindows = windows.filter(
        (w) =>
          w.executable.toLowerCase().includes(appName.toLowerCase()) ||
          w.title.toLowerCase().includes(appName.toLowerCase())
      );

      if (appWindows.length > 0) {
        console.log(`  ${appName.toUpperCase()}:`);
        appWindows.forEach((window) => {
          console.log(`    - ${window.title} (PID: ${window.pid})`);
        });
      }
    });

    // Display monitor information
    const uniqueMonitors = new Map<string, any>();
    windows.forEach((window) => {
      const monitorKey = `${window.monitorDimensions.width}x${window.monitorDimensions.height}`;
      if (!uniqueMonitors.has(monitorKey)) {
        uniqueMonitors.set(monitorKey, window.monitorDimensions);
      }
    });

    console.log(`\nDetected Monitors (${uniqueMonitors.size}):`);
    uniqueMonitors.forEach((monitor, resolution) => {
      console.log(
        `  • ${resolution} - Primary: ${monitor.isPrimary ? "Yes" : "No"}`
      );
    });

    // Display windows that span multiple monitors
    const multiMonitorWindows = windows.filter((w) => w.intersectsMultiple);
    if (multiMonitorWindows.length > 0) {
      console.log(
        `\nWindows Spanning Multiple Monitors (${multiMonitorWindows.length}):`
      );
      multiMonitorWindows.forEach((window) => {
        console.log(`  • ${window.title} (${window.executable})`);
      });
    }

    // Display statistics
    console.log("\n" + "=".repeat(80));
    console.log("STATISTICS:");
    console.log(`Total Windows: ${windows.length}`);
    console.log(`Focused Windows: ${focusedWindows.length}`);
    console.log(`Multi-Monitor Windows: ${multiMonitorWindows.length}`);
    console.log(`Unique Monitors: ${uniqueMonitors.size}`);

    const uniqueProcesses = new Set(windows.map((w) => w.executable)).size;
    console.log(`Unique Processes: ${uniqueProcesses}`);

    const windowsWithTitles = windows.filter(
      (w) => w.title && w.title.trim().length > 0
    ).length;
    console.log(`Windows with Titles: ${windowsWithTitles}`);
  } catch (error) {
    console.error("Error enumerating windows:", error);
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
