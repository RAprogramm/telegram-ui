<!-- SPDX-License-Identifier: MIT -->
<!-- SPDX-FileCopyrightText: 2026 Telegram UI contributors -->

# Architecture: telegram-webapp-sdk vs telegram-ui-core

## Current State Analysis

### Dependencies

```
┌─────────────────────────────────────────────────────────────────┐
│                    telegram-webapp-sdk (v0.5.0)                  │
│  Location: /home/ra/Projects/telegram-webapp-sdk               │
│  Description: Telegram WebApp SDK for Rust                      │
│  Features:                                                      │
│    - Comprehensive Telegram WebApp API coverage                 │
│    - No framework dependencies by default                       │
│    - Optional leptos/yew support via features                   │
│    - Mock support for testing                                   │
│  Dependencies:                                                  │
│    - wasm-bindgen, js-sys, web-sys (core WASM)                  │
│    - serde, serde_json (data serialization)                     │
│    - Optional: leptos, yew (framework support)                  │
└─────────────────────────────────────────────────────────────────┘
                              ↑
                              │ (optional)
                              │
┌─────────────────────────────────────────────────────────────────┐
│                    telegram-ui-core (v0.1.0)                     │
│  Location: /home/ra/Projects/telegram-ui/crates/telegram-ui-core│
│  Description: Telegram UI components for Rust                   │
│  Features:                                                      │
│    - UI components (Button, Spinner, etc.)                      │
│    - Depends on telegram-webapp-sdk                             │
│    - No framework dependencies by default                       │
│    - Optional leptos/yew support via features                   │
│  Dependencies:                                                  │
│    - telegram-webapp-sdk (required, path dep)                   │
│    - Optional: leptos, yew (framework support)                  │
└─────────────────────────────────────────────────────────────────┘
                              ↑
                              │ (examples use both)
                              │
┌─────────────────────────────────────────────────────────────────┐
│                    Examples (leptos/yew)                        │
│  Location: /home/ra/Projects/telegram-ui/examples/              │
│  Features:                                                      │
│    - Use telegram-ui-core for components                        │
│    - Use telegram-webapp-sdk directly when needed               │
│    - Framework-specific implementations                         │
└─────────────────────────────────────────────────────────────────┘
```

## Architecture Decision: SDK-First Approach

### Why SDK-First?

1. **Separation of Concerns**
   - **SDK**: Low-level API wrapper, handles JS interop
   - **UI**: High-level components, handles rendering and styling

2. **Dependency Flow**
   ```
   SDK (foundation) → UI (built on SDK) → Examples (use both)
   ```

3. **Flexibility**
   - SDK can be used without UI components
   - UI can be used without framework-specific features
   - Examples can use SDK directly for advanced features

4. **Reusability**
   - SDK is framework-agnostic
   - UI can be used with any framework (or vanilla JS)
   - Examples demonstrate framework-specific patterns

### Current Architecture

```
telegram-webapp-sdk (SDK)
├── No framework dependencies (core)
├── Optional: leptos (via feature)
├── Optional: yew (via feature)
└── Optional: mock (via feature)

telegram-ui-core (UI Library)
├── Depends on telegram-webapp-sdk (REQUIRED)
├── No framework dependencies (core)
├── Optional: leptos (via feature)
├── Optional: yew (via feature)
└── Components: Button, Spinner, Card, Cell, etc.

Examples
├── leptos-example
│   ├── Uses telegram-ui-core for components
│   ├── Uses telegram-webapp-sdk directly for advanced features
│   └── Framework-specific implementation
└── yew-example
    ├── Uses telegram-ui-core for components
    ├── Uses telegram-webapp-sdk directly for advanced features
    └── Framework-specific implementation
```

## Component Dependencies

### SDK Provides
- `TelegramWebApp` struct with instance management
- API modules: `api/`, `webapp/`, `core/`
- Framework adapters: `leptos/`, `yew/`
- Mock environment: `mock/`

### UI Provides
- Components: Button, Spinner, Card, Cell, List, etc.
- CSS styles with Telegram design system
- Helper types: ButtonSize, ButtonMode, etc.
- Platform detection

## Usage Patterns

### Pattern 1: Basic UI (No SDK needed)
```rust
use telegram_ui_core::Button;

let button = Button::new()
    .size(ButtonSize::M)
    .mode(ButtonMode::Filled)
    .children("Click me");
```

### Pattern 2: UI with SDK Features
```rust
use telegram_ui_core::Button;
use telegram_webapp_sdk::TelegramWebApp;

// Use UI component
let button = Button::new().children("Submit");

// Use SDK directly for advanced features
if let Some(webapp) = TelegramWebApp::instance() {
    let _ = webapp.expand();
}
```

### Pattern 3: Full Framework Integration
```rust
use telegram_ui_core::{Button, ButtonSize, ButtonMode};
use telegram_webapp_sdk::TelegramWebApp;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Button
            size=ButtonSize::M
            mode=ButtonMode::Filled
            children="Submit"
        />
    }
}
```

## Dependency Resolution

### From UI Library Perspective
```toml
[dependencies]
telegram-webapp-sdk = { path = "/home/ra/Projects/telegram-webapp-sdk", features = ["macros"] }
# Required for WebApp SDK functionality

leptos = { workspace = true, optional = true }
yew = { workspace = true, optional = true }
# Optional for framework-specific features
```

### From Example Perspective
```toml
[dependencies]
telegram-webapp-sdk = { path = "/home/ra/Projects/telegram-webapp-sdk", features = ["macros"] }
telegram-ui-core = { version = "0.1.0", path = "../../crates/telegram-ui-core" }
leptos = { workspace = true, features = ["csr"] }
# Use both SDK and UI, plus framework
```

## Benefits of This Architecture

1. **No Circular Dependencies**
   - SDK has no dependency on UI
   - UI depends on SDK (one-way)
   - Examples depend on both (flat dependency graph)

2. **Clear Separation**
   - SDK = API wrapper
   - UI = Component library
   - Examples = Framework-specific usage

3. **Flexibility**
   - Can use SDK without UI
   - Can use UI without framework
   - Can use both SDK and UI together

4. **Maintainability**
   - Each component has clear responsibility
   - Easier to test in isolation
   - Clear upgrade paths

## Migration Notes

### From Old Architecture (UI depends on SDK)
- **Old**: `telegram-ui-core` had optional `telegram-webapp-sdk`
- **New**: `telegram-ui-core` requires `telegram-webapp-sdk` as path dependency

### Breaking Changes
1. UI library now requires local SDK path
2. Examples need both SDK and UI dependencies
3. Framework features moved to examples

### Migration Steps
1. Add SDK as path dependency in UI Cargo.toml
2. Update examples to use both SDK and UI
3. Remove framework features from UI library
4. Add framework features to examples

## Future Enhancements

### Potential Improvements
1. **Framework-specific UI adapters**
   - `telegram-ui-leptos` crate
   - `telegram-ui-yew` crate
   - Reusable UI components with framework adapters

2. **Advanced UI components**
   - Card with sub-components (Cell, Chip)
   - List with virtualization
   - Modal with backdrop

3. **Theme support**
   - Light/dark mode
   - Custom theme configuration
   - CSS variable overrides

4. **Animations**
   - Entrance/exit animations
   - Loading states
   - Transitions

## Summary

The SDK-first architecture provides:
- ✅ Clear separation of concerns
- ✅ No circular dependencies
- ✅ Flexible usage patterns
- ✅ Maintainable codebase
- ✅ Scalable component library

This is the recommended architecture for professional Rust Telegram Mini App development.
