# CSS Asset Management in Rust Web Projects

## Current Project Structure Analysis

This project has:
- Root-level `styles.css` (514 lines)
- Example projects with their own `styles.css` (142 lines each)
- Scripts for linking CSS files (`scripts/link-css.sh`)

## Research Findings: Professional Rust Web CSS Patterns

### 1. Cargo Patterns for Asset Management

#### Build Scripts (build.rs)
- Use `build.rs` for asset processing and bundling
- Copy assets to output directory during compilation
- Integrate with bundlers like `trunk` or `wasm-pack`

#### Cargo Workspaces
- Share CSS across workspace members using symbolic links
- Centralize assets in `assets/` directory
- Use `include!` macros for CSS imports in Rust

### 2. CSS Sharing Between Library and Examples

#### Best Practice: Single Source of Truth
```
project/
├── assets/
│   └── styles.css          # Single source of truth
├── library/
│   └── Cargo.toml          # Includes assets
├── examples/
│   └── example1/
│       ├── public/         # Symlinks to assets
│       └── Cargo.toml
```

#### Implementation Pattern:
```toml
# In library Cargo.toml
[package]
include = [
    "src/",
    "assets/",
    "build.rs"
]

[build-dependencies]
glob = "0.3"
```

### 3. Leptos CSS Bundling Approaches

#### Leptos + Trunk
- Use `trunk` for bundling with asset handling
- CSS imported via `#[component]` or global imports
- Static assets served from `public/` directory

#### Leptos + Vite/Rspack
- Use `leptos-vite` or `leptos-rspack` integrations
- CSS bundled with JavaScript
- Hot module replacement for development

### 4. Yew CSS Bundling Approaches

#### Yew + Trunk (Most Common)
- Simple setup with `trunk.toml`
- CSS in `public/` directory served directly
- Asset fingerprinting for production

#### Yew + wasm-pack
- Compile to WASM with `wasm-pack build --target web`
- CSS bundled separately or inlined
- Use `wasm-bindgen` for JavaScript interop

### 5. Tools for Professional CSS Management

#### Asset Bundlers
1. **Trunk** - Simple, opinionated for wasm/web
2. **Vite** - Fast development, modern tooling
3. **Wasm-pack** - Low-level WASM compilation
4. **Rspack** - Rust-based Webpack alternative

#### CSS Processors
1. **Dart Sass** - Standard Sass implementation
2. **PostCSS** - CSS transformations with plugins
3. **Tailwind CSS** - Utility-first framework
4. **CSS Modules** - Scoped CSS with `css-module` crate

### 6. Avoiding CSS Duplication

#### Pattern 1: Shared Asset Directory
```
assets/
├── styles/
│   ├── base.css
│   ├── components.css
│   └── utilities.css
```

#### Pattern 2: Build Script Asset Copying
```rust
// build.rs
use std::fs;
use std::path::Path;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dest = Path::new(&manifest_dir).join("target").join("css");
    fs::create_dir_all(&dest).unwrap();
    fs::copy(
        Path::new(&manifest_dir).join("assets").join("styles.css"),
        dest.join("styles.css")
    ).unwrap();
}
```

#### Pattern 3: Symbolic Links
```bash
# scripts/link-css.sh
ln -sf ../assets/styles.css examples/*/public/styles.css
```

### 7. Recommended Professional Setup

#### For New Projects: Leptos + Trunk
```toml
# Cargo.toml
[dependencies]
leptos = { version = "0.6", features = ["csr"] }
leptos-use = "0.12"

[build-dependencies]
glob = "0.3"
```

```toml
# trunk.toml
[build]
target = "index.html"
out-dir = "dist"
public-url = "/"

[watch]
ignore = ["target", "node_modules"]
```

#### CSS Organization
```
assets/
├── styles/
│   ├── main.css           # Entry point
│   ├── base/              # Base styles
│   ├── components/        # Component styles
│   └── utilities/         # Utility classes
└── images/
```

### 8. Implementation Recommendations

#### For This Project:
1. Create shared CSS in `assets/styles/` directory
2. Use build script to copy to example `public/` directories
3. Implement CSS modules for component-scoped styles
4. Use CSS variables for theming consistency

#### Toolchain:
- **Build**: `trunk serve` for development, `trunk build` for production
- **Lint**: `stylelint` with Rust-specific configuration
- **Format**: `prettier` for CSS formatting
- **Testing**: `vitest` with CSS snapshot testing

### 9. Alternative Approaches

#### WASM-Only CSS (Advanced)
- Compile CSS to WASM with `css-module` crate
- Runtime CSS injection
- Maximum type safety but complex tooling

#### CSS-in-Rust (Experimental)
- `dioxus` supports CSS-in-Rust syntax
- Compile-time CSS generation
- Limited ecosystem support

### 10. Resources

#### Key Crates
- `trunk` - Web bundler
- `wasm-pack` - WASM compilation
- `leptos` - React-like framework
- `yew` - Elm-inspired framework
- `css-module` - CSS modules support

#### Documentation
- Leptos Book: https://book.leptos.dev/
- Yew Book: https://yew.rs/docs/
- Trunk Docs: https://trunkrs.dev/
- wasm-pack Book: https://rustwasm.github.io/wasm-pack/