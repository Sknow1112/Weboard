const canvas = document.getElementById("whiteboard");
const ctx = canvas.getContext("2d");
const colorPicker = document.getElementById("color-picker");
const colorInput = document.getElementById("color-input");
const eraserButton = document.getElementById("eraser");
const clearButton = document.getElementById("clear");
const zoomSlider = document.getElementById("zoom");

let isDrawing = false;
let currentColor = "#000000";
let isEraser = false;
let zoom = 1;
let offsetX = 0;
let offsetY = 0;

const socket = new WebSocket(`ws://${window.location.host}/ws`);

function resize() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  redraw();
}

function redraw() {
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.setTransform(zoom, 0, 0, zoom, offsetX, offsetY);
}

window.addEventListener("resize", resize);
resize();

canvas.addEventListener("mousedown", startDrawing);
canvas.addEventListener("mousemove", draw);
canvas.addEventListener("mouseup", stopDrawing);
canvas.addEventListener("mouseout", stopDrawing);

colorPicker.addEventListener("change", (e) => {
  currentColor = e.target.value;
  isEraser = false;
});

colorInput.addEventListener("change", (e) => {
  const hex = e.target.value.match(/^#?([a-f\d]{6})$/i);
  if (hex) {
    currentColor = `#${hex[1]}`;
    colorPicker.value = currentColor;
    isEraser = false;
  }
});

eraserButton.addEventListener("click", () => {
  isEraser = !isEraser;
  eraserButton.textContent = isEraser ? "Draw" : "Eraser";
});

clearButton.addEventListener("click", () => {
  socket.send(JSON.stringify({ type: "Clear" }));
});

zoomSlider.addEventListener("input", (e) => {
  zoom = parseFloat(e.target.value);
  redraw();
  socket.send(JSON.stringify({ type: "Zoom", value: zoom }));
});

function startDrawing(e) {
  isDrawing = true;
  draw(e);
}

function draw(e) {
  if (!isDrawing) return;

  const rect = canvas.getBoundingClientRect();
  const x = (e.clientX - rect.left - offsetX) / zoom;
  const y = (e.clientY - rect.top - offsetY) / zoom;

  ctx.lineWidth = isEraser ? 20 : 2;
  ctx.lineCap = "round";
  ctx.strokeStyle = isEraser ? "#FFFFFF" : currentColor;

  ctx.lineTo(x, y);
  ctx.stroke();
  ctx.beginPath();
  ctx.moveTo(x, y);

  socket.send(
    JSON.stringify({
      type: "Draw",
      x,
      y,
      color: currentColor,
      isEraser,
    }),
  );
}

function stopDrawing() {
  isDrawing = false;
  ctx.beginPath();
}

socket.onmessage = (event) => {
  const action = JSON.parse(event.data);

  switch (action.type) {
    case "Draw":
      ctx.lineWidth = action.isEraser ? 20 : 2;
      ctx.lineCap = "round";
      ctx.strokeStyle = action.isEraser ? "#FFFFFF" : action.color;
      ctx.lineTo(action.x, action.y);
      ctx.stroke();
      ctx.beginPath();
      ctx.moveTo(action.x, action.y);
      break;
    case "Clear":
      redraw();
      break;
    case "Zoom":
      zoom = action.value;
      zoomSlider.value = zoom;
      redraw();
      break;
  }
};
