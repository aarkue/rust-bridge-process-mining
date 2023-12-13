# _Python_ 🌉 Rust 🌉 _Java_
WIP Project to allow using a shared Rust library from both Python and Java.

## Structure
- Main (Shared) Rust Library: `process_mining/`
  - Used for:
    - Python Bindings:`python_bridge/`
    - Java Bindings: `java_bridge/`
    - Rust Program: `footbridge/`
  - Additionally:
    - Java Implementation using the Provided Bindings `java_side/`
   
## Main Library
- `EventLog` struct
- Function to add artificial start/end activities to an event log
- Allow building activity projections of logs (+ required structs)

## Java
- Uses `jni` to allow Java code to call shared Rust library
- Object Passing:
  - `byte[]` JSON-encoded data [faster]
  - Writing `File` to disk (containing JSON-encoded data)
- Required Java libraries:
  - `gson` for efficient JSON encoding/decoding
  
## Python
- Uses [maturin](https://github.com/PyO3/maturin) with the corresponding [PyO3 FFI Bindings](https://github.com/PyO3/PyO3)
- Can pass dicts and other types rather easily; but slow for larger data
- JSON-encoding/decoding possible (using faster `orjson`), but still no great passing performance
  - `bytes` encoding/decoding e.g., Event Logs is implemented
- Easier support to construct and use Rust struct from python
  - Used for wrapper structs like `PyBridgeEventLog`; Idea: Do heavy work on Rust side
- Polars: Allows converting PM4Py's pandas DataFrame to a Polars DataFrame; Can then easily be used on Rust side
  - But: Requires Polars dependency (also in python!)


## WASM Proof of Concept
- `pm_wasm/` contains a proof of concept of WASM bindings for the main library, e.g., exposed to JavaScript and executed in the browser.
- After building with `wasm-pack build --target web --release` the `index.html` file can be served (at root): The resulting webpage contains a file input allowing an XES event log to be selected.
After selecting a file, clicking the 'Go' button will execute the generated WASM and discover a Petri net, which will be logged in the console as JSON once the call finishes. Note that the in-browser WASM execution performance is limited, so some patience might be required.  

- For easy usage of `wasm-bindgen-rayon` we use the `parcel` bundler. The dev server can be started with `npx parcel index.html`