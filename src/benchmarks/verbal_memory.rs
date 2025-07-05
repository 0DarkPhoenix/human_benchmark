use crate::utils::{click_cookies_button, is_kill_switch_pressed};

use super::TestRunner;
use anyhow::Result;
use headless_chrome::Tab;
use scraper::{Html, Selector};
use std::{sync::Arc, time::Duration};

pub async fn run() -> Result<()> {
    println!("📝 Starting Verbal Memory Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/verbal-memory")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("🧠 Please complete the verbal memory test manually");
    println!("   Click 'SEEN' if you've seen the word before, 'NEW' if it's new");

    // Wait for user to complete
    std::thread::sleep(Duration::from_secs(180));

    println!("✅ Verbal Memory Test completed");

    Ok(())
}

fn verbal_memory_actions(tab: &Arc<Tab>) -> Result<()> {
    // Find and click the start button
    tab.wait_for_element(".css-de05nr.e19owgy710")?;
    let start_button = tab.find_element(".css-de05nr.e19owgy710")?;
    start_button.click()?;

    let mut seen_words_list: Vec<String> = Vec::new();

    while !is_kill_switch_pressed() {
        // Extract the HTML content from the page
        let html_content = tab.get_content()?;

        // Parse the word from the page
        let document = Html::parse_document(&html_content);
        let word_selector = Selector::parse(".word").unwrap();
        let word_element = document.select(&word_selector).next().unwrap();
        let word = word_element.text().collect::<String>();

        // Check if the word has been seen before. If not, add it to the list and continue
        if !seen_words_list.contains(&word) {
            // Click the "NEW" button
            let new_button = tab.find_element(".css-de05nr.e19owgy710")?;
            new_button.click()?;

            seen_words_list.push(word);
            continue;
        }
        // Click the "SEEN" button
        let seen_button = tab.find_element(".css-de05nr.e19owgy710")?;
        seen_button.click()?;
    }

    Ok(())
}
