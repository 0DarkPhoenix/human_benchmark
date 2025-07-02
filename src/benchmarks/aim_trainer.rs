use super::TestRunner;
use crate::utils::{
    click_cookies_button, click_on_pixel, is_kill_switch_pressed, spawn_completion_monitor,
};
use anyhow::Result;
use headless_chrome::Tab;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[cfg(target_os = "windows")]
use winapi::{
    shared::windef::HDC,
    um::{
        wingdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetPixel,
            SelectObject, SRCCOPY,
        },
        winuser::{GetDC, GetDesktopWindow, ReleaseDC},
    },
};

pub async fn run() -> Result<()> {
    println!("🎯 Starting Aim Trainer Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/aim")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Find the area where the targets appear and determine the pixel coordinates of the corners
    let target_area = tab.find_element(".css-42wpoy.e19owgy79")?;

    // Get the bounding rectangle of the target area
    let quad = target_area.get_box_model()?.content;

    // Get browser window position on screen
    let window_bounds = tab.get_bounds()?;
    let window_x = window_bounds.left as i32;
    let window_y = window_bounds.top as i32;
    let x_offset_browser = 5; // Offset to take the browser url bar and toolbar into account
    let y_offset_browser = 140; // Offset to take the browser url bar and toolbar into account

    // Convert browser coordinates to screen coordinates
    let (left, top) = (
        quad.top_left.x as i32 + window_x + x_offset_browser,
        quad.top_left.y as i32 + 50 + window_y + y_offset_browser,
    );
    let (right, bottom) = (
        quad.bottom_right.x as i32 + window_x + x_offset_browser,
        quad.bottom_right.y as i32 - 50 + window_y + y_offset_browser,
    );

    println!(
        "Target area bounds: ({}, {}) to ({}, {})",
        left, top, right, bottom
    );

    // Start by clicking in the middle to begin the test
    let center_x = (left + right) / 2;
    let center_y = (top + bottom) / 2;
    click_on_pixel(center_x, center_y)?;

    #[cfg(target_os = "windows")]
    {
        // Use ultra-fast Windows screenshot-based target detection
        let _ = ultra_fast_target_detection(left, top, right, bottom, &tab);
    }

    println!("✅ Aim Trainer Test completed");

    // Wait to see results
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}

#[cfg(target_os = "windows")]
fn ultra_fast_target_detection(
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    tab: &Arc<Tab>,
) -> Result<(), String> {
    println!("🚀 Using ultra-fast Windows screenshot-based target detection");

    // Shared atomic boolean to signal when test is complete
    let test_complete = Arc::new(AtomicBool::new(false));

    let completion_checker = spawn_completion_monitor(tab.clone(), test_complete.clone());

    unsafe {
        let hwnd = GetDesktopWindow();
        let hdc_screen = GetDC(hwnd);

        if hdc_screen.is_null() {
            return Err("Failed to get device context".to_string());
        }

        let mut targets_hit: i32 = 0;
        let width = right - left;
        let height = bottom - top;

        // Sample the background color from multiple points at the start
        let background_color = 0xD1872B;

        'main_loop: loop {
            // Check for kill switch (ESC key)
            if is_kill_switch_pressed() {
                println!("🛑 Kill switch activated (ESC pressed) - stopping aim trainer");
                break 'main_loop;
            }

            // Check if test is complete (non-blocking atomic read)
            if test_complete.load(Ordering::Relaxed) {
                break 'main_loop;
            }

            // Create compatible DC and bitmap for screenshot
            let hdc_mem = CreateCompatibleDC(hdc_screen);
            let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);
            let old_bitmap = SelectObject(hdc_mem, hbitmap as *mut _);

            // Take screenshot of the target area
            BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, left, top, SRCCOPY);

            // Analyze the screenshot to find targets
            if let Some((target_x, target_y)) =
                find_target_in_screenshot(hdc_mem, width, height, background_color)
            {
                // Convert relative coordinates back to screen coordinates
                let screen_x = left + target_x;
                let screen_y = top + target_y;

                // Position cursor and click
                let _ = click_on_pixel(screen_x, screen_y);

                targets_hit += 1;
                println!(
                    "✨ Target #{} hit at ({}, {})",
                    targets_hit, screen_x, screen_y
                );
            }

            // Cleanup
            SelectObject(hdc_mem, old_bitmap);
            DeleteObject(hbitmap as *mut _);
            DeleteDC(hdc_mem);
        }

        ReleaseDC(hwnd, hdc_screen);
    }

    // Signal the completion checker to stop and wait for it
    test_complete.store(true, Ordering::Relaxed);
    let _ = completion_checker.join();

    Ok(())
}

#[cfg(target_os = "windows")]
unsafe fn find_target_in_screenshot(
    hdc: HDC,
    width: i32,
    height: i32,
    background_color: u32,
) -> Option<(i32, i32)> {
    // Scan the screenshot for pixels that differ significantly from background
    let step_size = 60; // Scan every 60 pixels for performance

    for y in (10..height - 10).step_by(step_size) {
        for x in (10..width - 10).step_by(step_size) {
            let pixel_color = GetPixel(hdc, x, y);

            if pixel_color != background_color {
                return Some((x, y)); // Return first different pixel found
            }
        }
    }

    None // No different pixel found
}
