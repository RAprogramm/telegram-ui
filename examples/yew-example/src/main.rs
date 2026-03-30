//! Yew example application using Telegram UI
//!
//! This application demonstrates how to use Telegram UI components with Yew.

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Telegram UI Yew Example" }</h1>

            <div class="button-group">
                <h2>{ "Buttons" }</h2>
                <button class="telegram-ui-button telegram-ui-button--m telegram-ui-button--filled">
                    { "Filled Button" }
                </button>

                <button class="telegram-ui-button telegram-ui-button--m telegram-ui-button--outline">
                    { "Outline Button" }
                </button>

                <button class="telegram-ui-button telegram-ui-button--m telegram-ui-button--plain">
                    { "Plain Button" }
                </button>

                <button class="telegram-ui-button telegram-ui-button--s telegram-ui-button--gray">
                    { "Small Gray Button" }
                </button>

                <button class="telegram-ui-button telegram-ui-button--l telegram-ui-button--white">
                    { "Large White Button" }
                </button>
            </div>

            <div class="spinner-group">
                <h2>{ "Spinners" }</h2>
                <div class="telegram-ui-spinner telegram-ui-spinner--s"></div>
                <div class="telegram-ui-spinner telegram-ui-spinner--m"></div>
                <div class="telegram-ui-spinner telegram-ui-spinner--l"></div>
            </div>

            <div class="platform-demo">
                <h2>{ "Platform Demo" }</h2>
                <p>{ "This demonstrates the Telegram UI components that adapt to different platforms." }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
