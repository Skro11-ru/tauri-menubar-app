use std::sync::OnceLock;

use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

struct TrayIconBase {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

static BASE_TRAY_ICON: OnceLock<TrayIconBase> = OnceLock::new();

fn base_tray_icon() -> &'static TrayIconBase {
    BASE_TRAY_ICON.get_or_init(|| {
        let icon =
            Image::from_bytes(include_bytes!("../icons/32x32.png")).unwrap_or_else(|_| {
                panic!("tray icon image is missing or invalid");
            });

        TrayIconBase {
            rgba: icon.rgba().to_vec(),
            width: icon.width(),
            height: icon.height(),
        }
    })
}

fn set_pixel(pixels: &mut [u8], width: u32, height: u32, x: u32, y: u32, rgba: [u8; 4]) {
    if x >= width || y >= height {
        return;
    }

    let index = ((y * width + x) * 4) as usize;
    if index + 4 > pixels.len() {
        return;
    }

    pixels[index..index + 4].copy_from_slice(&rgba);
}

fn glyph_for_char(character: char) -> Option<[u8; 5]> {
    match character {
        '0' => Some([0b111, 0b101, 0b101, 0b101, 0b111]),
        '1' => Some([0b010, 0b110, 0b010, 0b010, 0b111]),
        '2' => Some([0b111, 0b001, 0b111, 0b100, 0b111]),
        '3' => Some([0b111, 0b001, 0b111, 0b001, 0b111]),
        '4' => Some([0b101, 0b101, 0b111, 0b001, 0b001]),
        '5' => Some([0b111, 0b100, 0b111, 0b001, 0b111]),
        '6' => Some([0b111, 0b100, 0b111, 0b101, 0b111]),
        '7' => Some([0b111, 0b001, 0b010, 0b010, 0b010]),
        '8' => Some([0b111, 0b101, 0b111, 0b101, 0b111]),
        '9' => Some([0b111, 0b101, 0b111, 0b001, 0b111]),
        '+' => Some([0b010, 0b010, 0b111, 0b010, 0b010]),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_glyph(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    origin_x: u32,
    origin_y: u32,
    scale: u32,
    glyph: [u8; 5],
    rgba: [u8; 4],
) {
    for (row_index, row_bits) in glyph.iter().enumerate() {
        for column_index in 0..3 {
            let bit = 1 << (2 - column_index);
            if row_bits & bit == 0 {
                continue;
            }

            let pixel_x = origin_x + (column_index as u32 * scale);
            let pixel_y = origin_y + (row_index as u32 * scale);

            for delta_y in 0..scale {
                for delta_x in 0..scale {
                    let x = pixel_x + delta_x;
                    let y = pixel_y + delta_y;
                    if x < width && y < height {
                        set_pixel(pixels, width, height, x, y, rgba);
                    }
                }
            }
        }
    }
}

fn badge_text(count: u32) -> String {
    if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    }
}

fn tray_icon_with_badge(count: u32) -> Image<'static> {
    let base = base_tray_icon();
    let mut pixels = base.rgba.clone();

    if count > 0 {
        let text = badge_text(count);
        let badge_size = (base.width.min(base.height) / 2).max(18);
        let badge_origin_x = base.width.saturating_sub(badge_size + 1);
        let badge_origin_y = 1;
        let badge_radius = badge_size / 2;
        let center_x = badge_origin_x + badge_radius;
        let center_y = badge_origin_y + badge_radius;
        let radius_squared = i64::from(badge_radius) * i64::from(badge_radius);

        for y in badge_origin_y..(badge_origin_y + badge_size).min(base.height) {
            for x in badge_origin_x..(badge_origin_x + badge_size).min(base.width) {
                let dx = i64::from(x) - i64::from(center_x);
                let dy = i64::from(y) - i64::from(center_y);
                if dx * dx + dy * dy <= radius_squared {
                    set_pixel(
                        &mut pixels,
                        base.width,
                        base.height,
                        x,
                        y,
                        [220, 53, 69, 255],
                    );
                }
            }
        }

        let characters: Vec<char> = text.chars().collect();
        let scale = match characters.len() {
            1 => 3,
            2 => 2,
            _ => 1,
        };
        let glyph_width = 3 * scale;
        let glyph_height = 5 * scale;
        let gap = scale;
        let text_width =
            characters.len() as u32 * glyph_width + characters.len().saturating_sub(1) as u32 * gap;
        let text_height = glyph_height;
        let text_origin_x = badge_origin_x + (badge_size.saturating_sub(text_width)) / 2;
        let text_origin_y = badge_origin_y + (badge_size.saturating_sub(text_height)) / 2;

        for (index, character) in characters.into_iter().enumerate() {
            if let Some(glyph) = glyph_for_char(character) {
                let glyph_origin_x = text_origin_x + index as u32 * (glyph_width + gap);
                draw_glyph(
                    &mut pixels,
                    base.width,
                    base.height,
                    glyph_origin_x,
                    text_origin_y,
                    scale,
                    glyph,
                    [255, 255, 255, 255],
                );
            }
        }
    }

    Image::new_owned(pixels, base.width, base.height)
}

pub fn sync_counter_indicators<R: tauri::Runtime>(
    app: &AppHandle<R>,
    count: u32,
) -> Result<(), String> {
    if let Some(tray) = app.tray_by_id("main") {
        let icon = tray_icon_with_badge(count);
        tray.set_icon(Some(icon))
            .map_err(|e| format!("Failed to set tray icon: {e}"))?;
        tray.set_tooltip(Some(format!("Счётчик: {count}")))
            .map_err(|e| format!("Failed to set tray tooltip: {e}"))?;
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_badge_count((count > 0).then_some(i64::from(count)));
    }

    Ok(())
}

pub fn setup_tray<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    let toggle_item = MenuItem::new(app, "Toggle", true, None::<&str>)?;
    let quit_item = MenuItem::new(app, "Quit", true, None::<&str>)?;
    let toggle_id = toggle_item.id().clone();
    let quit_id = quit_item.id().clone();
    let menu = MenuBuilder::new(app)
        .item(&toggle_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or("Default window icon is missing")?;

    TrayIconBuilder::with_id("main")
        .menu(&menu)
        .icon(icon)
        .icon_as_template(false)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app_handle, event| {
            if event.id == toggle_id {
                crate::window::toggle_main_window(app_handle);
            } else if event.id == quit_id {
                app_handle.exit(0);
            }
        })
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
            ) {
                crate::window::toggle_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    sync_counter_indicators(app, 0)?;

    Ok(())
}
