# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

- **Build release version**: `cargo build --release`
- **Run in development (bypass OS check)**: `cargo run -- --bypass`
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`
- **Clean build artifacts**: `cargo clean`

## Project Architecture

This is a Rust-based SCADA (Supervisory Control and Data Acquisition) simulation application that demonstrates legacy industrial control system compatibility issues.

### Core Components

**Main Application (`src/main.rs`)**:
- Windows-only application that enforces OS version compatibility (Windows 7 or earlier)
- Uses Windows API (`winapi` crate) for OS version detection via `GetVersionExW` and `RtlGetVersion`
- Embeds a webview (`web-view` crate) to display the SCADA interface
- Includes `--bypass` flag for development on modern systems

**SCADA Interface (`src/scada_ui.html`)**:
- **Critical**: Must maintain IE8 compatibility - uses extensive restrictions on CSS3, ES5+, and HTML5 features
- Implements a realistic industrial control interface with:
  - Process mimic diagrams showing reactor, pumps, valves, and piping
  - Real-time gauges (temperature, pressure, flow) with rotating needles
  - LED status indicators with blinking animations
  - Alarm management system with priority levels and acknowledgment
  - Digital displays showing process values
  - Trend charts using HTML5 Canvas (with IE8 fallbacks)
  - Motor status monitoring
  - Tank level visualization

**Build Configuration**:
- `build.rs`: Windows resource compilation for executable metadata and icon
- `rust-toolchain.toml`: Pins Rust version to 1.77.0 for consistency
- Release profile optimized for size (`opt-level = "z"`, LTO enabled)

**Web Assets (`web/` directory)**:
- Company website files (HTML, CSS, images)
- Visual C++ redistributables for Windows compatibility

### Key Technical Constraints

1. **IE8 Compatibility**: The SCADA UI must work in Internet Explorer 8, requiring:
   - No CSS3 features (flexbox, transforms, etc.)
   - No ES5+ JavaScript (no `addEventListener`, `querySelector`, array methods)
   - Use of IE-specific filters for animations and transformations

2. **Windows Version Enforcement**: Application intentionally fails on Windows 8+ to simulate legacy industrial software constraints

3. **Industrial Realism**: The interface simulates real SCADA systems with proper industrial terminology, color coding, and layout patterns

## Development Notes

- Use `--bypass` flag during development to skip OS version checks
- The SCADA interface is entirely self-contained in the HTML file with inline CSS and JavaScript
- All process values are simulated with random variations to demonstrate a live industrial system
- The web-view component displays the HTML interface in a native window that starts maximized