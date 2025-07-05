use anyhow::Result;
use headless_chrome::Tab;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use winapi::um::winuser::{
    mouse_event, GetAsyncKeyState, SetCursorPos, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    VK_ESCAPE,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Finds and clicks the cookies acceptance button on the Human Benchmark website.
///
/// This function waits for the cookies button element to appear, then attempts to click it.
/// A 500ms delay is added after clicking to allow the page to process the action.
///
/// ## Arguments
/// * `tab` - A reference to the browser tab where the action should be performed
///
/// ## Returns
/// Returns `Ok(())` if the cookies button was found and clicked successfully, or if no
/// cookies button was present. Returns an error if waiting for the element fails.
///
/// ## Errors
/// This function will return an error if:
/// - The tab fails to wait for the `.fc-button-label` element
/// - The element click operation fails
pub fn click_cookies_button(tab: &Arc<Tab>) -> Result<()> {
    // Wait for the element to appear
    std::thread::sleep(Duration::from_secs(3));

    // Try to find and click the first type of cookie button
    if let Ok(accept_cookies_button) = tab.find_element(".fc-button-label") {
        accept_cookies_button.click()?;
        println!("Clicked accept cookies button (.fc-button-label)");
        return Ok(());
    }

    // If first type not found, try the second type
    if let Ok(accept_cookies_button) = tab.find_element(".css-47sehv") {
        accept_cookies_button.click()?;
        println!("Clicked accept cookies button (.css-47sehv)");
        return Ok(());
    }

    println!("No cookie button found to click");
    Ok(())
}

/// Sets the cursor to the pixel coordinates and performs a mouse click
///
/// This function handles the mouse cursor position placement and click at once.
/// The mouse cursor position and mouse click logic used are from the Win32 Api
///
/// ## Arguments
/// * `x` - x-coordinate of the target pixel
/// * `y` - y-coordinate of the target pixel
///
/// ## Returns
/// Returns `Ok(())` if the click operation was successful
pub fn click_on_pixel(x: i32, y: i32) -> Result<()> {
    unsafe {
        // Set the cursor position using the Win32 Api
        SetCursorPos(x, y);

        // Click using the Win32 Api
        mouse_event(MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
    Ok(())
}

/// Checks if the ESC key is currently pressed (kill switch)
///
/// This function uses the Win32 API to check if the ESC key is currently being pressed.
/// It can be used as a kill switch to gracefully exit from long-running operations.
///
/// ## Returns
/// Returns `true` if the ESC key is currently pressed, `false` otherwise
pub fn is_kill_switch_pressed() -> bool {
    unsafe {
        // Check if ESC key is pressed (0x8000 bit indicates key is currently down)
        (GetAsyncKeyState(VK_ESCAPE) as u16 & 0x8000) != 0
    }
}

/// Spawns a background thread that monitors for a DOM element's presence
/// and signals completion when found.
pub fn spawn_completion_monitor(
    tab: Arc<Tab>,
    completion_signal: Arc<AtomicBool>,
) -> JoinHandle<()> {
    // Spawn a new thread to monitor for the completion signal
    thread::spawn(move || {
        while !completion_signal.load(Ordering::Relaxed) {
            // Check if the 'Save score' button is present
            if tab.find_element(".css-qm6rs9.e19owgy710").is_ok() {
                println!("🏁 Test completed - found completion element",);
                completion_signal.store(true, Ordering::Relaxed);
                break;
            }
            // Check every 100ms to avoid overwhelming the browser
            thread::sleep(Duration::from_millis(100));
        }
    })
}
