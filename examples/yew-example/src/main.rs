// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 Telegram UI contributors

use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <h1>{"Telegram UI Components"}</h1>
            <div class="container">
                {view_component_button()}
                {view_component_text()}
                {view_component_input()}
                {view_component_checkbox()}
                {view_component_radio()}
                {view_component_select()}
                {view_component_slider()}
                {view_component_switch()}
                {view_component_avatar()}
                {view_component_button_group()}
                {view_component_card()}
                {view_component_list()}
                {view_component_modal()}
                {view_component_popup()}
                {view_component_progress()}
                {view_component_search()}
                {view_component_tabs()}
                {view_component_toolstrip()}
                {view_component_actions_sheet()}
                {view_component_alert()}
                {view_component_snackbar()}
                {view_component_loading()}
            </div>
        </div>
    }
}

#[function_component(view_component_button)]
fn view_component_button() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Button"}</h3>
            <button class="tg-button">{"Button"}</button>
            <button class="tg-button tg-button-primary">{"Primary"}</button>
            <button class="tg-button tg-button-danger">{"Danger"}</button>
            <button class="tg-button tg-button-outline">{"Outline"}</button>
            <button class="tg-button tg-button-link">{"Link"}</button>
        </div>
    }
}

#[function_component(view_component_text)]
fn view_component_text() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Text"}</h3>
            <p class="tg-text tg-text-body1">{"Body1 - The quick brown fox jumps over the lazy dog"}</p>
            <p class="tg-text tg-text-body2">{"Body2 - The quick brown fox jumps over the lazy dog"}</p>
            <p class="tg-text tg-text-subtitle1">{"Subtitle1 - The quick brown fox jumps over the lazy dog"}</p>
            <p class="tg-text tg-text-subtitle2">{"Subtitle2 - The quick brown fox jumps over the lazy dog"}</p>
            <p class="tg-text tg-text-caption">{"Caption - The quick brown fox jumps over the lazy dog"}</p>
            <p class="tg-text tg-text-overline">{"Overline - The quick brown fox jumps over the lazy dog"}</p>
        </div>
    }
}

#[function_component(view_component_input)]
fn view_component_input() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Input"}</h3>
            <div class="tg-input-wrapper">
                <input r#type="text" class="tg-input" placeholder="Enter text..." />
            </div>
            <div class="tg-input-wrapper tg-input-wrapper-filled">
                <input r#type="text" class="tg-input" value="Pre-filled value" />
                <span class="tg-input-label">{"Label"}</span>
            </div>
        </div>
    }
}

#[function_component(view_component_checkbox)]
fn view_component_checkbox() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Checkbox"}</h3>
            <label class="tg-checkbox">
                <input r#type="checkbox" />
                <span class="tg-checkbox-icon"></span>
                <span class="tg-checkbox-label">{"Checkbox"}</span>
            </label>
            <label class="tg-checkbox">
                <input r#type="checkbox" checked=true />
                <span class="tg-checkbox-icon"></span>
                <span class="tg-checkbox-label">{"Checked"}</span>
            </label>
            <label class="tg-checkbox">
                <input r#type="checkbox" disabled=true />
                <span class="tg-checkbox-icon"></span>
                <span class="tg-checkbox-label">{"Disabled"}</span>
            </label>
        </div>
    }
}

#[function_component(view_component_radio)]
fn view_component_radio() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Radio"}</h3>
            <label class="tg-radio">
                <input r#type="radio" name="radio-group" />
                <span class="tg-radio-icon"></span>
                <span class="tg-radio-label">{"Option 1"}</span>
            </label>
            <label class="tg-radio">
                <input r#type="radio" name="radio-group" checked=true />
                <span class="tg-radio-icon"></span>
                <span class="tg-radio-label">{"Option 2"}</span>
            </label>
            <label class="tg-radio">
                <input r#type="radio" name="radio-group" disabled=true />
                <span class="tg-radio-icon"></span>
                <span class="tg-radio-label">{"Disabled"}</span>
            </label>
        </div>
    }
}

#[function_component(view_component_select)]
fn view_component_select() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Select"}</h3>
            <div class="tg-select-wrapper">
                <select class="tg-select">
                    <option>{"Option 1"}</option>
                    <option>{"Option 2"}</option>
                    <option>{"Option 3"}</option>
                </select>
                <span class="tg-select-arrow"></span>
            </div>
        </div>
    }
}

