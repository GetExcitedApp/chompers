# Chompers

A high-performance native Node.js addon for enumerating and analyzing windows on Windows systems. Built with Rust and NAPI-RS for optimal performance and memory safety.

## Features

- **Fast Window Enumeration**: Efficiently enumerate all visible windows on the system
- **Detailed Window Information**: Get comprehensive details about each window including:
  - Window title and class name
  - Process ID and executable path
  - Monitor information and dimensions
  - Focus state
  - Multi-monitor detection
  - Command line arguments
- **Multi-Monitor Support**: Detect windows spanning multiple monitors
- **Native Performance**: Built with Rust for maximum performance
- **Type Safety**: Full TypeScript support with detailed type definitions

## Installation
```bash
npm install @getexcited/chompers
```

## Usage

### Basic Window Enumeration

```javascript
import { getAllWindows } from '@getexcited/chompers';

// Get all visible windows
const windows = getAllWindows();

console.log(`Found ${windows.length} windows`);

// Print details of each window
windows.forEach((window, index) => {
  console.log(`Window ${index + 1}:`);
  console.log(`  Title: "${window.title}"`);
  console.log(`  Executable: ${window.executable}`);
  console.log(`  PID: ${window.pid}`);
  console.log(`  Focused: ${window.focused}`);
  console.log(`  Monitor: ${window.monitorDimensions.width}x${window.monitorDimensions.height}`);
  console.log(`  Spans Multiple Monitors: ${window.intersectsMultiple}`);
});
```

### Finding Specific Windows

```javascript
import { getAllWindows } from '@getexcited/chompers';

const windows = getAllWindows();

// Find all browser windows
const browsers = windows.filter(w => 
  w.executable.includes('chrome') || 
  w.executable.includes('firefox') || 
  w.executable.includes('edge')
);

// Find the currently focused window
const focusedWindow = windows.find(w => w.focused);

// Find windows on a specific monitor
const primaryMonitorWindows = windows.filter(w => 
  w.monitorDimensions.index === 0
);

// Find windows spanning multiple monitors
const multiMonitorWindows = windows.filter(w => w.intersectsMultiple);
```

### Game Detection Example

```javascript
import { getAllWindows } from '@getexcited/chompers';

const gameExecutables = [
  'valorant.exe',
  'csgo.exe',
  'fortnite.exe',
  'minecraft.exe',
  // Add more game executables
];

const windows = getAllWindows();
const gameWindows = windows.filter(window => 
  gameExecutables.some(game => 
    window.executable.toLowerCase().includes(game.toLowerCase())
  )
);

if (gameWindows.length > 0) {
  console.log('Detected games:');
  gameWindows.forEach(game => {
    console.log(`- ${game.title} (${game.executable})`);
  });
}
```

## API Reference

### `getAllWindows(): WindowInfo[]`

Returns an array of all visible windows on the system.

#### WindowInfo Interface

```typescript
interface WindowInfo {
  className: string;           // Window class name
  executable: string;          // Executable filename (e.g., "notepad.exe")
  title: string;              // Window title
  pid: number;                // Process ID
  productName: string | null; // Product name (currently not implemented)
  hwnd: number;               // Window handle
  fullExe: string;            // Full path to executable
  monitorDimensions: MonitorDimensions; // Monitor information
  intersectsMultiple: boolean; // True if window spans multiple monitors
  focused: boolean;           // True if window is currently focused
  arguments: string[];        // Command line arguments
}

interface MonitorDimensions {
  width: number;              // Monitor width in pixels
  height: number;             // Monitor height in pixels
  index: number;              // Monitor index (0-based)
}
```

## Use Cases

- **Game Detection**: Automatically detect running games for streaming software
- **Window Management**: Build custom window managers or productivity tools
- **System Monitoring**: Monitor application usage and window states
- **Multi-Monitor Setup**: Manage windows across multiple displays
- **Automation**: Automate window-related tasks and workflows

## Performance

Chompers is built with Rust and uses native Windows APIs for optimal performance:

- **Fast Enumeration**: Efficiently processes hundreds of windows
- **Low Memory Usage**: Minimal memory footprint
- **Native Speed**: No overhead from interpreted languages
- **Thread Safe**: Safe for use in multi-threaded Node.js applications

## Platform Support

- **Windows**: Full support (Windows 10/11 recommended)
- **macOS**: Not supported
- **Linux**: Not supported

## Requirements

- Node.js 16 or higher
- Windows 10 or higher

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

GNU General Public License v3.0 - see LICENSE file for details.

 
