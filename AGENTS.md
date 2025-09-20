# AGENTS.md

This file provides guidance to AI Agents when working with code in this repository.

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

## Educational Context & Design Philosophy

**Project Purpose**: This is an educational tool designed to teach students about virtualization benefits through hands-on experience with legacy software compatibility issues.

**Fictional Company**: "Forlenza Industrial Systems" - established 1987, Pittsburgh-based industrial automation company. All company information is fictional and created for educational purposes only.

**Target Era**: Early-to-mid 2000s industrial software (circa 2007) - intentionally outdated to demonstrate legacy system challenges. Website should reflect the web design standards of 2003-2007 period.

**Forlenza Industrial Systems Company Profile**:
- Founded: 1987 by Anthony Forlenza
- Headquarters: Pittsburgh, Pennsylvania  
- Regional offices: Chicago, Houston, Los Angeles
- Industry: Industrial automation and SCADA solutions
- Key milestone: ISO 9001:2000 certification (2006)
- Partnership: Rockwell Automation integration (2006)
- Website aesthetic: Corporate business site circa 2003-2007

**Key Educational Messages**:
- Legacy industrial systems often cannot be upgraded due to safety certifications, cost, and integration complexity
- Virtualization provides a cost-effective solution for preserving critical legacy functionality
- Real-world industrial software often has strict OS compatibility requirements

## Code Style & Conventions

### Rust Code Guidelines
- Follow standard Rust formatting (`cargo fmt`)
- Use descriptive variable names that reflect industrial terminology
- Include comprehensive error handling for Windows API calls
- Maintain clear separation between OS detection logic and UI display
- Comment any platform-specific or legacy compatibility code

### HTML/CSS/JavaScript Guidelines for SCADA Interface
- **Maintain IE8 compatibility at all costs** - this is a hard requirement
- Use traditional table layouts instead of modern CSS layout methods
- Stick to ES3 JavaScript syntax and avoid modern browser APIs
- Use inline event handlers (`onclick=""`) instead of `addEventListener`
- Implement animations using `setTimeout`/`setInterval` rather than CSS transitions
- Use RGBA color values sparingly (IE8 limited support)

### Early 2000s Website Guidelines (web/ directory)
**DOCTYPE & Standards**: Use HTML 4.01 Transitional or XHTML 1.0 Transitional
```html
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
```

**Typography & Fonts**:
- Primary: Times New Roman, Times, serif (default browser serif)
- Secondary: Arial, Helvetica, sans-serif for headers
- Monospace: Courier New, Courier, monospace for technical specs
- Font sizes: Use pixels (12px, 14px) rather than em/rem units
- Avoid web fonts - stick to system fonts only

**Color Palette (Early 2000s Business)**:
- Background: `#ffffff` (white) or `#f0f0f0` (light gray)
- Text: `#000000` (black) or `#333333` (dark gray)
- Links: `#0000ff` (blue), visited: `#800080` (purple)
- Headers: `#000080` (navy blue), `#800000` (maroon)
- Borders: `#808080` (gray), `#c0c0c0` (silver)
- Highlights: `#ffffcc` (light yellow), `#f5f5f5` (off-white)

**Layout Techniques**:
- Use `<table>` for complex layouts (this was standard practice)
- Minimal CSS - mostly inline styles or simple `<style>` blocks
- No CSS Grid, Flexbox, or modern layout methods
- Use `cellpadding`, `cellspacing`, `border` attributes on tables
- Spacer GIFs for precise spacing (if absolutely necessary)

**Navigation Patterns**:
- Simple horizontal nav bars using tables or basic lists
- Text-based navigation with basic hover effects
- No dropdown menus or complex interactions
- "Site Map" and "Contact Us" prominent
- Copyright notice at bottom

**Content Structure**:
- Heavy use of nested tables for alignment
- Liberal use of `&nbsp;` for spacing
- Basic form styling with minimal CSS
- Image alignment using `align="left"` or `align="right"`
- Multiple columns achieved with table cells

**Period-Appropriate Elements**:
- "Best viewed with [Browser] at [Resolution]" notices
- "Last updated on [Date]" timestamps
- Basic hit counters or "visitor" counters
- Email addresses as `mailto:` links
- Phone numbers prominently displayed
- Fax numbers listed alongside phone numbers

**Technical Specifications Styling**:
- Use definition lists (`<dl>`, `<dt>`, `<dd>`) for spec sheets
- Simple bordered tables for feature comparisons
- Minimal icons - prefer text-based indicators
- Download links with file sizes in parentheses
- "System Requirements" in bordered boxes

**Image Guidelines**:
- Use GIF for simple graphics and logos
- JPEG for photographs (lower quality acceptable)
- No PNG transparency (limited support)
- Alt text should be simple and descriptive
- Images should have borders (`border="1"`) when appropriate

