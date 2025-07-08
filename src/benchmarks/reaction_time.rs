use std::{sync::Arc, time::Duration};

use crate::{
    benchmarks::TestRunner,
    utils::{click_cookies_button, click_on_pixel, is_kill_switch_pressed},
};

use anyhow::Result;

use headless_chrome::Tab;

pub async fn run() -> Result<()> {
    println!("🚦 Starting Reaction Time Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/reactiontime")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Get the browser window position and reaction area coordinates
    let (click_x, click_y) = get_reaction_area_coordinates(&tab)?;
    println!("Reaction area coordinates: ({}, {})", click_x, click_y);

    for round in 1..6 {
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

        reaction_time_actions(&tab, click_x, click_y)?;
    }

    // Wait to see the result
    std::thread::sleep(Duration::from_secs(5));

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

fn reaction_time_actions(tab: &Arc<Tab>, click_x: i32, click_y: i32) -> Result<()> {
    // Find the element to scan
    let reaction_element = tab.find_element(".view-waiting.e18o0sx0.css-saet2v.e19owgy77")?;

    while !is_kill_switch_pressed() {
        let reaction_element_content = reaction_element.get_content()?;

        // Check if "Click!" is present in the HTML content
        if reaction_element_content.contains("Click!") {
            click_on_pixel(click_x, click_y)?;
            break;
        }
    }

    Ok(())
}
