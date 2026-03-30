# CSS Refactoring Plan

## Current State

- **Main crate**: `styles.css` (514 lines) - full component styles
- **Examples**: Each has a copy of `styles.css` (142 lines) - partial styles
- **Problem**: CSS duplication, maintenance overhead

## Solution Architecture

### Approach 1: Shared CSS via CDN/External URL (Recommended for WebAssembly)

**Advantages**:
- Single source of truth
- Browser caching benefits
- Easy to version
- Works with existing trunk bundling

**Implementation**:

1. **Upload CSS to CDN** (GitHub Releases, jsDelivr, or similar)
2. **Update example HTML** to reference the shared CSS:

```html
<!-- leptos-example/index.html -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/username/telegram-ui@latest/styles.css">
```

3. **Keep minimal CSS in examples** only for demo-specific styling

### Approach 2: Build-time Asset Copying

**Advantages**:
- No external dependencies
- Works offline
- Full control over asset handling

**Implementation**:

1. **Create a build script** (`build.rs`) that copies CSS to examples
2. **Update examples** to reference the copied CSS
3. **Add CI check** to ensure examples CSS matches main CSS

### Approach 3: TypeScript/CSS Module Import (For modern bundlers)

**Advantages**:
- Type-safe CSS imports
- Tree-shaking support
- Scoped styles

**Implementation**:

1. **Convert to CSS modules** or use CSS-in-Rust approach
2. **Import CSS** in example entry points
3. **Bundle CSS** with examples

## Recommended Implementation: Approach 1

### Phase 1: Centralize CSS

1. Keep main `styles.css` as the single source
2. Remove CSS duplication from examples
3. Add CI check to validate CSS consistency

### Phase 2: Distribution

1. Publish CSS to CDN (GitHub Releases + jsDelivr)
2. Update examples to use CDN URL
3. Document CSS versioning strategy

### Phase 3: Automation

1. Add pre-release script to update CDN
2. Add CI check for CSS version sync
3. Create CSS changelog automation

## Migration Steps

### Step 1: Update Example HTML Files

Replace local CSS references with CDN reference:

```html
<!-- Before -->
<link data-trunk rel="css" href="styles.css"/>

<!-- After -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/telegram-rs/ui@v0.1.0/styles.css">
```

### Step 2: Add CSS Validation Script

Create script to ensure examples use the latest CSS:

```bash
#!/bin/bash
# scripts/validate-css-sync.sh

MAIN_CSS_HASH=$(sha256sum styles.css | cut -d' ' -f1)
EXAMPLE_CSS_HASH=$(sha256sum examples/leptos-example/styles.css | cut -d' ' -f1)

if [ "$MAIN_CSS_HASH" != "$EXAMPLE_CSS_HASH" ]; then
    echo "CSS files are out of sync!"
    exit 1
fi
```

### Step 3: Add Documentation

Update README with CSS architecture explanation:
- Where CSS is hosted
- How to update CSS
- Versioning strategy

## Benefits

1. **Single Source of Truth**: One file to maintain
2. **Easier Updates**: Changes propagate automatically
3. **Reduced Build Times**: No CSS duplication
4. **Better Caching**: Browser can cache shared CSS
5. **Smaller Deltas**: Only CSS changes, not duplicated code

## Testing Strategy

1. **Unit Tests**: Verify CSS loads correctly
2. **Integration Tests**: Check component rendering with shared CSS
3. **Visual Regression**: Snapshot tests for UI appearance
4. **CI Pipeline**: Automated CSS validation

## Rollout Plan

1. ✅ Audit current CSS duplication
2. 🔄 Choose CDN provider
3. 🔄 Upload CSS to CDN
4. 🔄 Update examples to use CDN
5. 🔄 Add CSS validation to CI
6. 🔄 Document CSS architecture
7. 🔄 Monitor for issues

## Alternative: Embedded CSS in Rust

For a more Rust-native approach, consider embedding CSS in Rust binary:

```rust
pub fn get_styles() -> &'static str {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/styles.css"))
}
```

Then provide helper functions to:
- Serve CSS from a web server
- Inject CSS into the DOM
- Provide CSS as a data URL

This approach works well for:
- Server-side rendering
- Desktop applications
- Environments where CDN access is restricted
