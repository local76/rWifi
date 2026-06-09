#![allow(deprecated)]
//! scout: Terminal User Interface WiFi network manager for Windows.
//!
//! **Taxonomy Classification**: Application Coordinator.

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

mod config;
mod input;
mod logger;
mod wlan;
mod win32;
mod app;
mod ui;

// Re-exports for submodules
#[cfg(not(windows))]
pub use crate::wlan::windows_sys;

// Embedded markdown documentation files
pub const README_CONTENT: &str = include_str!("../README.md");
pub const SUPPORT_CONTENT: &str = include_str!("../SUPPORT.md");
pub const LICENSE_CONTENT: &str = include_str!("../LICENSE.md");
pub const COPYRIGHT_CONTENT: &str = include_str!("../COPYRIGHT.md");
pub const PRIVACY_CONTENT: &str = include_str!("../PRIVACY.md");
pub const SECURITY_CONTENT: &str = include_str!("../SECURITY.md");
pub const CONTRIBUTING_CONTENT: &str = include_str!("../CONTRIBUTING.md");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::AppConfig::load();
    win32::relaunch_in_conhost_if_needed();

    #[cfg(windows)]
    let _hwnd = win32::hide_console_at_startup();

    logger::set_event_log_enabled(config.enable_event_log);
    logger::log_message("INFO", "scout application starting up...");
    
    let _instance_guard = match win32::SingleInstanceGuard::try_new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let _title_guard = win32::ConsoleTitleGuard::new("scout");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let _ = execute!(stdout, ratatui::crossterm::terminal::SetSize(100, 35));
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let _borderless = if config.enable_borderless {
        Some(win32::BorderlessConsole::enable())
    } else {
        None
    };
    std::thread::sleep(Duration::from_millis(50)); // Allow dimensions settle delay

    if _borderless.is_none() {
        win32::center_console_window();
    }

    #[cfg(windows)]
    {
        // Re-show the console window after TUI init (parity with helm/pulse).
        unsafe extern "system" {
            fn ShowWindow(hWnd: *mut std::ffi::c_void, nCmdShow: i32) -> i32;
            fn SetForegroundWindow(hWnd: *mut std::ffi::c_void) -> i32;
        }
        let hwnd = win32::hide_console_at_startup().unwrap_or(std::ptr::null_mut());
        if !hwnd.is_null() {
            unsafe {
                ShowWindow(hwnd, 5); // SW_SHOW
                SetForegroundWindow(hwnd);
            }
        }
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::AppState::new();
    
    // Initial Scan
    app.scan_wifi(true);

    let mut last_tick = Instant::now();

    while !app.should_quit {
        app.check_scan_results();
        app.sync_power_status_if_needed();

        // Check periodic scanning
        let scan_interval = if app.on_battery {
            Duration::from_secs(12)
        } else {
            Duration::from_secs(6)
        };
        if app.last_scan.elapsed() > scan_interval && !app.show_password_overlay {
            app.scan_wifi(false);
        }

        // Render UI
        let dark_mode = match config.theme_mode.as_str() {
            "dark" => true,
            "light" => false,
            _ => win32::query_dark_mode(),
        };
        let accent_color = win32::get_dwm_accent_color();
        let theme = ui::get_theme(dark_mode, accent_color);

        terminal.draw(|f| ui::draw_ui(f, &mut app, &theme))?;

        let tick_rate = if app.on_battery {
            Duration::from_millis(500)
        } else {
            Duration::from_millis(250)
        };
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        app::keys::handle_keypress(&mut app, key.code, &theme);
                    }
                }
                Event::Mouse(mouse_event) => {
                    app::mouse::handle_mouse(&mut app, mouse_event);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if app.status_ttl > 0 {
                app.status_ttl = app.status_ttl.saturating_sub(tick_rate.as_millis() as u32);
                if app.status_ttl == 0 {
                    app.status_msg = "Ready. Press Tab to cycle focus. Press Space/r to scan. (? for help)".to_string();
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        ratatui::crossterm::cursor::Show
    )?;
    terminal.show_cursor()?;

    Ok(())
}
