# NautilOS
UEFI-compatible Operating System written in Rust.
### Notes
* The default target of the project is `x86_64-unknown-uefi`.
* The recommended editor is Visual Studio Code as this project comes with configuration for it.
* Issues may occur (e.g.: stuck forever) while building the project with the `nightly-x86_64-pc-windows-gnu` toolchain on Microsoft Windows. Building with the MSVC toolchain is recommended while on Microsoft Windows.
## Building
To build the project, run:
```
cargo build [--target <target triple>] [<additional arguments>] [--release]
```
## Check
To build the project, run:
```
cargo check [--target <target triple>] [<additional arguments>] [--release]
```
## Documentation
To build the project's documentation, run:
```
cargo doc [--target <target triple>] [<additional arguments>] [--release]
```
## Clippy
To run `clippy`, run:
```
cargo clippy [--target <target triple>] [<additional arguments>] [--release]
```
