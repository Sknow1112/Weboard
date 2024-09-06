const canvas = document.getElementById("whiteboard");
const ctx = canvas.getContext("2d");
const colorPicker = document.getElementById("color-picker");
const sizeSlider = document.getElementById("size-slider");
const clearBtn = document.getElementById("clear-btn");

let isDrawing = false;
let currentPath = [];

canvas.width = window.innerWidth;
canvas.height = window.innerHeight - 50;

const ws = new WebSocket(`ws://${window.location.host}/ws`);

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  if (message.Update) {
    redrawWhiteboard(message.Update);
  } else if (message.Clear) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
  }
};

function redrawWhiteboard(actions) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  for (const action of actions) {
    drawPath(action);
  }
}

function drawPath(action) {
  ctx.beginPath();
  ctx.strokeStyle = action.color;
  ctx.lineWidth = action.size;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";
  for (let i = 0; i < action.points.length; i++) {
    const point = action.points[i];
    if (i === 0) {
      ctx.moveTo(point.x, point.y);
    } else {
      ctx.lineTo(point.x, point.y);
    }
  }
  ctx.stroke();
}

canvas.addEventListener("mousedown", startDrawing);
canvas.addEventListener("mousemove", draw);
canvas.addEventListener("mouseup", stopDrawing);
canvas.addEventListener("mouseout", stopDrawing);

function startDrawing(e) {
  isDrawing = true;
  currentPath = [];
  const point = getPoint(e);
  currentPath.push(point);
  ctx.beginPath();
  ctx.moveTo(point.x, point.y);
}

function draw(e) {
  if (!isDrawing) return;

  const point = getPoint(e);
  currentPath.push(point);

  // Clear the canvas and redraw the current path
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  redrawWhiteboard([]); // Redraw all previous actions
  drawCurrentPath();
}

function drawCurrentPath() {
  ctx.beginPath();
  ctx.strokeStyle = colorPicker.value;
  ctx.lineWidth = sizeSlider.value;
  ctx.lineCap = "round";
  ctx.lineJoin = "round";
  for (let i = 0; i < currentPath.length; i++) {
    const point = currentPath[i];
    if (i === 0) {
      ctx.moveTo(point.x, point.y);
    } else {
      ctx.lineTo(point.x, point.y);
    }
  }
  ctx.stroke();
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
  currentPath = [];
}

function getPoint(e) {
  const rect = canvas.getBoundingClientRect();
  return {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top,
  };
}

clearBtn.addEventListener("click", () => {
  ws.send(JSON.stringify({ Clear: null }));
});

window.addEventListener("resize", () => {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight - 50;
  redrawWhiteboard([]);
});
