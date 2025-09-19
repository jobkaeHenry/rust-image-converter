# Rust Image Converter

High-performance image converter built with Rust and WebAssembly. Convert images between JPEG, PNG, WebP, and GIF formats with quality optimization.

## Features

- 🚀 **High Performance**: Built with Rust and WebAssembly for maximum speed
- 🖼️ **Multiple Formats**: Supports JPEG, PNG, WebP, and GIF (first frame only)
- 🎯 **Quality Control**: Adjustable quality settings for optimal file size
- 📦 **Small Bundle**: Optimized WASM binary for minimal bundle size
- 🔧 **TypeScript Support**: Full TypeScript definitions included
- 🌐 **Browser Compatible**: Works in all modern browsers

## Installation

```bash
npm install rust-image-converter
```

## Usage

### Basic Usage

```javascript
import init, { convert } from 'rust-image-converter';

// Initialize the WASM module
await init();

// Convert image data
const imageData = new Uint8Array(/* your image data */);
const convertedData = convert(imageData, {
  format: 'webp',
  quality: 0.8
});
```

### TypeScript

```typescript
import init, { convert, ConvertOptions } from 'rust-image-converter';

interface ConvertOptions {
  format?: string;
  quality?: number;
}

const options: ConvertOptions = {
  format: 'webp',
  quality: 0.8
};

await init();
const result = convert(imageData, options);
```

## API Reference

### `init()`

Initializes the WebAssembly module. Must be called before using the `convert` function.

```javascript
await init();
```

### `convert(data, options)`

Converts image data to the specified format.

**Parameters:**
- `data: Uint8Array` - The input image data
- `options: ConvertOptions` - Conversion options
  - `format?: string` - Output format (`'jpeg'`, `'png'`, `'webp'`)
  - `quality?: number` - Quality setting (0.0 to 1.0, default: 0.8)

**Returns:**
- `Uint8Array` - The converted image data

## Supported Formats

| Input Format | Output Format | Notes |
|--------------|---------------|-------|
| JPEG | JPEG, PNG, WebP | ✅ Full support |
| PNG | JPEG, PNG, WebP | ✅ Full support |
| WebP | JPEG, PNG, WebP | ✅ Full support |
| GIF | JPEG, PNG, WebP | ⚠️ First frame only |

## Quality Optimization

The converter uses a two-step optimization process:

1. **JPEG Compression**: Images are first compressed using JPEG with the specified quality
2. **Format Conversion**: The compressed image is then converted to the target format

This approach provides excellent compression ratios while maintaining image quality.

## Browser Support

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Examples

### Convert JPEG to WebP

```javascript
import init, { convert } from 'rust-image-converter';

await init();

const jpegData = new Uint8Array(/* JPEG data */);
const webpData = convert(jpegData, {
  format: 'webp',
  quality: 0.9
});
```

### Convert with Quality Control

```javascript
// High quality (larger file)
const highQuality = convert(imageData, {
  format: 'webp',
  quality: 0.95
});

// Low quality (smaller file)
const lowQuality = convert(imageData, {
  format: 'webp',
  quality: 0.5
});
```

### Handle GIF Files

```javascript
// GIF files will be converted using the first frame only
const gifData = new Uint8Array(/* GIF data */);
const webpData = convert(gifData, {
  format: 'webp',
  quality: 0.8
});
```

## Performance

- **Speed**: 10-50x faster than JavaScript-only solutions
- **Memory**: Efficient memory usage with automatic cleanup
- **Bundle Size**: ~635KB WASM binary + minimal JavaScript wrapper

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0
- Initial release
- Support for JPEG, PNG, WebP, and GIF formats
- Quality optimization with two-step compression
- TypeScript definitions
- Browser compatibility
