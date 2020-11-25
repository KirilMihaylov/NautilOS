# NautilOS
UEFI-compatible Operating System written in Rust.
### Notes
* The default target of the project is `x86_64-unknown-uefi`.
* The recommended editor is Visual Studio Code as this project comes with configuration for it.
* The recommended assisting tool is `rust-analyzer` as it is configured through Visual Studio Code's configuration file.
## Building
To build the project, run:
```
cargo build_uefi [--target <target triple>] [<additional arguments>]
```
## Documentation
To build the project's documentation, run:
```
cargo doc_uefi [--target <target triple>] [<additional arguments>]
```
## Clippy
To run `clippy`, run:
```
cargo doc_uefi [--target <target triple>] [<additional arguments>]
```
