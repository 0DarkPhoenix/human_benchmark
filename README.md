# Human Benchmark
### 🚧 Under development 🚧
A CLI tool written in Rust and optimized for Windows (x86) with scripts to beat every [Human Benchmark](https://humanbenchmark.com/) test.

Human Benchmark is a website where you can do multiple tests to see how you compare to other humans in areas like reaction time, memory, typing speed, and more. Doing these test yourself is fun, but it is even more fun to see how far you can push those tests to their absolute limit using your computer to do the hard work ;)

## Installation
Whichever installation method you choose, make sure you have [Chrome](https://www.google.com/chrome/) installed.

### Simple
1. Download the latest Windows x86 executable from the releases page (**Coming Soon**)
2. Run the executable

### Advanced
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository
3. Run in debug mode with `cargo run main.rs` or run in release mode with `cargo run --release main.rs` (Performance difference is negligible)

## Usage
When you run the executable, you are greeted with an overview of the available tests. By typing a number, you can select the test you want to run. After pressing enter, an automated Chrome instance is launched for the test, and the script will complete the test for you.
![CLI Options Overview](assets/images/cli_options_overview.png)

## Showcases
### Reaction Time
⏯ Video coming soon...

### Typing
⏯ Video coming soon...

### Sequence Memory
🚧 Under development 🚧

### Aim Trainer
⏯ Video coming soon...

### Number Memory
🚧 Under development 🚧

### Chimp Test
⏯ Video coming soon...

### Verbal Memory
🚧 Under development 🚧

### Visual Memory
🚧 Under development 🚧

## Crates
- [headless_chrome](https://crates.io/crates/headless_chrome) for launching a automated Chrome instance, retrieve HTML content from the page and perform click actions in non-performance-critical sections
- [win32api](https://crates.io/crates/win32api) for performance-critical actions like positioning the mouse cursor, performing mouse clicks, and performing keyboard actions
- [scraper](https://crates.io/crates/scraper) for parsing HTML content

