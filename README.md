# WebAssembly Learning Project

A hands-on exploration of WebAssembly (WASM) outside the browser environment, focusing on runtime integration and system interfaces.

## 📁 Project Structure

```
├── wat-modules/          # WebAssembly Text format modules
├── wasmtime-as-lib/      # Rust integration with Wasmtime runtime
└── wasi-experiments/     # WASI and WASI-NN explorations
```

## 🎯 Learning Objectives

This repository serves as a practical learning journey through WebAssembly's capabilities beyond web browsers, exploring:

- **WebAssembly Text Format (WAT)**: Hand-written WASM modules for understanding core concepts
- **Runtime Integration**: Embedding WebAssembly in Rust applications using Wasmtime
- **System Interfaces**: Experimenting with WASI (WebAssembly System Interface) and specialized interfaces like WASI-NN

## 📂 Folder Details

### `wat-modules/`
Contains WebAssembly Text format (`.wat`) files demonstrating fundamental WASM concepts:
- Basic arithmetic operations
- Memory management
- Function exports and imports
- Control flow structures

### `wasmtime-as-lib/`
Rust source code showcasing Wasmtime integration:
- Loading and instantiating WASM modules
- Host function implementations
- Memory sharing between host and WASM
- Error handling and debugging techniques

### `wasi-experiments/`
Explorations of WebAssembly System Interface standards:
- **WASI**: File system access, environment variables, command-line arguments
- **WASI-NN**: Neural network inference capabilities
- Other emerging WASI proposals and extensions

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- [WebAssembly Binary Toolkit (WABT)](https://github.com/WebAssembly/wabt) for WAT compilation
- [Wasmtime CLI](https://wasmtime.dev/) (optional, for standalone testing)

### Building and Running

1. **Compile WAT modules:**
   ```bash
   cd wat-modules
   wat2wasm example.wat -o example.wasm
   ```

2. **Run Rust integration examples:**
   ```bash
   cd wasmtime-as-lib
   cargo run --example basic_integration
   ```

3. **Test WASI experiments:**
   ```bash
   cd wasi-experiments
   cargo run --example file_operations
   ```

## 📚 Learning Resources

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [Wasmtime Documentation](https://docs.wasmtime.dev/)
- [WASI Specification](https://wasi.dev/)
- [WebAssembly Text Format Reference](https://webassembly.github.io/spec/core/text/index.html)

## 🛠️ Development Notes

This is a learning-focused repository where each folder represents a different aspect of WebAssembly development. The progression from WAT modules to runtime integration to system interfaces provides a comprehensive understanding of WebAssembly's capabilities in server-side and systems programming contexts.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🤝 Contributing

This is primarily a personal learning project, but suggestions, improvements, and additional examples are welcome! Feel free to open issues or submit pull requests.

---

*Learning WebAssembly one module at a time* 🦀⚡