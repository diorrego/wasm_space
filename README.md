# Wasm Space Project

![Wasm Space](https://ik.imagekit.io/dior7woku/space.gif?updatedAt=1720232105603)

## Description

This project demonstrates the use of Rust and WebAssembly to create an interactive web application. The "Wasm Space" simulates a starry sky where stars move away from the cursor.

## Technologies

- Rust
- WebAssembly
- wasm-bindgen
- Parcel

## Project Structure

- `src/`: Rust source files.

- `wasm_space_parcel/`: Contains the web application using the generated WASM module.

## Setup

To get this project up and running, follow these steps:

### Prerequisites

- Install [Rust and Cargo](https://rustup.rs/)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install [Node.js and npm](https://nodejs.org/en/)

### Building the Project

1. Navigate to the project root and build the WASM module:

```
wasm-pack build --target web
```

2. Move to the wasm_space_parcel directory:

```
cd wasm_space_parcel
```

3. Install JavaScript dependencies:

```
npm install
```

4. Start the development server:

```
npm run start
```

Open the application in your browser at: [http://localhost:1234](http://localhost:1234)

## Author

- Diego Orrego
- GitHub: [diorrego](https://github.com/diorrego)
