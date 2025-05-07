<template>
    <div class="canvas-wrapper">
        <canvas ref="canvasRef" @mousemove="handleMouseMove" @mouseout="handleMouseOut"></canvas>
    </div>
</template>
<script setup>
import { onMounted, onUnmounted, ref, render } from 'vue';
import init, { 
    draw_rectangle, 
    draw_table, 
    get_cell_at_position, 
    highlight_cell, 
    clear_canvas,
    highlight_with_mode,
    HighlightMode 
 } from '@/wasm/wasm_module';

const canvasRef = ref(null);
const overlayCanvasRef = ref(null); // New canvas for highlights only
const ctx = ref(null);
const cellWidth = 120;
const cellHeight = 40;
const rows = 200;
const columns = 35;
const hoveredCell = ref(null)
const previousHoveredCell = ref(null); // Track previous cell for cleanup
const tableX = ref(0)
const tableY = ref(0)
const lastRenderTime = ref(0)
const renderThrottleMs = 16 // ~60fps
const pendingRedraw = ref(false)
const rafId = ref(null)
const currentHighlightMode = ref(HighlightMode.Row) // Set to row highlighting by default

function createOverlayCanvas() {
    // Create an overlay canvas positioned on top of the main canvas
    overlayCanvasRef.value = document.createElement('canvas');
    overlayCanvasRef.value.style.position = 'absolute';
    overlayCanvasRef.value.style.top = '0';
    overlayCanvasRef.value.style.left = '0';
    overlayCanvasRef.value.style.pointerEvents = 'none'; // Pass events to canvas below
    
    // Add to the same container as the main canvas
    canvasRef.value.parentElement.appendChild(overlayCanvasRef.value);
}

function initializeTable() {
    if (!canvasRef.value) return;
    // draw_rectangle(canvasRef.value, 50, 50, 100, 100);


    // Get the device pixel ratio
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);

    // Get the CSS dimensions of the canvas container
    const displayWidth = cellWidth * columns + (2 * tableX.value);
    const displayHeight = cellHeight * rows + (2 * tableY.value);

    
    // Set the canvas CSS dimensions
    canvasRef.value.style.width = `${displayWidth}px`;
    canvasRef.value.style.height = `${displayHeight}px`;
    
    // Set the canvas buffer dimensions accounting for DPR
    canvasRef.value.width = displayWidth * dpr;
    canvasRef.value.height = displayHeight * dpr;

    // Scale the context to maintain the same visual size
    const context = canvasRef.value.getContext('2d');
    context.scale(dpr, dpr);
    
    // Configure overlay canvas the same way
    overlayCanvasRef.value.style.width = `${displayWidth}px`;
    overlayCanvasRef.value.style.height = `${displayHeight}px`;
    overlayCanvasRef.value.width = displayWidth * dpr;
    overlayCanvasRef.value.height = displayHeight * dpr;
    
    // Scale overlay context
    const overlayContext = overlayCanvasRef.value.getContext('2d');
    overlayContext.scale(dpr, dpr);
    
    // Draw the main table (only once)
    drawMainTable();
    
    // Set up positioning
    updateOverlayPosition();
}

function drawMainTable() {
    // Draw the table once - we won't redraw it for hover effects
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
    
    draw_table(
        canvasRef.value,
        tableX.value,
        tableY.value,
        canvasRef.value.width / dpr,
        canvasRef.value.height / dpr,
        cellWidth,
        cellHeight,
        rows, columns
    );
}
function updateHighlight() {
    // Use requestAnimationFrame for smoother rendering
    if (pendingRedraw.value) return

    pendingRedraw.value = true
    
    rafId.value = requestAnimationFrame(() => {
        const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
        const overlayContext = overlayCanvasRef.value.getContext('2d');
        // Clear only the overlay canvas
        overlayContext.clearRect(
            0, 
            0, 
            overlayCanvasRef.value.width / dpr, 
            overlayCanvasRef.value.height / dpr
        );

        
        // Redraw highlight if needed
        if (hoveredCell.value && hoveredCell.value.row >= 0 && hoveredCell.value.col >= 0) {
            highlight_with_mode(
                overlayCanvasRef.value,
                hoveredCell.value.row,
                hoveredCell.value.col,
                tableX.value,
                tableY.value,
                cellWidth,
                cellHeight,
                rows,
                columns,
                "rgba(0, 123, 255, 0.2)", // Highlight color
                currentHighlightMode.value
            )
        }

        pendingRedraw.value = false
    })
}

