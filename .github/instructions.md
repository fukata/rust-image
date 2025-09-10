# Instructions for rust-image

This document provides comprehensive instructions for working with the rust-image project, a WebAssembly-based image resizing tool built with Rust.

## Project Overview

rust-image is a web application that allows users to resize images directly in their browser using WebAssembly (WASM) technology. The application provides a fast, client-side image processing solution with offline capabilities.

### Key Features
- Client-side image resizing using Rust + WebAssembly
- Support for multiple image formats (PNG, JPEG, GIF)
- Offline functionality via Service Worker
- Responsive web interface
- EXIF orientation correction
- Image processing history

## Prerequisites

Before working with this project, ensure you have the following installed:

### Required Tools
1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack** (for building WebAssembly packages)
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **A modern web browser** that supports WebAssembly

## Installation & Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/fukata/rust-image.git
   cd rust-image
   ```

2. **Install dependencies**
   The Rust dependencies are defined in `Cargo.toml` and will be automatically installed during the build process.

## Build Process

### Development Build
```bash
wasm-pack build --target web
```

### Production Build
```bash
wasm-pack build --target web --release
```

This will generate the following files in the `pkg/` directory:
- `rust_image.js` - JavaScript bindings
- `rust_image_bg.wasm` - WebAssembly module
- Other supporting files

## Development Workflow

### 1. Local Development
1. Build the WebAssembly module:
   ```bash
   wasm-pack build --target web
   ```

2. Serve the files using a local web server:
   ```bash
   # Using Python 3
   python -m http.server 8000
   
   # Using Node.js (if you have http-server installed)
   npx http-server -p 8000
   ```

3. Open your browser and navigate to `http://localhost:8000`

### 2. Making Changes

#### Rust Code Changes
- Edit files in the `src/` directory
- The main entry point is `src/lib.rs`
- Rebuild with `wasm-pack build --target web`
- Refresh your browser to test changes

#### Frontend Changes
- Edit `index.html` for UI changes
- Edit `sw.js` for service worker functionality
- No rebuild required for frontend-only changes

### 3. Testing
- Test image resizing functionality with various formats
- Test offline functionality by disconnecting from the internet
- Verify responsive design on different screen sizes
- Test EXIF orientation handling with rotated images

## File Structure

```
rust-image/
├── src/
│   └── lib.rs              # Main Rust code for image processing
├── .github/
│   ├── workflows/
│   │   └── pages.yml       # GitHub Pages deployment
│   └── instructions.md     # This file
├── pkg/                    # Generated WebAssembly files (after build)
├── index.html              # Main web application
├── sw.js                   # Service worker for offline functionality
├── alpine.js               # Alpine.js for reactive frontend
├── logo.png                # Application logo
├── offline.html            # Offline fallback page
├── Cargo.toml              # Rust dependencies and configuration
└── README.md               # Project documentation (Japanese)
```

## Usage Instructions

### For End Users
1. Open the web application in your browser
2. Click or drag-and-drop an image file
3. Adjust the width and height settings
4. Select the output format if desired
5. Click "Resize" to process the image
6. Download the resized image using the download button

### Image Format Support
- **Input formats**: PNG, JPEG, GIF, and other formats supported by the `image` crate
- **Output formats**: PNG, JPEG, GIF

## Contributing

### Code Style
- Follow standard Rust formatting using `rustfmt`
- Use descriptive variable and function names
- Add comments for complex image processing logic

### Submitting Changes
1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test thoroughly
4. Build and verify the WebAssembly module works
5. Commit your changes with descriptive messages
6. Push to your fork and submit a pull request

### Areas for Contribution
- Additional image format support
- Performance optimizations
- UI/UX improvements
- Accessibility enhancements
- Documentation improvements
- Test coverage

## Deployment

### GitHub Pages
The project is configured for deployment to GitHub Pages via the `.github/workflows/pages.yml` workflow.

### Manual Deployment
1. Build the project: `wasm-pack build --target web --release`
2. Upload all files (including the `pkg/` directory) to your web server
3. Ensure the server serves `.wasm` files with the correct MIME type:
   ```
   application/wasm
   ```

## Troubleshooting

### Common Issues

**Build fails with wasm-pack errors:**
- Ensure you have the latest version of wasm-pack installed
- Check that your Rust installation is up to date
- Verify that the `wasm32-unknown-unknown` target is installed:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

**WebAssembly module doesn't load:**
- Check browser developer tools for CORS errors
- Ensure you're serving files via HTTP/HTTPS, not opening `file://` URLs
- Verify the server is configured to serve `.wasm` files

**Image processing fails:**
- Check that the input image format is supported
- Verify the image file is not corrupted
- Check browser console for error messages

## Performance Considerations

- WebAssembly provides near-native performance for image processing
- Large images may take longer to process
- The application runs entirely client-side, so no data is sent to external servers
- Service Worker caching improves offline performance and load times

## Security Notes

- All image processing happens client-side
- No image data is transmitted to external servers
- The application works entirely offline after initial load
- Be cautious when modifying the Service Worker to avoid security vulnerabilities

## License

Please refer to the repository's LICENSE file for licensing information.

## Support

For questions, issues, or contributions:
- Open an issue on GitHub
- Submit a pull request for improvements
- Check existing issues and documentation first

---

**Note**: This project primarily uses Japanese in the user interface and some documentation. The codebase and technical documentation use English for broader accessibility.