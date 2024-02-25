# WebAssembly Actors: From Cloud to Edge

The course
https://learning.edx.org/course/course-v1:LinuxFoundationX+LFD134x+1T2024/home

Course files
https://github.com/lftraining/LFD134-labs


## Chapter 1

### Tools and docs

WABT (WebAssembly Binary Toolkit)
https://github.com/WebAssembly/wabt
- wat2wasm
- wasm2wat
- etc.

wasmtime WebAssembly runtime
https://wasmtime.dev/

wasmer WebAssembly runtime
https://github.com/wasmerio/wasmer

wasmergo (WebAssembly runtime fo go based on wasmer)
https://github.com/wasmerio/wasmer-go

WabAssembly spec
https://webassembly.github.io/spec/

Why Rust and WebAssembly
https://rustwasm.github.io/book/why-rust-and-webassembly.html


### Run WASM with go runner
```
go mod init toerunner.go 
go mod tidy
wat2wasm tictactoe.wat 
go run toerunner.go 
```


### wat instructions
- loop - The only loop instruction in WebAssembly. There are no “for” or “while” instructions. The loop can be branched to with br, and doing so will jump to the beginning of the loop.
- br - An unconditional branch instruction that can branch to a loop, block, or if statement.
- if - A conditional instruction that can optionally return a value.
- br_if - A conditional branch instruction.
- block - A statement that can be branched out of with br, but branching to a block will jump to the end of the block.
- Call - A statement which calls a function.

### new wat instruction 
- memory.size - Returns the size, in pages, of the module’s current linear memory limit.
- memory.grow - Requests that memory be expanded by the given number of pages.
- memory.init - Copies data from a passive data segment (U32) into memory.
- memory.copy -  Copies data from a source memory region.
- memory.fill - Sets all values in a region to a given byte.
- (type).store - Stores a value of the given data type (e.g. i32) at the given memory location
- (type).load - Retrieves a value of the given data type from the given memory location.




## Chapter 2

Rust and WebAssembly book
https://rustwasm.github.io/docs/book/

MDN WebAssembly doc
https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

### Building wasm from rust

Data operations in Rust
- Move
- Copy
- Clone
- Borrow

```
rustup target list
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```


### calculator project
```
cargo new --lib calc
cargo build --target wasm32-unknown-unknown
```

Add this to the Cargo.toml file
```
[lib]
crate-type = ["cdylib"]
```
Add this macro on your function for it not to be removed by compiler optimization (unused functino ?)
```
#[no_mangle]
```
Check the function add is being exported
```
wasm2wat target/wasm32-unknown-unknown/release/calc.wasm
```
Invoke it 
```
wasmtime target/wasm32-unknown-unknown/release/calc.wasm --invoke add 12 23
wasmtime target/wasm32-unknown-unknown/release/calc.wasm --invoke sub 12 23
wasmtime target/wasm32-unknown-unknown/release/calc.wasm --invoke div 1200 23
wasmtime target/wasm32-unknown-unknown/release/calc.wasm --invoke mul 1200 23
```

### Word counter project

```
cargo new --bin wordcounter
```
Run native build
```
cargo build
cargo run -- ../../README.md
```
Run wasi build
```
cargo build --target wasm32-wasi
wasmtime --dir=../../ target/wasm32-wasi/debug/wordcounter.wasm ../../README.md
```

### Tic-tac-toe project
use unsafe block to allow mutation of static global variable 

```
cargo build --target wasm32-unknown-unknown
```
change path in toerunnger.go file
```
go run toerunnger.go
```

### WASI (WesAsembly System Interface)

https://wasi.dev/


## Chapter 3

### Module
The module is the raw bytes, or the compiled WebAssembly instructions, data, parameters, and custom sections.

### Module Instances
These represent runtime instances of modules. The state of a module instance can change over time and be disposed or recreated.

### Store
The store is a container for all of the global state within a host runtime. Its main job is to maintain references to instances, such as tables, functions, memories, and globals.

### Memory Instances
A managed chunk of linear memory available to a given module. According to the specification, memory instances are always found in sizes that are multiples of the WebAssembly page size, which is 64KB. However, in practicality, you may find fit-for-purpose runtimes (such as those tailored for IoT or specific hardware) using smaller pages or with alternative memory management schemes.

### Additional Instances (Tables, Functions, Globals)
As with the other abstractions within a module; tables, functions, and globals are also instantiated and can change at runtime.

### Exports
Functions exported by the module.

### Imports
Imports from external modules, made available for execution within the WebAssembly module instance. Of all the areas where host runtimes differ, their treatment and support for imports and exports often has the greatest divergence.


### Run wasm calculator in the browser

Create index.html file loading index.js
Create this index.js to run calc.wasm (from chapter 2)
```
let importObject

WebAssembly.instantiateStreaming(fetch('calc.wasm') ,importObject)
.then( (obj) => console.log(obj.instance.exports.add(12, 23)))
```

Run static server with 
```
python -m http.server 8000
```

### Run wasm in Rust host

engines choice:
- Wasm3
- Wasmtime
- Wasmer

We'll use wasmer. Note from wasm3 github page: "I regret to inform the community that since my house was destroyed by russians who invaded my country, Wasm3 will enter a minimal maintenance phase".

Add this to Cargo.toml
```
[dependencies]
wasmer = "4.2.5"
```

We need to read the wasm file into a u8 vector, then create a store and a module from store and bytes vector. Then we can instantiate the module passing the store, the module and an empty imports object. We can then retrieve the calc.wasm exported functions from the instance and use it with the call method.


### Run wasm in Go host 

See chpter 1 to install wasmer package

### Interactive WebAssembly lab

create an interactive calculator