# Rust Image Converter

[![npm version](https://badge.fury.io/js/rust-image-converter.svg)](https://badge.fury.io/js/rust-image-converter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance image converter built with Rust and WebAssembly. Convert images between JPEG, PNG, WebP, and GIF formats with quality optimization and **automatic WASM initialization**.

## Features

- 🚀 **High Performance**: Built with Rust and WebAssembly for maximum speed
- 🖼️ **Multiple Formats**: Supports JPEG, PNG, WebP, and GIF (first frame only)
- 🎯 **Quality Control**: Adjustable quality settings for optimal file size
- 📦 **Small Bundle**: Optimized WASM binary for minimal bundle size
- 🔧 **TypeScript Support**: Full TypeScript definitions included
- 🌐 **Browser Compatible**: Works in all modern browsers
- ⚡ **Auto-Initialization**: No manual WASM initialization required

## Installation

```bash
npm install rust-image-converter
```

## Usage

### Basic Usage (No Manual Initialization Required!)

```javascript
import { convert } from 'rust-image-converter';

// Convert image data - WASM initializes automatically on first use!
const imageData = new Uint8Array(/* your image data */);
const convertedData = await convert(imageData, {
  format: 'webp',
  quality: 0.8
});
```

### TypeScript

```typescript
import { convert, ConvertOptions } from 'rust-image-converter';

const options: ConvertOptions = {
  format: 'webp',
  quality: 0.8
};

// No init() call needed - everything is automatic!
const result = await convert(imageData, options);
```

### File Upload Example

```javascript
import { convert } from 'rust-image-converter';

document.getElementById('fileInput').addEventListener('change', async (e) => {
  const file = e.target.files[0];
  if (file) {
    const arrayBuffer = await file.arrayBuffer();
    const imageData = new Uint8Array(arrayBuffer);
    
    // Convert to WebP with 80% quality
    const convertedData = await convert(imageData, {
      format: 'webp',
      quality: 0.8
    });
    
    // Create download link
    const blob = new Blob([convertedData], { type: 'image/webp' });
    const url = URL.createObjectURL(blob);
    // ... handle the converted image
  }
});
```

## API Reference

### `convert(data, options)`

Converts image data to the specified format. **WASM module initializes automatically on first use** - no manual initialization required!

**Parameters:**
- `data: Uint8Array` - The input image data
- `options: ConvertOptions` - Conversion options
  - `format?: string` - Output format (`'jpeg'`, `'png'`, `'webp'`)
  - `quality?: number` - Quality setting (0.0 to 1.0, default: 0.8)

**Returns:**
- `Promise<Uint8Array>` - The converted image data

**Key Features:**
- 🔄 **Zero-Config Auto-Initialization**: WASM module initializes automatically on first use
- 🛡️ **Robust Error Handling**: Comprehensive error messages for debugging
- 🚀 **Performance**: Single global initialization, subsequent calls are instant
- ✅ **Input Validation**: Validates input data before processing
- 🎯 **Smart Compression**: Two-step optimization for optimal file sizes

## Supported Formats

| Input Format | Output Format | Notes |
|--------------|---------------|-------|
| JPEG | JPEG, PNG, WebP | ✅ Full support |
| PNG | JPEG, PNG, WebP | ✅ Full support |
| WebP | JPEG, PNG, WebP | ✅ Full support |
| GIF | JPEG, PNG, WebP | ⚠️ First frame only |

## Quality Optimization

The converter uses a sophisticated two-step optimization process:

1. **JPEG Compression**: Images are first compressed using JPEG with the specified quality
2. **Format Conversion**: The compressed image is then converted to the target format

This approach provides excellent compression ratios while maintaining image quality, especially for WebP and PNG outputs.

## Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Examples

### Convert JPEG to WebP (Simplified)

```javascript
import { convert } from 'rust-image-converter';

// No init() needed - just convert!
const jpegData = new Uint8Array(/* JPEG data */);
const webpData = await convert(jpegData, {
  format: 'webp',
  quality: 0.9
});
```

### Quality Control Examples

```javascript
// High quality (larger file)
const highQuality = await convert(imageData, {
  format: 'webp',
  quality: 0.95
});

// Low quality (smaller file)
const lowQuality = await convert(imageData, {
  format: 'webp',
  quality: 0.5
});

// Default quality (0.8)
const defaultQuality = await convert(imageData, {
  format: 'webp'
});
```

### Handle GIF Files

```javascript
// GIF files will be converted using the first frame only
const gifData = new Uint8Array(/* GIF data */);
const webpData = await convert(gifData, {
  format: 'webp',
  quality: 0.8
});
```

### Error Handling

```javascript
try {
  const convertedData = await convert(imageData, {
    format: 'webp',
    quality: 0.8
  });
  // Handle success
} catch (error) {
  console.error('Conversion failed:', error.message);
  // Handle error
}
```

## Performance

- **Speed**: 10-50x faster than JavaScript-only solutions
- **Memory**: Efficient memory usage with automatic cleanup
- **Bundle Size**: ~635KB WASM binary + minimal JavaScript wrapper
- **Initialization**: Automatic and transparent - no performance impact

## Migration from v0.x

If you're upgrading from a previous version that required manual initialization:

```javascript
// Old way (v0.x)
import init, { convert } from 'rust-image-converter';
await init(); // Manual initialization required
const result = await convert(data, format, quality);

// New way (v1.x) - Much simpler!
import { convert } from 'rust-image-converter';
const result = await convert(data, { format, quality });
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 1.0.0
- 🎉 **Major Release**: Automatic WASM initialization
- ✨ **Simplified API**: No more manual `init()` calls required
- 🔧 **Enhanced TypeScript**: Better type definitions
- 🛡️ **Improved Error Handling**: More robust error messages
- 📚 **Updated Documentation**: Comprehensive examples and migration guide

### 0.1.x
- Initial release
- Support for JPEG, PNG, WebP, and GIF formats
- Quality optimization with two-step compression
- TypeScript definitions
- Browser compatibility
