use napi::Result;
use serde::{Deserialize, Serialize};
use windows::{
  core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::System::Threading::*,
  Win32::UI::WindowsAndMessaging::*,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct MonitorDimensions {
  pub width: i32,
  pub height: i32,
  pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct WindowInfo {
  pub class_name: String,
  pub executable: String,
  pub title: String,
  pub pid: u32,
  pub product_name: Option<String>,
  pub hwnd: i64,
  pub full_exe: String,
  pub monitor_dimensions: MonitorDimensions,
  pub intersects_multiple: bool,
  pub focused: bool,
  pub arguments: Vec<String>,
}

struct EnumWindowsData {
  windows: Vec<WindowInfo>,
  focused_hwnd: HWND,
  monitors: Vec<(HMONITOR, MonitorDimensions)>,
}

#[napi]
pub fn get_all_windows() -> Result<Vec<WindowInfo>> {
  unsafe {
    // Get the currently focused window
    let focused_hwnd = GetForegroundWindow();

    // Enumerate monitors first
    let mut monitors = Vec::new();
    let monitors_ptr = &mut monitors as *mut Vec<(HMONITOR, MonitorDimensions)>;
    let _ = EnumDisplayMonitors(
      None,
      None,
      Some(enum_monitors_proc),
      LPARAM(monitors_ptr as isize),
    );

    // Prepare data for window enumeration
    let mut data = EnumWindowsData {
      windows: Vec::new(),
      focused_hwnd,
      monitors,
    };

    let data_ptr = &mut data as *mut EnumWindowsData;

    // Enumerate all windows
    let result = EnumWindows(Some(enum_windows_proc), LPARAM(data_ptr as isize));
    if result.is_err() {
      return Err(napi::Error::from_reason("Failed to enumerate windows"));
    }

    Ok(data.windows)
  }
}

unsafe extern "system" fn enum_monitors_proc(
  hmonitor: HMONITOR,
  _hdc: HDC,
  _lprect: *mut RECT,
  lparam: LPARAM,
) -> BOOL {
  let monitors_ptr = lparam.0 as *mut Vec<(HMONITOR, MonitorDimensions)>;
  let monitors = &mut *monitors_ptr;

  let mut monitor_info = MONITORINFO {
    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
    ..Default::default()
  };

  if GetMonitorInfoW(hmonitor, &mut monitor_info) != FALSE {
    let width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
    let height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

    monitors.push((
      hmonitor,
      MonitorDimensions {
        width,
        height,
        index: monitors.len() as i32,
      },
    ));
  }

  TRUE
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
  let data_ptr = lparam.0 as *mut EnumWindowsData;
  let data = &mut *data_ptr;

  // Skip invisible windows
  if IsWindowVisible(hwnd) == FALSE {
    return TRUE;
  }

  // Get window title
  let mut title_buffer = [0u16; 512];
  let title_len = GetWindowTextW(hwnd, &mut title_buffer);
  let title = if title_len > 0 {
    String::from_utf16_lossy(&title_buffer[..title_len as usize])
  } else {
    String::new()
  };

  // Get window class name
  let mut class_buffer = [0u16; 256];
  let class_len = GetClassNameW(hwnd, &mut class_buffer);
  let class_name = if class_len > 0 {
    String::from_utf16_lossy(&class_buffer[..class_len as usize])
  } else {
    String::new()
  };

  // Get process ID
  let mut pid: u32 = 0;
  GetWindowThreadProcessId(hwnd, Some(&mut pid));

  if pid == 0 {
    return TRUE;
  }

  // Get basic process information
  let (executable, full_exe) = get_process_executable_path(pid);

  // Check if window is focused
  let focused = hwnd == data.focused_hwnd;

  // Get monitor information and check for multi-monitor spanning
  let (monitor_dimensions, intersects_multiple) = get_window_monitor_info(hwnd, &data.monitors);

  let window_info = WindowInfo {
    class_name,
    executable: executable.clone(),
    title,
    pid,
    product_name: None, // Product name extraction not implemented
    hwnd: hwnd.0 as i64,
    full_exe: full_exe.clone(),
    monitor_dimensions,
    intersects_multiple,
    focused,
    arguments: if !full_exe.is_empty() {
      vec![full_exe]
    } else {
      Vec::new()
    },
  };

  data.windows.push(window_info);

  TRUE
}

unsafe fn get_process_executable_path(pid: u32) -> (String, String) {
  let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);

  if let Ok(handle) = process_handle {
    let mut buffer = [0u16; 1024];
    let mut size = buffer.len() as u32;

    // Try QueryFullProcessImageNameW
    if QueryFullProcessImageNameW(
      handle,
      PROCESS_NAME_WIN32,
      PWSTR(buffer.as_mut_ptr()),
      &mut size,
    )
    .is_ok()
      && size > 0
    {
      let full_path = String::from_utf16_lossy(&buffer[..size as usize]);
      let executable = std::path::Path::new(&full_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();
      let _ = CloseHandle(handle);
      return (executable, full_path);
    }

    let _ = CloseHandle(handle);
  }

  (String::new(), String::new())
}

unsafe fn get_window_monitor_info(
  hwnd: HWND,
  monitors: &[(HMONITOR, MonitorDimensions)],
) -> (MonitorDimensions, bool) {
  // Check if window intersects with multiple monitors
  // MONITOR_DEFAULTTONULL returns NULL if window spans multiple monitors
  let monitor_null_check = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONULL);
  let intersects_multiple = monitor_null_check.0 as usize == 0; // NULL handle means spans multiple monitors

  // Get the primary monitor for this window
  let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);

  // Find the monitor in our enumerated list to get the correct index
  for (hmon, monitor_dims) in monitors {
    if hmon.0 == monitor.0 {
      return (monitor_dims.clone(), intersects_multiple);
    }
  }

  // Fallback: get monitor info directly if not found in our list
  let mut monitor_info = MONITORINFO {
    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
    ..Default::default()
  };

  if GetMonitorInfoW(monitor, &mut monitor_info) != FALSE {
    let width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
    let height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

    return (
      MonitorDimensions {
        width,
        height,
        index: 0, // Default to 0 if we can't find the index
      },
      intersects_multiple,
    );
  }

  // Ultimate fallback
  (
    MonitorDimensions {
      width: 1920,
      height: 1080,
      index: 0,
    },
    false,
  )
}
