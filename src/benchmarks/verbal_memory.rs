use crate::utils::{
    click_cookies_button, click_on_pixel, determ_center_of_element, is_kill_switch_pressed, Point,
};

use super::TestRunner;
use anyhow::Result;
use headless_chrome::Tab;
use std::{collections::HashSet, sync::Arc, time::Duration};

pub async fn run() -> Result<()> {
    println!("📝 Starting Verbal Memory Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/verbal-memory")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    verbal_memory_actions(&tab)?;

    println!("✅ Verbal Memory Test completed");

    // Wait to see the result
    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}

fn verbal_memory_actions(tab: &Arc<Tab>) -> Result<()> {
    // Find and click the start button
    tab.wait_for_element(".css-de05nr.e19owgy710")?;
    let start_button = tab.find_element(".css-de05nr.e19owgy710")?;
    start_button.click()?;

    // Get browser window position on screen
    let window_bounds = tab.get_bounds()?;
    let window_x = window_bounds.left as i32;
    let window_y = window_bounds.top as i32;
    let x_offset_browser = 5; // Offset to take the browser url bar and toolbar into account
    let y_offset_browser = 140; // Offset to take the browser url bar and toolbar into account

    let mut seen_words: HashSet<String> = HashSet::new();
    let mut new_button_position: Option<Point> = None;
    let mut seen_button_position: Option<Point> = None;

    // Initialize the new button
    let buttons = tab.find_elements(".css-de05nr.e19owgy710")?;
    for button in buttons {
        match button.get_inner_text()?.as_str() {
            "SEEN" => {
                seen_button_position = Some(determ_center_of_element(
                    &button,
                    &window_x,
                    &window_y,
                    &x_offset_browser,
                    &y_offset_browser,
                )?);
            }
            "NEW" => {
                new_button_position = Some(determ_center_of_element(
                    &button,
                    &window_x,
                    &window_y,
                    &x_offset_browser,
                    &y_offset_browser,
                )?);
            }
            _ => {}
        }
    }

    // Initialize the "word" element
    let word_element = tab.find_element(".word")?;
    let mut last_word = String::new();

    while !is_kill_switch_pressed() {
        let word = word_element.get_inner_text()?;

        // Only process if this is a new word (different from the last one we processed)
        if last_word != word {
            // Check if the word has been seen before. If not, add it to the list and continue
            if !seen_words.contains(&word) {
                seen_words.insert(word.clone()); // Only clone when inserting into HashSet

                // Click the "NEW" button
                let position = new_button_position.as_ref().unwrap();
                click_on_pixel(position.x, position.y)?;
            } else {
                // Click the "SEEN" button
                let position = seen_button_position.as_ref().unwrap();
                click_on_pixel(position.x, position.y)?;
            }

            // Move the word instead of cloning
            last_word = word;
        }
    }

    Ok(())
}
