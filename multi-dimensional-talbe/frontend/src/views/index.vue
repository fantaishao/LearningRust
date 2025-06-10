<template>
    <div class="canvas-wrapper" ref="wrapperRef">
        <canvas
            ref="canvasRef"
            @mousemove="handleMouseMove"
            @mousedown="handleMouseDown"
            @mouseleave="handleMouseOut"
        ></canvas>
        <!-- Column splitters -->
        <div
        v-for="(left, colIdx) in colSplitterPositions"
        :key="'col-splitter-' + colIdx"
        class="splitter splitter-vertical"
        :style="{ left: left + 'px', height: tableHeight + 'px', top: 0 }"
        @mousedown="startColResize(colIdx, $event)"
        ></div>
        <!-- Row splitters -->
        <div
        v-for="(top, rowIdx) in rowSplitterPositions"
        :key="'row-splitter-' + rowIdx"
        class="splitter splitter-horizontal"
        :style="{ top: top + 'px', width: tableWidth + 'px', left: 0 }"
        @mousedown="startRowResize(rowIdx, $event)"
        ></div>
    </div>
</template>
<script setup>
import { onMounted, onUnmounted, ref, computed } from 'vue';
import init, {
    draw_table,
    get_cell_at_position,
    highlight_with_mode,
    HighlightMode
} from '@/wasm/wasm_module';

const canvasRef = ref(null);
const overlayCanvasRef = ref(null);
const wrapperRef = ref(null);

const rows = 200;
const columns = 35;
const tableX = ref(0);
const tableY = ref(0);

const cellWidths = ref(Array(columns).fill(120));
const cellHeights = ref(Array(rows).fill(40));

const hoveredCell = ref(null);
const currentHighlightMode = ref(HighlightMode.Row);

const resizing = ref(false);
const resizeType = ref(null); // 'row' or 'col'
const resizeIndex = ref(null);
const startPos = ref(0);
const startSize = ref(0);

function getResizeHandle(event) {
    const rect = canvasRef.value.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // Check for column handles
    let accWidth = tableX.value;
    for (let i = 0; i < columns; i++) {
        accWidth += cellWidths.value[i];
        if (Math.abs(x - accWidth) < 5) {
            return { type: 'col', index: i };
        }
    }
    // Check for row handles
    let accHeight = tableY.value;
    for (let i = 0; i < rows; i++) {
        accHeight += cellHeights.value[i];
        if (Math.abs(y - accHeight) < 5) {
            return { type: 'row', index: i };
        }
    }
    return null;
}

function handleMouseDown(event) {
    const handle = getResizeHandle(event);
    if (handle) {
        resizing.value = true;
        resizeType.value = handle.type;
        resizeIndex.value = handle.index;
        startPos.value = handle.type === 'col' ? event.clientX : event.clientY;
        startSize.value = handle.type === 'col'
            ? cellWidths.value[handle.index]
            : cellHeights.value[handle.index];
        event.preventDefault();
    }
}

function handleMouseMove(event) {
    if (resizing.value) {
        handleMouseMoveResize(event);
        return;
    }
    // Normal hover logic
    const rect = canvasRef.value.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    const cell = get_cell_at_position(
        x,
        y,
        tableX.value,
        tableY.value,
        Float64Array.from(cellWidths.value),
        Float64Array.from(cellHeights.value),
        rows, columns
    );
    if (cell && cell.row !== undefined && cell.col !== undefined) {
        hoveredCell.value = { row: cell.row, col: cell.col };
    } else {
        hoveredCell.value = null;
    }
    updateHighlight();
}

function handleMouseMoveResize(event) {
    if (!resizing.value) return;
    const delta = (resizeType.value === 'col')
        ? event.clientX - startPos.value
        : event.clientY - startPos.value;
    if (resizeType.value === 'col') {
        cellWidths.value[resizeIndex.value] = Math.max(20, startSize.value + delta);
    } else {
        cellHeights.value[resizeIndex.value] = Math.max(10, startSize.value + delta);
    }
    initializeTable();
    updateHighlight();
}

