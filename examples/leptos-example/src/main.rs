//! Leptos example application using Telegram UI
//!
//! This application demonstrates how to use Telegram UI components with Leptos.

use leptos::prelude::*;
use telegram_ui_core::{Button, Spinner};

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Telegram UI - Leptos Example"</h1>
            <p>"Framework: Leptos 0.8.0"</p>
            
            <div class="button-group">
                <h2>"Buttons"</h2>
                
                <button class="telegram-ui-button telegram-ui-button--m telegram-ui-button--filled">
                    "Filled Button"
                </button>
                
                <button class="telegram-ui-button telegram-ui-button--m telegram-ui-button--outline">
                    "Outline Button"
                </button>
                
                <button class="telegram-ui-button telegram-ui-button--m telegram-ui-button--plain">
                    "Plain Button"
                </button>
                
                <button class="telegram-ui-button telegram-ui-button--s telegram-ui-button--gray">
                    "Small Gray Button"
                </button>
                
                <button class="telegram-ui-button telegram-ui-button--l telegram-ui-button--white">
                    "Large White Button"
                </button>
            </div>
            
            <div class="spinner-group">
                <h2>"Spinners"</h2>
                <div class="telegram-ui-spinner telegram-ui-spinner--s"></div>
                <div class="telegram-ui-spinner telegram-ui-spinner--m"></div>
                <div class="telegram-ui-spinner telegram-ui-spinner--l"></div>
            </div>
            
            <div class="platform-demo">
                <h2>"Platform Demo"</h2>
                <p>"This demonstrates the Telegram UI components."</p>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    
    leptos::mount::mount_to_body(move || view! {
        <App />
    });
}
