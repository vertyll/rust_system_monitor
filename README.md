# Rust System Monitor

A lightweight, cross-platform system monitoring application that displays real-time system metrics in the system tray. Built with Rust for performance and reliability.

## Features

- **System Tray Integration**: Displays system metrics directly in your system tray with custom-generated icons
- **Real-time Monitoring**: Tracks CPU usage, RAM usage, and other system metrics
- **Multi-language Support**: Internationalization (i18n) support with fluent localization
- **Configurable**: Customizable refresh intervals, active monitors, and other settings
- **Modern UI**: Clean, responsive interface built with egui framework
- **Cross-platform**: Works on Windows, macOS, and Linux

## System Requirements

- Windows 10/11, macOS 10.14+, or Linux with system tray support
- Rust (for building from source)

## Installation

### From Source

1. Clone the repository

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Configuration

The application uses a `config.toml` file for configuration. Key settings include:

- **Active Monitors**: Choose which system metrics to display
- **Refresh Intervals**: Configure how often metrics are updated
- **Language**: Set the application language
- **UI Settings**: Customize window size and appearance

## Architecture

The application follows a modular architecture with clear separation of concerns:

- **App Core** (`app.rs`): Main application logic and lifecycle management
- **System Tray** (`tray/`): Tray icon management and menu handling
- **Monitoring** (`monitor/`): System metrics collection and processing
- **UI** (`ui/`): Settings interface and user interactions
- **Configuration** (`config/`): Application settings management
- **Internationalization** (`i18n/`): Multi-language support
- **Error Handling** (`error/`): Centralized error management

### Key Components

- **SystemMonitor**: Collects system metrics using the `sysinfo` crate
- **SystemTray**: Manages tray icons with dynamically generated graphics
- **I18nManager**: Handles localization using the Fluent framework
- **AppConfig**: Manages application configuration with TOML persistence

## Usage

1. **Launch the Application**: Run the executable or use `cargo run`
2. **System Tray**: Look for the application icons in your system tray
3. **Settings**: Right-click any tray icon and select "Settings" to configure the application
4. **Monitoring**: View real-time metrics displayed on the tray icons
5. **Exit**: Right-click and select "Quit" to exit the application

### Tray Icon Features

- Icons display current metric values with custom-generated graphics
- Right-click context menu for quick access to settings and exit
- Icons automatically show/hide based on active monitor configuration
- Hover tooltips provide additional information

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Mikołaj Gawron** - [gawrmiko@gmail.com](mailto:gawrmiko@gmail.com)

## Acknowledgments

- Built with the excellent Rust ecosystem
- Uses the `sysinfo` crate for cross-platform system information
- UI powered by the `egui` immediate mode GUI framework
- Internationalization supported by Mozilla's Fluent framework
