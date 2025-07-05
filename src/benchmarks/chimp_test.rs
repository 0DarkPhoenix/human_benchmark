use super::TestRunner;
use crate::utils::{click_cookies_button, click_on_pixel, is_kill_switch_pressed, Point};
use anyhow::Result;
use headless_chrome::{Element, Tab};
use rayon::prelude::*;
use scraper::{Html, Selector};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct NumberPosition {
    number: u32,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct GridCell {
    row: usize,
    col: usize,
    screen_position: Point,
}

#[derive(Debug)]
struct ChimpGrid {
    cells: Vec<Vec<GridCell>>,
    grid_bounds: (usize, usize), // (rows, cols)
}

impl ChimpGrid {
    /// Initialize a new grid
    fn new() -> Self {
        Self {
            cells: Vec::new(),
            grid_bounds: (0, 0),
        }
    }

    /// Add a new cell to the grid
    fn add_cell(&mut self, cell: GridCell) {
        // Ensure the grid is large enough
        let target_row = cell.row;
        let target_col = cell.col;

        while self.cells.len() <= target_row {
            self.cells.push(Vec::new());
        }

        let row = &mut self.cells[target_row];
        while row.len() <= target_col {
            row.push(GridCell {
                row: target_row,
                col: row.len(),
                screen_position: Point { x: 0, y: 0 },
            });
        }

        // Update the cell
        row[target_col] = cell.clone();

        // Update grid bounds
        self.grid_bounds.0 = self.cells.len();
        if !self.cells.is_empty() {
            self.grid_bounds.1 = self.cells.iter().map(|row| row.len()).max().unwrap_or(0);
        }
    }

    /// Get the coordinates of a cell based on the row and column
    fn get_coordinates(&self, row: usize, col: usize) -> Option<Point> {
        if row < self.cells.len() && col < self.cells[row].len() {
            Some(self.cells[row][col].screen_position.clone())
        } else {
            None
        }
    }
}

pub async fn run() -> Result<()> {
    println!("🐵 Starting Chimp Test");

    let runner = TestRunner::new()?;
    let tab = runner.get_tab()?;

    tab.navigate_to("https://humanbenchmark.com/tests/chimp")?;

    // Handle cookies
    click_cookies_button(&tab)?;

    // Wait for the ads to load in
    std::thread::sleep(std::time::Duration::from_secs(5));

    chimp_test_actions(&tab)?;

    println!("✅ Chimp Test completed");

    // Wait to see the result
    std::thread::sleep(Duration::from_secs(5));

    Ok(())
}

/// Create a grid template from the HTML elements
///
/// This function will return a grid template where the screen coordinates of each cell are stored.
/// The grid template will be used by later logic to know where to click on the screen for each cell.
fn create_grid_template_from_html(
    tab: &Tab,
    window_x: &i32,
    window_y: &i32,
    x_offset: &i32,
    y_offset: &i32,
) -> Result<ChimpGrid> {
    let mut grid = ChimpGrid::new();

    // Find all row elements
    let rows = tab.find_elements(".css-k008qs")?;

    // Process rows in parallel and collect cells
    let cells: Result<Vec<Vec<GridCell>>> = rows
        .par_iter()
        .enumerate()
        .map(|(row_index, row_element)| -> Result<Vec<GridCell>> {
            // Find all cells in this row (both filled and empty)
            let all_cell_elements = row_element.find_elements(".css-ggichp, .css-19b5rdt")?;

            let row_cells: Result<Vec<GridCell>> = all_cell_elements
                .iter()
                .enumerate()
                .map(|(col_index, cell_element)| -> Result<GridCell> {
                    // Get the position of this cell
                    let screen_position = determ_center_of_element(
                        cell_element,
                        window_x,
                        window_y,
                        x_offset,
                        y_offset,
                    )?;

                    Ok(GridCell {
                        row: row_index,
                        col: col_index,
                        screen_position,
                    })
                })
                .collect();
            row_cells
        })
        .collect();

    // Add all cells to the grid sequentially
    for row_cells in cells? {
        for cell in row_cells {
            grid.add_cell(cell);
        }
    }
    Ok(grid)
}

/// Parse HTML content to find numbers and their grid positions
fn parse_numbers_from_html(html_content: &str) -> Result<Vec<NumberPosition>> {
    let document = Html::parse_document(html_content);
    let row_selector = Selector::parse(".css-k008qs").unwrap();
    let cell_selector = Selector::parse(".css-ggichp, .css-19b5rdt").unwrap();

    let mut number_positions = Vec::new();

    // Iterate through all rows
    for (row_index, row_element) in document.select(&row_selector).enumerate() {
        // Iterate through all cells in the row
        for (col_index, cell_element) in row_element.select(&cell_selector).enumerate() {
            // Check if this cell has a data-cellnumber attribute
            if let Some(cell_number_str) = cell_element.value().attr("data-cellnumber") {
                if let Ok(number) = cell_number_str.parse::<u32>() {
                    number_positions.push(NumberPosition {
                        number,
                        row: row_index,
                        col: col_index,
                    });
                }
            }
        }
    }

    Ok(number_positions)
}

fn chimp_test_actions(tab: &Arc<Tab>) -> Result<()> {
    // Get browser window position on screen
    let window_bounds = tab.get_bounds()?;
    let window_x = window_bounds.left as i32;
    let window_y = window_bounds.top as i32;
    let x_offset_browser = 5; // Offset to take the browser url bar and toolbar into account
    let y_offset_browser = 140; // Offset to take the browser url bar and toolbar into account

    let mut grid: Option<ChimpGrid> = None;

    let next_button = tab.find_element(".css-de05nr.e19owgy710")?;
    let next_button_location = determ_center_of_element(
        &next_button,
        &window_x,
        &window_y,
        &x_offset_browser,
        &y_offset_browser,
    )?;

    let mut pass: u8 = 1;
    let start_time = Instant::now();
    let mut init_grid_time: u128 = 0;

    while !is_kill_switch_pressed() {
        println!("Pass {}", pass);
        // Press the start/continue button
        println!("Searching the start/continue button");
        std::thread::sleep(Duration::from_millis(5));
        click_on_pixel(next_button_location.x, next_button_location.y)?;
        println!("Clicked the start/continue button");

        // Wait a moment for the grid to appear
        std::thread::sleep(Duration::from_millis(5));

        // Initialize the grid on the first run
        if grid.is_none() {
            println!("Initializing grid structure from HTML");

            grid = Some(create_grid_template_from_html(
                tab,
                &window_x,
                &window_y,
                &x_offset_browser,
                &y_offset_browser,
            )?);
            init_grid_time = start_time.elapsed().as_millis();
        }
        // Set the current grid
        let template_grid = grid.as_ref().unwrap();

        // Get the inner HTML of the container
        let html_content = tab.get_content()?;

        // Parse numbers and their positions from HTML
        let number_positions = parse_numbers_from_html(&html_content)?;

        // Sort numbers by their value to click them in order
        let mut sorted_positions = number_positions;
        sorted_positions.sort_by_key(|pos| pos.number);

        // Click numbers in sorted order
        for pos in sorted_positions {
            if let Some(point) = template_grid.get_coordinates(pos.row, pos.col) {
                std::thread::sleep(Duration::from_millis(3));
                click_on_pixel(point.x, point.y)?;
            } else {
                println!(
                    "Warning: Could not find coordinates for number {} at row {}, col {}",
                    pos.number, pos.row, pos.col
                );
            }
        }

        // Increment the pass counter
        pass += 1;

        // Check if we've reached the maximum number of passes
        if pass > 37 {
            let end_time = start_time.elapsed().as_millis();
            println!("Total time: {} milliseconds", end_time);
            println!("Initialize grid time: {} milliseconds", init_grid_time);
            println!(
                "Clicked through all stages in {} milliseconds",
                end_time - init_grid_time
            );
            break;
        }
    }

    Ok(())
}

/// Calculate the position of the element by determining the pixel coordinates of the element's center.
fn determ_center_of_element(
    element: &Element,
    window_x: &i32,
    window_y: &i32,
    x_offset: &i32,
    y_offset: &i32,
) -> Result<Point> {
    let quad = element.get_box_model()?.content;
    let center_x = (quad.top_left.x + quad.top_right.x) / 2.0;
    let center_y = (quad.top_left.y + quad.bottom_left.y) / 2.0;
    let screen_position = Point {
        x: center_x as i32 + window_x + x_offset,
        y: center_y as i32 + window_y + y_offset,
    };
    Ok(screen_position)
}