### Industrial UI Standards
- Follow traditional SCADA color conventions:
  - Green: Normal/Running states
  - Red: Alarm/Fault conditions  
  - Yellow: Warning/Caution states
  - Gray: Inactive/Offline equipment
- Use monospace fonts for digital displays and data values
- Implement proper industrial control button styling (raised/pressed effects)
- Include appropriate status indicators and alarm acknowledgment patterns

## Testing Guidelines

### Compatibility Testing
- **Primary Target**: Test on Windows 7 (or VM) to verify full functionality
- **Error Testing**: Verify proper error messages on Windows 8/10/11
- **Browser Testing**: Test SCADA interface in Internet Explorer 8 if possible
- **Bypass Testing**: Ensure `--bypass` flag works correctly for development

### Functional Testing Areas
- OS version detection and error display
- SCADA interface loading and responsiveness
- Real-time data updates and animations
- Button interactions and alert dialogs
- Window sizing and layout behavior

## Common Development Pitfalls to Avoid

### Rust-Specific Issues
- **Don't** update the Rust version beyond 1.77.0 without testing compatibility
- **Don't** add dependencies that break Windows 7 compatibility
- **Don't** modify the Windows API calls without understanding version detection implications
- **Don't** remove the `#![windows_subsystem = "windows"]` attribute (prevents console window)

### SCADA Interface Issues
- **Don't** use modern JavaScript features (ES5+, let/const, arrow functions, modern array methods)
- **Don't** use CSS3 features (flexbox, grid, transforms, transitions, gradients)
- **Don't** use modern HTML5 elements or attributes
- **Don't** break the industrial control system aesthetic with modern UI patterns
- **Don't** use external dependencies or CDN resources (must be self-contained)

### Early 2000s Website Issues
- **Don't** use modern HTML5 semantic elements (`<header>`, `<nav>`, `<section>`, `<article>`)
- **Don't** use CSS3 features (border-radius, box-shadow, gradients, transforms)
- **Don't** use modern color formats (HSL, CSS variables, alpha transparency)
- **Don't** implement responsive design or mobile-first approaches
- **Don't** use modern JavaScript (ES5+, jQuery, modern frameworks)
- **Don't** include social media integration or modern web APIs
- **Don't** use modern image formats (WebP, SVG for complex graphics)
- **Don't** implement accessibility features beyond basic alt text
- **Don't** use external CSS frameworks or icon fonts

## File Organization & Naming

### Required Files
- `src/main.rs` - Main application entry point
- `src/scada_ui.html` - Complete SCADA interface (self-contained)
- `src/icon.ico` - Application icon for Windows executable
- `Cargo.toml` - Project configuration with locked dependencies
- `build.rs` - Windows resource compilation
- `rust-toolchain.toml` - Rust version specification

### Optional/Supporting Files
- `web/index.html` - Company website homepage (early 2000s business style)
- `web/*.html` - Additional company website pages (download, support, about)
- `web/style.css` - Period-appropriate CSS (minimal, table-based layouts)
- `web/images/` - Company logos, product images (GIF/JPEG only)
- `README.md` - Educational documentation and setup instructions
- `LICENSE` - MIT license for educational use

## Version Control Guidelines

### What to Commit
- All source code and configuration files
- Documentation and educational materials
- Web assets and company website files
- Build scripts and resource files

### What NOT to Commit
- `target/` directory (Rust build artifacts)
- `Cargo.lock` (let it generate based on Cargo.toml)
- IDE-specific configuration files (`.vscode/`, `.idea/`, etc.)
- Temporary files or OS-specific artifacts

## Deployment & Distribution

### Creating Release Builds
1. Ensure building on Windows (required for proper resource compilation)
2. Run `cargo build --release` to create optimized executable
3. Test executable on both Windows 7 and modern Windows versions
4. Verify proper error messages and UI functionality
5. Package with any required redistributables

### Educational Distribution
- Provide clear instructions for setting up Windows 7 VMs
- Include troubleshooting guide for common compatibility issues
- Document the educational objectives and learning outcomes
- Emphasize the mock/educational nature of the software

## Support & Troubleshooting

### Common Issues
- **Build failures on non-Windows**: This is Windows-only software by design
- **Missing icon errors**: Ensure `src/icon.ico` exists before building
- **Webview display issues**: Check that HTML is self-contained with no external dependencies
- **OS detection problems**: May require administrative privileges on some systems
- **Website anachronisms**: Ensure web assets maintain early 2000s styling and don't use modern techniques
- **Period authenticity**: Company timeline and technology references should be consistent with 2003-2007 era

### Debug Information
- Use `cargo run -- --bypass` for development testing
- Enable debug mode in webview for JavaScript troubleshooting
- Check Windows event logs for application-level errors
- Verify Windows API function availability on target systems