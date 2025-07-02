use crate::utils::click_cookies_button;

use super::TestRunner;
use anyhow::Result;
use std::time::Duration;

pub async fn run(max_level: u32) -> Result<()> {
    println!("🧠 Starting Sequence Memory Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/sequence")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("🎮 Please play the sequence memory game manually");
    println!("   Target level: {}", max_level);
    println!("   Watch the sequence and repeat it by clicking the squares");

    // Wait for user to complete the game
    std::thread::sleep(Duration::from_secs(60));

    println!("✅ Sequence Memory Test completed");

    Ok(())
}
