use crate::utils::click_cookies_button;

use super::TestRunner;
use anyhow::Result;
use std::time::Duration;

pub async fn run(max_digits: u32) -> Result<()> {
    println!("🔢 Starting Number Memory Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/memory")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("🧠 Please complete the number memory test manually");
    println!("   Target digits: {}", max_digits);
    println!("   Memorize the numbers and type them back");

    // Wait for user to complete
    std::thread::sleep(Duration::from_secs(120));

    println!("✅ Number Memory Test completed");

    Ok(())
}
