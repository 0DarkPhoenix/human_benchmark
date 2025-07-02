pub mod aim_trainer;
pub mod chimp_test;
pub mod number_memory;
pub mod reaction_time;
pub mod sequence_memory;
pub mod typing;
pub mod verbal_memory;
pub mod visual_memory;

use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::ffi::OsStr;
use std::sync::Arc;

pub struct TestRunner {
    browser: Browser,
}

impl TestRunner {
    pub fn new() -> Result<Self> {
        let browser = Browser::new(
            LaunchOptions::default_builder()
                .args(vec![
                    OsStr::new("--no-sandbox"),
                    OsStr::new("--disable-dev-shm-usage"),
                    OsStr::new("--disable-gpu"),
                    OsStr::new("--disable-background-timer-throttling"),
                    OsStr::new("--disable-backgrounding-occluded-windows"),
                    OsStr::new("--disable-renderer-backgrounding"),
                ])
                .headless(false) // Set to true for headless mode
                .build()
                .expect("Could not find chrome-executable"),
        )?;

        Ok(Self { browser })
    }

    pub fn get_tab(&self) -> Result<Arc<Tab>> {
        self.browser.new_tab()
    }
}
