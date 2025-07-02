use super::TestRunner;
use crate::utils::click_cookies_button;

use anyhow::Result;

#[cfg(target_os = "windows")]
use winapi::um::winuser::{keybd_event, VkKeyScanA, KEYEVENTF_KEYUP, VK_SHIFT, VK_SPACE};

pub async fn run() -> Result<()> {
    println!("⌨️  Starting Typing Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/typing")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Click the text area to focus it
    tab.wait_for_element(".e1q0za6r0.css-1c2t4mr.e19owgy77")?;
    let text_area = tab.find_element(".e1q0za6r0.css-1c2t4mr.e19owgy77")?;
    text_area.click()?;
    println!("Clicked text area");

    // Read the text to type
    tab.wait_for_element(".letters.notranslate")?;
    let letters_container = tab.find_element(".letters.notranslate")?;
    println!("Found letters container");

    let text_to_type = letters_container.get_inner_text()?;
    println!("Text to type: {}", text_to_type);
    println!("Characters count: {}", text_to_type.len());

    // Wait a moment before starting to type
    std::thread::sleep(std::time::Duration::from_millis(1000));

    // Type the text using Win32 API
    #[cfg(target_os = "windows")]
    {
        match ultra_fast_typing(&text_to_type) {
            Ok(_) => println!("✅ Typing completed successfully!"),
            Err(e) => println!("❌ Typing failed: {}", e),
        }
    }

    // Wait to see results
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}

#[cfg(target_os = "windows")]
enum KeyAction {
    KeyPress(u8),
    KeyRelease(u8),
    ShiftPress,
    ShiftRelease,
    Space,
}

#[cfg(target_os = "windows")]
fn ultra_fast_typing(text: &str) -> Result<(), String> {
    println!("🚀 Starting ultra-fast Win32 typing...");

    // Pre-calculate all key actions
    let actions = build_key_actions(text)?;
    println!("📋 Pre-calculated {} key actions", actions.len());

    // Execute all actions at once
    execute_key_actions(&actions)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn build_key_actions(text: &str) -> Result<Vec<KeyAction>, String> {
    let mut actions = Vec::with_capacity(text.len() * 3); // Rough estimate for capacity

    for ch in text.chars() {
        match ch {
            ' ' => {
                actions.push(KeyAction::Space);
            }
            '\'' => {
                // Handle apostrophe specifically using VK_OEM_7 (0xDE)
                actions.push(KeyAction::KeyPress(0xDE));
                actions.push(KeyAction::KeyRelease(0xDE));
                actions.push(KeyAction::Space);
            }
            '"' => {
                // Handle apostrophe specifically using VK_OEM_7 (0xDE)
                actions.push(KeyAction::ShiftPress);
                actions.push(KeyAction::KeyPress(0xDE));
                actions.push(KeyAction::KeyRelease(0xDE));
                actions.push(KeyAction::ShiftRelease);
                actions.push(KeyAction::Space);
            }
            _ if ch.is_ascii() => unsafe {
                let vk_code = VkKeyScanA(ch as u8 as i8);

                // Check if VkKeyScanA failed (returns -1/0xFFFF)
                if vk_code == -1 {
                    println!("VkKeyScanA failed for character: '{}'", ch);
                    continue;
                }

                let vk = (vk_code & 0xFF) as u8;
                let shift_needed = (vk_code & 0x100) != 0;

                // Validate virtual key code
                if vk == 0 {
                    println!("Invalid virtual key code for character: '{}'", ch);
                    continue;
                }

                if shift_needed {
                    actions.push(KeyAction::ShiftPress);
                }

                actions.push(KeyAction::KeyPress(vk));
                actions.push(KeyAction::KeyRelease(vk));

                if shift_needed {
                    actions.push(KeyAction::ShiftRelease);
                }
            },
            _ => {
                println!("Skipping non-ASCII character: '{}'", ch);
                continue;
            }
        }
    }

    Ok(actions)
}

#[cfg(target_os = "windows")]
fn execute_key_actions(actions: &[KeyAction]) -> Result<(), String> {
    unsafe {
        for action in actions {
            match action {
                KeyAction::KeyPress(vk) => {
                    keybd_event(*vk, 0, 0, 0);
                }
                KeyAction::KeyRelease(vk) => {
                    keybd_event(*vk, 0, KEYEVENTF_KEYUP, 0);
                }
                KeyAction::ShiftPress => {
                    keybd_event(VK_SHIFT as u8, 0, 0, 0);
                }
                KeyAction::ShiftRelease => {
                    keybd_event(VK_SHIFT as u8, 0, KEYEVENTF_KEYUP, 0);
                }
                KeyAction::Space => {
                    keybd_event(VK_SPACE as u8, 0, 0, 0);
                    keybd_event(VK_SPACE as u8, 0, KEYEVENTF_KEYUP, 0);
                }
            }
        }
    }

    Ok(())
}