function handleMouseUp() {
    if (resizing.value) {
        resizing.value = false;
        resizeType.value = null;
        resizeIndex.value = null;
    }
}

function handleMouseOut() {
    hoveredCell.value = null;
    updateHighlight();
}

function initializeTable() {
    if (!canvasRef.value) return;
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);

    // Calculate total width/height
    const displayWidth = cellWidths.value.reduce((a, b) => a + b, 0) + (2 * tableX.value);
    const displayHeight = cellHeights.value.reduce((a, b) => a + b, 0) + (2 * tableY.value);

    canvasRef.value.style.width = `${displayWidth}px`;
    canvasRef.value.style.height = `${displayHeight}px`;
    canvasRef.value.width = displayWidth * dpr;
    canvasRef.value.height = displayHeight * dpr;

    const context = canvasRef.value.getContext('2d');
    context.setTransform(1, 0, 0, 1, 0, 0); // Reset transform
    context.scale(dpr, dpr);

    draw_table(
        canvasRef.value,
        tableX.value,
        tableY.value,
        displayWidth,
        displayHeight,
        Float64Array.from(cellWidths.value),
        Float64Array.from(cellHeights.value),
        rows, columns
    );
}

function updateHighlight() {
    if (!canvasRef.value) return;
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
    // For simplicity, just redraw the table and highlight on top
    initializeTable();
    if (hoveredCell.value && hoveredCell.value.row >= 0 && hoveredCell.value.col >= 0) {
        highlight_with_mode(
            canvasRef.value,
            hoveredCell.value.row,
            hoveredCell.value.col,
            tableX.value,
            tableY.value,
            Float64Array.from(cellWidths.value),
            Float64Array.from(cellHeights.value),
            rows,
            columns,
            "rgba(0, 123, 255, 0.2)",
            currentHighlightMode.value
        );
    }
}

onMounted(async () => {
    await init();
    initializeTable();
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
});
onUnmounted(() => {
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
});

const colSplitterPositions = computed(() => {
  let positions = [];
  let acc = tableX.value;
  for (let i = 0; i < columns - 1; i++) {
    acc += cellWidths.value[i];
    positions.push(acc - 2);
  }
  return positions;
});
const rowSplitterPositions = computed(() => {
  let positions = [];
  let acc = tableY.value;
  for (let i = 0; i < rows - 1; i++) {
    acc += cellHeights.value[i];
    positions.push(acc - 2);
  }
  return positions;
});
const tableWidth = computed(() =>
  cellWidths.value.reduce((a, b) => a + b, 0)
);
const tableHeight = computed(() =>
  cellHeights.value.reduce((a, b) => a + b, 0)
);

function startColResize(colIdx, event) {
  resizing.value = true;
  resizeType.value = 'col';
  resizeIndex.value = colIdx;
  startPos.value = event.clientX;
  startSize.value = cellWidths.value[colIdx];
  event.preventDefault();
}
function startRowResize(rowIdx, event) {
  resizing.value = true;
  resizeType.value = 'row';
  resizeIndex.value = rowIdx;
  startPos.value = event.clientY;
  startSize.value = cellHeights.value[rowIdx];
  event.preventDefault();
}
</script>

<style lang="scss" scoped>
.canvas-wrapper {
    width: 100%;
    height: 100%;
    overflow: auto;
    border: 1px solid #ccc;
    position: relative;
    canvas {
        display: block;
        position: absolute;
        top: 0;
        left: 0;
    }
}
.splitter {
  position: absolute;
  z-index: 10;
  background: transparent;
  transition: background 0.2s;
}
.splitter-vertical {
  width: 5px;
  cursor: col-resize;
  top: 0;
}
.splitter-horizontal {
  height: 5px;
  cursor: row-resize;
  left: 0;
}
.splitter:hover, .splitter.active {
  background: rgba(0, 123, 255, 0.2);
  border-radius: 2px;
}
</style>
