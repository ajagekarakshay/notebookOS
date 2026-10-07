# NotebookOS

NotebookOS is a local-first research notebook built with Rust and GPUI Kit. The product direction covers structured documents, full-sheet handwriting, spatial canvases, linked research, boards, calendars, and imported meeting notes across desktop and Apple devices.

The first native slice currently provides:

- a workspace shell with Notes, Boards, Calendar, and Meetings destinations;
- a research journal with stable page identity;
- separate document, handwritten-page, and canvas surfaces;
- embedded handwriting blocks inside ordinary documents;
- a full white-sheet handwriting surface with paper and tool controls;
- a page-creation dialog for all three page types.

The handwriting surface is the application structure for the future platform ink engines. Apple Pencil input and vector-stroke persistence are not connected yet.

## Run locally

Install Rust through `rustup`, then run:

```powershell
cargo run -p notebookos
```

The repository pins the Rust toolchain required by the current GPUI dependency.

## Validate

```powershell
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

The product specification lives in [`docs/product-spec.md`](docs/product-spec.md). The interactive design reference is [`design/prototype.html`](design/prototype.html).
