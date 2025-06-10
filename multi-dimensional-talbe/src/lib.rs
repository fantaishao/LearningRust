use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use js_sys::Float64Array;

// Define highlighting modes as a type
#[wasm_bindgen]
pub enum HighlightMode {
    Cell,
    Row,
    Column
}

/***
 * draw rectangle in canvas of browser
 * @param {HTMLElement} canvas - canvas element
 * @param {number} x - x position of rectangle
 * @param {number} y - y position of rectangle
 * @param {number} width - width of rectangle
 * @param {number} height - height of rectangle
 */
#[wasm_bindgen]
pub fn draw_rectangle(canvas: &web_sys::HtmlCanvasElement, x: i32, y: i32, width: i32, height: i32) {
    let context = canvas.get_context("2d").unwrap().unwrap();
    let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();
    ctx.begin_path();
    ctx.rect(x as f64, y as f64, width as f64, height as f64);
    ctx.stroke();
}

/***
 * draw a table with specified rows and columns
 * @param {HTMLElement} canvas - canvas element
 * @param {number} x - x position of table
 * @param {number} y - y position of table
 * @param {number} width - total width of table
 * @param {number} height - total height of table
 * @param {number} cell_widths - widths of grid
 * @param {number} cell_heights - heights of grid
 * @param {number} rows - number of rows
 * @param {number} cols - number of columns
 */
#[wasm_bindgen]
pub fn draw_table(
    canvas: &web_sys::HtmlCanvasElement,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    cell_widths: &js_sys::Float64Array,
    cell_heights: &js_sys::Float64Array,
    rows: i32,
    cols: i32
) {
    let context = canvas.get_context("2d").unwrap().unwrap();
    let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    // draw the outer rectangle
    ctx.begin_path();
    ctx.rect(x as f64, y as f64, width as f64, height as f64);
    ctx.stroke();

    ctx.set_font("14px Arial");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    // Precompute cumulative positions
    let mut y_pos = y as f64;
    for row in 0..rows {
        let cell_height = cell_heights.get_index(row as u32);
        let mut x_pos = x as f64;
        for col in 0..cols {
            let cell_width = cell_widths.get_index(col as u32);

            // Draw vertical lines
            if col > 0 {
                ctx.begin_path();
                ctx.move_to(x_pos, y_pos);
                ctx.line_to(x_pos, y_pos + cell_height);
                ctx.stroke();
            }

            // Add text to each cell (row,col coordinates)
            let text = format!("{},{}", row, col);
            let text_x = x_pos + (cell_width / 2.0);
            let text_y = y_pos + (cell_height / 2.0);
            ctx.stroke_text(&text, text_x, text_y).unwrap();

            x_pos += cell_width;
        }
        // Draw horizontal lines
        if row > 0 {
            ctx.begin_path();
            ctx.move_to(x as f64, y_pos);
            ctx.line_to(x as f64 + width as f64, y_pos);
            ctx.stroke();
        }
        y_pos += cell_height;
    }
}

#[wasm_bindgen]
pub fn get_cell_at_position(
    x_pos: f64,
    y_pos: f64,
    table_x: f64,
    table_y: f64,
    cell_widths: &js_sys::Float64Array,
    cell_heights: &js_sys::Float64Array,
    rows: i32,
    cols: i32
) -> JsValue {
    if x_pos < table_x || y_pos < table_y {
        return JsValue::NULL;
    }

    let mut rel_x = x_pos - table_x;
    let mut rel_y = y_pos - table_y;

    let mut col = -1;
    let mut acc_width = 0.0;
    for c in 0..cols {
        let w = cell_widths.get_index(c as u32);
        if rel_x < acc_width + w {
            col = c;
            break;
        }
        acc_width += w;
    }

    let mut row = -1;
    let mut acc_height = 0.0;
    for r in 0..rows {
        let h = cell_heights.get_index(r as u32);
        if rel_y < acc_height + h {
            row = r;
            break;
        }
        acc_height += h;
    }

    if row < 0 || row >= rows || col < 0 || col >= cols {
        return JsValue::NULL;
    }

    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"row".into(), &(row as f64).into()).unwrap();
    js_sys::Reflect::set(&result, &"col".into(), &(col as f64).into()).unwrap();

    result.into()
}

// Enhanced highlight function with mode parameter
#[wasm_bindgen]
pub fn highlight_with_mode(
    canvas: &web_sys::HtmlCanvasElement,
    row: i32,
    col: i32,
    table_x: f64,
    table_y: f64,
    cell_widths: &js_sys::Float64Array,
    cell_heights: &js_sys::Float64Array,
    rows: i32,
    cols: i32,
    highlight_color: &str,
    mode: HighlightMode
) {
    if row < 0 || col < 0 {
        return;
    }

    let context = canvas.get_context("2d").unwrap().unwrap();
    let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    ctx.save();
    ctx.set_fill_style(&JsValue::from_str(highlight_color).into());

    // Calculate positions
    let mut x = table_x;
    for c in 0..col {
        x += cell_widths.get_index(c as u32);
    }
    let mut y = table_y;
    for r in 0..row {
        y += cell_heights.get_index(r as u32);
    }
    let cell_w = cell_widths.get_index(col as u32);
    let cell_h = cell_heights.get_index(row as u32);

    match mode {
        HighlightMode::Cell => {
            ctx.fill_rect(x, y, cell_w, cell_h);
        },
        HighlightMode::Row => {
            let mut row_x = table_x;
            let mut row_width = 0.0;
            for c in 0..cols {
                row_width += cell_widths.get_index(c as u32);
            }
            ctx.fill_rect(row_x, y, row_width, cell_h);
        },
        HighlightMode::Column => {
            let mut col_y = table_y;
            let mut col_height = 0.0;
            for r in 0..rows {
                col_height += cell_heights.get_index(r as u32);
            }
            ctx.fill_rect(x, col_y, cell_w, col_height);
        },
    }

    ctx.restore();
}

// Function to hightlight a cell
#[wasm_bindgen]
pub fn highlight_cell(
    canvas: &web_sys::HtmlCanvasElement,
    row: i32,
    col: i32,
    table_x: f64,
    table_y: f64,
    cell_width: f64,
    cell_height: f64,
    highlight_color: &str
) {
    // Check if highlight is needed at all
    if row < 0 || col < 0 {
        return;
    }

    let context = canvas.get_context("2d").unwrap().unwrap();
    let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    let x = table_x + (col as f64 * cell_width);
    let y = table_y + (row as f64 * cell_height);

    ctx.save();

    // Set highlight style
    ctx.set_fill_style(&JsValue::from_str(highlight_color).into());

    // Draw highlight rectangle
    ctx.fill_rect(
        x,
        y,
        cell_width,
        cell_height
    );

    ctx.restore();
}

#[wasm_bindgen]
pub fn clear_canvas(canvas: &web_sys::HtmlCanvasElement) {
    let context = canvas.get_context("2d").unwrap().unwrap();
    let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();
    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}