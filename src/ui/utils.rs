//! Layout and QR generation rendering helpers.
//!
//! **Taxonomy Classification**: UI Rendering (UI Utilities).

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout helper to get a centered percentage rect.
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Fixed-size layout helper to center a popup.
pub fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(r.width),
        height: height.min(r.height),
    }
}

/// Text wrapping helper for simple paragraph blocks.
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut words = text.split_whitespace();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    while let Some(word) = words.next() {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

/// Generates a payload-ready QR code matching WiFi credentials format:
/// `WIFI:T:WPA;S:SSID;P:PASSWORD;;`
pub fn generate_qr_code_lines(ssid: &str, password: &str, auth_type: &str) -> Vec<String> {
    use qrcodegen::{QrCode, QrCodeEcc};
    let auth = match auth_type {
        "WPA2-Personal" | "WPA-Personal" | "WPA3-Personal" | "WPA2-Enterprise" | "WPA3-Enterprise" | "WPA2PSK" | "WPAPSK" | "WPA3SAE" => "WPA",
        "Open" | "none" => "nopass",
        "WEP" => "WEP",
        _ => "WPA",
    };
    let payload = if auth == "nopass" {
        format!("WIFI:T:nopass;S:{};;", ssid)
    } else {
        format!("WIFI:T:{};S:{};P:{};;", auth, ssid, password)
    };
    
    let qr = match QrCode::encode_text(&payload, QrCodeEcc::Medium) {
        Ok(code) => code,
        Err(_) => return vec!["Failed to generate QR code".to_string()],
    };
    
    let size = qr.size();
    let border = 2; // quiet zone
    let total_size = size + border * 2;
    
    let mut lines = Vec::new();
    
    for y_idx in (0..total_size).step_by(2) {
        let mut line = String::new();
        for x_idx in 0..total_size {
            let x = x_idx as i32 - border;
            let y1 = y_idx as i32 - border;
            let y2 = y_idx as i32 + 1 - border;
            
            let val1 = if x >= 0 && x < size && y1 >= 0 && y1 < size {
                qr.get_module(x, y1)
            } else {
                false
            };
            
            let val2 = if x >= 0 && x < size && y2 >= 0 && y2 < size {
                qr.get_module(x, y2)
            } else {
                false
            };
            
            let ch = match (val1, val2) {
                (true, true) => "█",
                (true, false) => "▀",
                (false, true) => "▄",
                (false, false) => " ",
            };
            line.push_str(ch);
        }
        lines.push(line);
    }
    
    lines
}
