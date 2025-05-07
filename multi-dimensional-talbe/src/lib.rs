use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

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
 * @param {number} cell_width - width of grid
 * @param {number} cell_height - height of grid
 * @param {number} rows - number of rows
 * @param {number} cols - number of columns
 */
#[wasm_bindgen]
pub fn draw_table(canvas: &web_sys::HtmlCanvasElement, x: i32, y: i32, width: i32, height: i32, cell_width: f64, cell_height: f64, rows: i32, cols: i32) {
    let context = canvas.get_context("2d").unwrap().unwrap();
    let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();

    // draw the outer rectangle
    ctx.begin_path();
    ctx.rect(x as f64, y as f64, width as f64, height as f64);
    ctx.stroke();

     // Set font for text
     ctx.set_font("14px Arial");
     ctx.set_text_align("center");
     ctx.set_text_baseline("middle");

    // Draw grid with text in each cell
    for row in 0..rows {
        let y_pos = y as f64 + (row as f64 * cell_height);
        
        // draw horizontal lines
        if row > 0 {
            ctx.begin_path();
            ctx.move_to(x as f64, y_pos);
            ctx.line_to(x as f64 + width as f64, y_pos);
            ctx.stroke();
        }
        
        for col in 0..cols {
            let x_pos = x as f64 + (col as f64 * cell_width);
            
            // draw vertical lines
            if col > 0 {
                ctx.begin_path();
                ctx.move_to(x_pos, y as f64);
                ctx.line_to(x_pos, y as f64 + height as f64);
                ctx.stroke();
            }
            
            // Add text to each cell (row,col coordinates)
            let text = format!("{},{}", row, col);
            let text_x = x_pos + (cell_width / 2.0);
            let text_y = y_pos + (cell_height / 2.0);
            ctx.stroke_text(&text, text_x, text_y).unwrap();
        }
    }

}

#[wasm_bindgen]
pub fn get_cell_at_position(
    x_pos: f64,
    y_pos: f64,
    table_x: f64,
    table_y: f64,
    cell_width: f64,
    cell_height: f64,
    rows: i32,
    cols: i32
) -> JsValue {
    // check if position is inside the table
    if x_pos < table_x || y_pos < table_y {
        return JsValue::NULL;
    }

    // Calculate relative position within table
    let rel_x = x_pos - table_x;
    let rel_y = y_pos - table_y;

    // Calculate cell indices
    let col = (rel_x / cell_width).floor() as i32;
    let row = (rel_y / cell_height).floor() as i32;
    
    // Check if within table bounds
    if row < 0 || row >= rows || col < 0 || col >= cols {
        return JsValue::NULL;
    }

    // Create return object
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
    cell_width: f64,
    cell_height: f64,
    rows: i32,
    cols: i32,
    highlight_color: &str,
    mode: HighlightMode
) {
 // Check if highlight is needed at all
 if row < 0 || col < 0 {
    return
 }

 let context = canvas.get_context("2d").unwrap().unwrap();
 let ctx = context.unchecked_into::<web_sys::CanvasRenderingContext2d>();

 ctx.save();

 // Set highlight style
 ctx.set_fill_style(&JsValue::from_str(highlight_color).into());

 // Draw highlight base on mode
 match mode {
    HighlightMode::Cell => {
        let x = table_x + (col as f64 * cell_width);
        let y = table_y + (row as f64 * cell_height);
        
        ctx.fill_rect(
            x,
            y,
            cell_width,
            cell_height
        );
    },
    HighlightMode::Row => {
        let y = table_y + (row as f64 * cell_height);
        let row_width = cols as f64 * cell_width;
        
        ctx.fill_rect(
            table_x,
            y,
            row_width,
            cell_height
        );
    },
    HighlightMode::Column => {
        let x = table_x + (col as f64 * cell_width);
        let col_height = rows as f64 * cell_height;
        
        ctx.fill_rect(
            x,
            table_y,
            cell_width,
            cell_height
        );
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