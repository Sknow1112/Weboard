const backgroundCanvas = document.getElementById("background-canvas");
const drawingCanvas = document.getElementById("drawing-canvas");
const bgCtx = backgroundCanvas.getContext("2d");
const drawCtx = drawingCanvas.getContext("2d");
const colorPicker = document.getElementById("color-picker");
const sizeSlider = document.getElementById("size-slider");
const clearBtn = document.getElementById("clear-btn");

let isDrawing = false;
let currentPath = [];

function resizeCanvases() {
  const container = document.getElementById("canvas-container");
  const width = container.clientWidth;
  const height = container.clientHeight;

  backgroundCanvas.width = width;
  backgroundCanvas.height = height;
  drawingCanvas.width = width;
  drawingCanvas.height = height;
}

resizeCanvases();

const ws = new WebSocket(`ws://${window.location.host}/ws`);

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  if (message.Update) {
    redrawWhiteboard(message.Update);
  } else if (message.Clear) {
    bgCtx.clearRect(0, 0, backgroundCanvas.width, backgroundCanvas.height);
  }
};

function redrawWhiteboard(actions) {
  bgCtx.clearRect(0, 0, backgroundCanvas.width, backgroundCanvas.height);
  for (const action of actions) {
    drawPath(bgCtx, action);
  }
}

function drawPath(context, action) {
  context.beginPath();
  context.strokeStyle = action.color;
  context.lineWidth = action.size;
  context.lineCap = "round";
  context.lineJoin = "round";
  for (let i = 0; i < action.points.length; i++) {
    const point = action.points[i];
    if (i === 0) {
      context.moveTo(point.x, point.y);
    } else {
      context.lineTo(point.x, point.y);
    }
  }
  context.stroke();
}

drawingCanvas.addEventListener("mousedown", startDrawing);
drawingCanvas.addEventListener("mousemove", draw);
drawingCanvas.addEventListener("mouseup", stopDrawing);
drawingCanvas.addEventListener("mouseout", stopDrawing);

function startDrawing(e) {
  isDrawing = true;
  currentPath = [];
  const point = getPoint(e);
  currentPath.push(point);
  drawCtx.beginPath();
  drawCtx.moveTo(point.x, point.y);
}

function draw(e) {
  if (!isDrawing) return;

  const point = getPoint(e);
  currentPath.push(point);

  drawCtx.clearRect(0, 0, drawingCanvas.width, drawingCanvas.height);
  drawPath(drawCtx, {
    color: colorPicker.value,
    size: parseFloat(sizeSlider.value),
    points: currentPath,
  });
}

function stopDrawing() {
  if (!isDrawing) return;
  isDrawing = false;

  const action = {
    color: colorPicker.value,
    size: parseFloat(sizeSlider.value),
    points: currentPath,
  };

  ws.send(JSON.stringify({ Draw: action }));
  drawCtx.clearRect(0, 0, drawingCanvas.width, drawingCanvas.height);
  drawPath(bgCtx, action);
  currentPath = [];
}

function getPoint(e) {
  const rect = drawingCanvas.getBoundingClientRect();
  return {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top,
  };
}

clearBtn.addEventListener("click", () => {
  ws.send(JSON.stringify({ Clear: null }));
});

window.addEventListener("resize", () => {
  resizeCanvases();
  redrawWhiteboard([]);
});
