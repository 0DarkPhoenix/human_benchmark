use super::TestRunner;
use anyhow::Result;
use std::time::Duration;

pub async fn run() -> Result<()> {
    println!("👁️ Starting Visual Memory Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/visual-memory")?;
    tab.wait_for_element("div[data-testid='start-button']")?;

    println!("🧠 Please complete the visual memory test manually");
    println!("   Memorize the pattern and click the squares that lit up");

    // Wait for user to complete
    std::thread::sleep(Duration::from_secs(180));

    println!("✅ Visual Memory Test completed");

    Ok(())
}