function updateOverlayPosition() {
    // Position the overlay exactly over the main canvas
    if (overlayCanvasRef.value && canvasRef.value) {
        const rect = canvasRef.value.getBoundingClientRect();
        overlayCanvasRef.value.style.top = `${rect.top}px`;
        overlayCanvasRef.value.style.left = `${rect.left}px`;
    }
}

function handleMouseMove(event) {
    // Throttle the processing of mouse events
    const now = performance.now()
    if (now - lastRenderTime.value < renderThrottleMs) {
        return // Skip processing this event if we're throttling
    }

    // Get mouse posiiton relative to canvas
    const rect = canvasRef.value.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    
    // IMPORTANT: Don't multiply by DPR here since we scaled the context
    // We want the logical coordinates, not the physical ones
    const mouseX = x;
    const mouseY = y;

    // Get cell at current posiiton
    const cell = get_cell_at_position(
        mouseX,
        mouseY,
        tableX.value,
        tableY.value,
        cellWidth,
        cellHeight,
        rows, columns
    )
    let shouldRedraw = false;

    if (cell != null) {
        // Check if hovering over a new cell
        if (!hoveredCell.value || hoveredCell.value.row !== cell.row || hoveredCell.value.col !== cell.col) {
            // update hovered cell
            hoveredCell.value = {
                row: cell.row,
                col: cell.col
            }

            shouldRedraw = true;
            lastRenderTime.value = now
        }
    } else if (hoveredCell.value !== null) {
        // Mouse not over any cell, clear highlight
        hoveredCell.value = null;
        shouldRedraw = true;
        lastRenderTime.value = now
    }

        // Only redraw if necessary
    if (shouldRedraw) {
        updateHighlight();
    }
}

function handleMouseOut() {// Only redraw if we had a highlighted cell
    if (hoveredCell.value !== null) {
        // Clear highlighting when mouse leaves canvas
        hoveredCell.value = null;
        updateHighlight();
    }
}

// Method to change highlight mode
function setHighlightMode(mode) {
    // mode can be HighlightMode.Cell, HighlightMode.Row, or HighlightMode.Column
    currentHighlightMode.value = mode;
    
    // Redraw highlight with new mode if we have a hovered cell
    if (hoveredCell.value) {
        updateHighlight();
    }
}

// Prepare for future dynamic sizing
function updateCellDimensions(newWidth, newHeight) {
    // Update cell dimensions
    cellWidth = newWidth;
    cellHeight = newHeight;
    
    // Redraw the entire table with new dimensions
    initializeTable();
}

// Handle scrolling to keep the overlay in sync
function handleScroll() {
    updateOverlayPosition();
}

onMounted(async () => {
    // Initialize the WASM module first
    await init();
    
    
    // Create overlay canvas for highlighting
    createOverlayCanvas();

    initializeTable()


    window.addEventListener('scroll', handleScroll);
    const wrapper = canvasRef.value?.parentElement;
    if (wrapper) {
        wrapper.addEventListener('scroll', handleScroll);
    }
})

onUnmounted(() => {
    if (rafId.value) {
        cancelAnimationFrame(rafId.value)
    }
    
    window.removeEventListener('scroll', handleScroll);
    const wrapper = canvasRef.value?.parentElement;
    if (wrapper) {
        wrapper.removeEventListener('scroll', handleScroll);
    }
    
    // Remove overlay canvas
    if (overlayCanvasRef.value && overlayCanvasRef.value.parentElement) {
        overlayCanvasRef.value.parentElement.removeChild(overlayCanvasRef.value);
    }
})
</script>

<style lang="scss" scoped>
.canvas-wrapper {
    width: 100%;    /* viewport width */
    height: 100%;   /* viewport height */
    overflow: auto;  /* show scrollbars when content >  wrapper */
    border: 1px solid #ccc;
  
    /* ensure no extra whitespace around the canvas */
    canvas {
        display: block;
    }
}
</style>