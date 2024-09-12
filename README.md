Weboard is a real-time collaborative whiteboard application built with Rust and WebSockets. It allows multiple users to draw on a shared canvas simultaneously.

## Features

- Real-time drawing synchronization
- Color picker for choosing different colors
- Adjustable pen size
- Clear canvas functionality
- Responsive design

## Running the Application

1. Make sure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Run `cargo build --release` to build the application.
5. Run `cargo run --release` to start the server.
6. Open a web browser and go to `http://localhost:7860` to use the whiteboard.

## Docker Deployment

To deploy the application using Docker:

1. Build the Docker image: `docker build -t weboard .`
2. Run the container: `docker run -p 7860:7860 weboard`

The application will be accessible at `http://localhost:7860`.

## License

This project is open-source. Credit me for use.