#[function_component(view_component_slider)]
fn view_component_slider() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Slider"}</h3>
            <input r#type="range" class="tg-slider" min="0" max="100" value="50" />
            <input r#type="range" class="tg-slider tg-slider-disabled" min="0" max="100" value="75" disabled=true />
        </div>
    }
}

#[function_component(view_component_switch)]
fn view_component_switch() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Switch"}</h3>
            <label class="tg-switch">
                <input r#type="checkbox" />
                <span class="tg-switch-track"></span>
                <span class="tg-switch-thumb"></span>
            </label>
            <label class="tg-switch">
                <input r#type="checkbox" checked=true />
                <span class="tg-switch-track"></span>
                <span class="tg-switch-thumb"></span>
            </label>
            <label class="tg-switch">
                <input r#type="checkbox" disabled=true />
                <span class="tg-switch-track"></span>
                <span class="tg-switch-thumb"></span>
            </label>
        </div>
    }
}

#[function_component(view_component_avatar)]
fn view_component_avatar() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Avatar"}</h3>
            <div class="tg-avatar tg-avatar-circle tg-avatar-xl">
                <img src="https://via.placeholder.com/100" alt="Avatar" />
            </div>
            <div class="tg-avatar tg-avatar-circle tg-avatar-lg">
                <img src="https://via.placeholder.com/80" alt="Avatar" />
            </div>
            <div class="tg-avatar tg-avatar-circle">
                <img src="https://via.placeholder.com/64" alt="Avatar" />
            </div>
            <div class="tg-avatar tg-avatar-circle tg-avatar-sm">
                <img src="https://via.placeholder.com/48" alt="Avatar" />
            </div>
            <div class="tg-avatar tg-avatar-circle tg-avatar-xs">
                <img src="https://via.placeholder.com/32" alt="Avatar" />
            </div>
        </div>
    }
}

#[function_component(view_component_button_group)]
fn view_component_button_group() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Button Group"}</h3>
            <div class="tg-button-group">
                <button class="tg-button">{"One"}</button>
                <button class="tg-button">{"Two"}</button>
                <button class="tg-button">{"Three"}</button>
            </div>
            <div class="tg-button-group tg-button-group-vertical">
                <button class="tg-button">{"One"}</button>
                <button class="tg-button">{"Two"}</button>
                <button class="tg-button">{"Three"}</button>
            </div>
        </div>
    }
}

#[function_component(view_component_card)]
fn view_component_card() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Card"}</h3>
            <div class="tg-card">
                <div class="tg-card-header">
                    <div class="tg-card-title">{"Card Title"}</div>
                    <div class="tg-card-subtitle">{"Card Subtitle"}</div>
                </div>
                <div class="tg-card-content">
                    {"Card content goes here. This is where you would place your main content."}
                </div>
                <div class="tg-card-footer">
                    <button class="tg-button tg-button-link">{"Action"}</button>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_list)]
fn view_component_list() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"List"}</h3>
            <div class="tg-list">
                <div class="tg-list-item">
                    <div class="tg-list-item-content">
                        <div class="tg-list-item-title">{"Item 1"}</div>
                        <div class="tg-list-item-description">{"Description 1"}</div>
                    </div>
                    <div class="tg-list-item-after">
                        <span class="tg-counter">{"1"}</span>
                    </div>
                </div>
                <div class="tg-list-item">
                    <div class="tg-list-item-content">
                        <div class="tg-list-item-title">{"Item 2"}</div>
                        <div class="tg-list-item-description">{"Description 2"}</div>
                    </div>
                </div>
                <div class="tg-list-item tg-list-item-selected">
                    <div class="tg-list-item-content">
                        <div class="tg-list-item-title">{"Item 3"}</div>
                        <div class="tg-list-item-description">{"Description 3"}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_modal)]
fn view_component_modal() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Modal"}</h3>
            <div class="tg-modal tg-modal-active">
                <div class="tg-modal-overlay"></div>
                <div class="tg-modal-content">
                    <div class="tg-modal-header">
                        <div class="tg-modal-title">{"Modal Title"}</div>
                        <button class="tg-button tg-button-clear tg-modal-close"></button>
                    </div>
                    <div class="tg-modal-body">
                        {"Modal content goes here."}
                    </div>
                    <div class="tg-modal-footer">
                        <button class="tg-button tg-button-primary">{"OK"}</button>
                        <button class="tg-button tg-button-cancel">{"Cancel"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_popup)]
