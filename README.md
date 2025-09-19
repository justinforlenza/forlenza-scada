# Forlenza Industrial SCADA Control System

The Forlenza Industrial SCADA Control System v2.1 simulates a realistic industrial control interface, designed to demonstrate common compatibility issues found in legacy SCADA systems. This application is ideal for virtualization demonstrations and educational purposes.

## Features

- **Legacy Compatibility Simulation:** Enforces OS version checking, only running on Windows 7 or earlier to mimic real-world legacy SCADA constraints.
- **Industrial Control UI:** Includes a sample HTML-based interface (`src/scada_ui.html`) for operator interaction.
- **Rust Implementation:** Built with Rust for performance and reliability.
- **Demonstrates Common Issues:** Showcases typical problems encountered when running legacy SCADA software on modern systems.

## Technologies Used

- **Rust**: Main application logic and system integration.
- **Webview**: For embedding the HTML UI in a native window.

## Usage

1. **System Requirements:**
	- Windows 7 or earlier (application will not run on newer OS versions)
	- Rust toolchain (for building from source)

2. **Running the Application:**
	- Download or clone the repository.
	- Build the project using Cargo:
	  ```pwsh
	  cargo build --release
	  ```
	- Run the executable from the `target/release` directory on a compatible Windows system.

3. **Virtualization Demo:**
	- Use a virtual machine running Windows 7 to demonstrate the application and its compatibility checks.

## Project Structure

- `src/main.rs`: Main Rust source file.
- `src/scada_ui.html`: Industrial control system UI.
- `Cargo.toml`: Rust project configuration and dependencies.

## Build Instructions

1. Install [Rust](https://www.rust-lang.org/tools/install) if not already installed.
2. Open a terminal in the project directory.
3. Run:
	```pwsh
	cargo build --release
	```
4. The compiled binary will be located in `target/release/forlenza-scada.exe`.

## License

This project is provided for demonstration and educational purposes. See the repository for license details.


