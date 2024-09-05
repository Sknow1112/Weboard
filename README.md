# Weboard

Weboard is a collaborative whiteboard application built with Rust and WebSocket technology. It allows multiple users to draw on an infinite canvas in real-time.

## Features

- Real-time collaboration
- Infinite canvas
- Color selection (using color picker or hex input)
- Eraser tool
- Clear canvas option
- Zoom in/out functionality

## Running the Application

1. Make sure you have Rust and Docker installed on your system.
2. Clone this repository.
3. Build the Docker image:
   ```
   docker build -t weboard .
   ```
4. Run the Docker container:
   ```
   docker run -p 7860:7860 weboard
   ```
5. Open your web browser and navigate to `http://localhost:7860`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