fn view_component_popup() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Popup"}</h3>
            <div class="tg-popup tg-popup-active">
                <div class="tg-popup-overlay"></div>
                <div class="tg-popup-content">
                    <div class="tg-list">
                        <div class="tg-list-item">
                            <span class="tg-list-item-title">{"Item 1"}</span>
                        </div>
                        <div class="tg-list-item">
                            <span class="tg-list-item-title">{"Item 2"}</span>
                        </div>
                        <div class="tg-list-item">
                            <span class="tg-list-item-title">{"Item 3"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_progress)]
fn view_component_progress() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Progress"}</h3>
            <div class="tg-progress">
                <div class="tg-progress-track">
                    <div class="tg-progress-indicator" style="width: 50%;"></div>
                </div>
            </div>
            <div class="tg-progress tg-progress-indeterminate">
                <div class="tg-progress-track">
                    <div class="tg-progress-indicator"></div>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_search)]
fn view_component_search() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Search"}</h3>
            <div class="tg-search">
                <div class="tg-search-input-wrapper">
                    <input r#type="text" class="tg-search-input" placeholder="Search..." />
                    <span class="tg-search-icon"></span>
                </div>
                <button class="tg-search-cancel"></button>
            </div>
        </div>
    }
}

#[function_component(view_component_tabs)]
fn view_component_tabs() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Tabs"}</h3>
            <div class="tg-tabs">
                <div class="tg-tabs-item tg-tabs-item-active">
                    <span class="tg-tabs-item-text">{"Tab 1"}</span>
                </div>
                <div class="tg-tabs-item">
                    <span class="tg-tabs-item-text">{"Tab 2"}</span>
                </div>
                <div class="tg-tabs-item">
                    <span class="tg-tabs-item-text">{"Tab 3"}</span>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_toolstrip)]
fn view_component_toolstrip() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Toolstrip"}</h3>
            <div class="tg-toolstrip">
                <button class="tg-button tg-button-small">{"Btn 1"}</button>
                <button class="tg-button tg-button-small">{"Btn 2"}</button>
                <button class="tg-button tg-button-small">{"Btn 3"}</button>
                <button class="tg-button tg-button-small">{"Btn 4"}</button>
            </div>
        </div>
    }
}

#[function_component(view_component_actions_sheet)]
fn view_component_actions_sheet() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Actions Sheet"}</h3>
            <div class="tg-actions-sheet tg-actions-sheet-active">
                <div class="tg-actions-sheet-overlay"></div>
                <div class="tg-actions-sheet-content">
                    <div class="tg-list">
                        <div class="tg-list-item tg-list-item-clickable">
                            <span class="tg-list-item-title">{"Action 1"}</span>
                        </div>
                        <div class="tg-list-item tg-list-item-clickable">
                            <span class="tg-list-item-title">{"Action 2"}</span>
                        </div>
                        <div class="tg-list-item tg-list-item-clickable">
                            <span class="tg-list-item-title">{"Action 3"}</span>
                        </div>
                        <div class="tg-list-item tg-actions-sheet-cancel">
                            <span class="tg-list-item-title">{"Cancel"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(view_component_alert)]
fn view_component_alert() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Alert"}</h3>
            <div class="tg-alert tg-alert-success">
                <span class="tg-alert-title">{"Success"}</span>
                <span class="tg-alert-message">{"Operation completed successfully."}</span>
            </div>
            <div class="tg-alert tg-alert-error">
                <span class="tg-alert-title">{"Error"}</span>
                <span class="tg-alert-message">{"An error occurred."}</span>
            </div>
            <div class="tg-alert tg-alert-warning">
                <span class="tg-alert-title">{"Warning"}</span>
                <span class="tg-alert-message">{"Please review your input."}</span>
            </div>
            <div class="tg-alert tg-alert-info">
                <span class="tg-alert-title">{"Info"}</span>
                <span class="tg-alert-message">{"Here is some information."}</span>
            </div>
        </div>
    }
}

#[function_component(view_component_snackbar)]
fn view_component_snackbar() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Snackbar"}</h3>
            <div class="tg-snackbar tg-snackbar-active">
                <div class="tg-snackbar-content">{"Action completed"}</div>
                <button class="tg-snackbar-action">{"Undo"}</button>
            </div>
        </div>
    }
}

#[function_component(view_component_loading)]
fn view_component_loading() -> Html {
    html! {
        <div class="component-demo">
            <h3>{"Loading"}</h3>
            <div class="tg-loading tg-loading-small"></div>
            <div class="tg-loading"></div>
            <div class="tg-loading tg-loading-large"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
