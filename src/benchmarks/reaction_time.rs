use crate::{benchmarks::TestRunner, utils::click_cookies_button};

use anyhow::Result;

#[cfg(target_os = "windows")]
use winapi::um::{
    wingdi::GetPixel,
    winuser::{
        mouse_event, GetDC, GetDesktopWindow, ReleaseDC, SetCursorPos, MOUSEEVENTF_LEFTDOWN,
        MOUSEEVENTF_LEFTUP,
    },
};

pub async fn run(rounds: u32) -> Result<()> {
    println!("🚦 Starting Reaction Time Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/reactiontime")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    let mut results = Vec::new();

    // Get the browser window position and reaction area coordinates
    let (click_x, click_y) = get_reaction_area_coordinates(&tab)?;
    println!("Reaction area coordinates: ({}, {})", click_x, click_y);

    for round in 1..=rounds.min(4) {
        println!("Round {}/{}", round, rounds);

        // Click start button
        if round == 1 {
            // Wait for the ads to load in
            std::thread::sleep(std::time::Duration::from_secs(5));

            tab.wait_for_element(".view-splash.e18o0sx0.css-saet2v.e19owgy77")?;
            let start_button = tab.find_element(".view-splash.e18o0sx0.css-saet2v.e19owgy77")?;
            start_button.click()?;
        } else {
            tab.wait_for_element(".view-result.e18o0sx0.css-saet2v.e19owgy77")?;
            let continue_button = tab.find_element(".view-result.e18o0sx0.css-saet2v.e19owgy77")?;
            continue_button.click()?;
        }

        // Wait for the red waiting screen to appear
        tab.wait_for_element(".view-waiting.e18o0sx0.css-saet2v.e19owgy77")?;

        #[cfg(target_os = "windows")]
        {
            // Use ultra-fast Windows optimization
            match ultra_fast_windows_detection(click_x, click_y) {
                Ok(_) => {
                    // Read the reaction time from the page
                    tab.wait_for_element(".view-result.e18o0sx0.css-saet2v.e19owgy77")?;

                    // Extract reaction time from the deepest div inside h1
                    if let Ok(h1_element) = tab.find_element("h1") {
                        if let Ok(time_div) = h1_element.find_element("div") {
                            if let Ok(time_text) = time_div.get_inner_text() {
                                if let Ok(reaction_ms) =
                                    time_text.trim_end_matches(" ms").parse::<u32>()
                                {
                                    results.push(reaction_ms);
                                    println!("  Ultra-fast result: {} ms", reaction_ms);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Windows optimization failed: {}", e);
                }
            }
        }
    }

    if !results.is_empty() {
        let average = results.iter().sum::<u32>() as f32 / results.len() as f32;
        let best = *results.iter().min().unwrap();
        let worst = *results.iter().max().unwrap();
        println!("\n📊 Ultra-Optimized Reaction Time Results:");
        println!("  Average: {:.1} ms", average);
        println!("  Best: {} ms", best);
        println!("  Worst: {} ms", worst);
        println!("  All results: {:?}", results);
    }

    Ok(())
}

fn get_reaction_area_coordinates(tab: &headless_chrome::Tab) -> Result<(i32, i32)> {
    // Get browser window position and size
    let bounds = tab.get_bounds()?;

    // Calculate the center of the reaction area
    // The reaction area is typically centered in the viewport
    let center_x = (bounds.left + (bounds.width / 2.0) as u32) as i32;
    let center_y = (bounds.top + (bounds.height / 2.0) as u32) as i32;

    Ok((center_x, center_y))
}

#[cfg(target_os = "windows")]
fn ultra_fast_windows_detection(click_x: i32, click_y: i32) -> Result<u32, String> {
    println!(
        "Using MAXIMUM SPEED Windows detection at ({}, {}) synced to 165Hz",
        click_x, click_y
    );

    unsafe {
        let hwnd = GetDesktopWindow();
        let hdc = GetDC(hwnd);

        if hdc.is_null() {
            return Err("Failed to get device context".to_string());
        }

        // Target green color
        const TARGET_GREEN: u32 = 0x6ADB4B;

        // Pre-position cursor to minimize click latency
        SetCursorPos(click_x, click_y);

        // Remove frame synchronization entirely - just poll as fast as possible
        let start_time = std::time::Instant::now();
        let mut iteration_count = 0u32;

        loop {
            // Direct pixel read - no frame waiting
            let pixel_color = GetPixel(hdc, click_x, click_y);

            // Immediate color check and click
            if pixel_color == TARGET_GREEN {
                // INSTANT click - no delay whatsoever
                mouse_event(MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
                ReleaseDC(hwnd, hdc);
                return Ok(0);
            }

            // Check timeout less frequently to reduce overhead
            iteration_count += 1;
            if iteration_count % 10000 == 0 && start_time.elapsed().as_secs() > 10 {
                break;
            }
        }

        // Cleanup on timeout
        ReleaseDC(hwnd, hdc);
    }

    Err("Green color not detected within timeout".to_string())
}
