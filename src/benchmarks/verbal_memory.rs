use crate::utils::click_cookies_button;

use super::TestRunner;
use anyhow::Result;
use std::time::Duration;

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
