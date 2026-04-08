use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

mod pages;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");

    // Inject CSS from telegram-ui
    let styles = telegram_ui::get_styles();
    let style_elem = document
        .create_element("style")?
        .dyn_into::<HtmlElement>()?;
    style_elem.set_text_content(Some(styles));
    document.head().unwrap().append_child(&style_elem)?;

    // Create app container
    let container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    container.set_class_name("tgui-app-root tgui-platform-base tg-theme-auto");
    body.append_child(&container)?;

    // Build navigation
    let nav = document.create_element("nav")?.dyn_into::<HtmlElement>()?;
    nav.set_class_name("app-nav");
    nav.set_inner_html(
        "<a href='javascript:void(0)' data-page='home' class='nav-item active'>🏠 Home</a>\
         <a href='javascript:void(0)' data-page='buttons' class='nav-item'>🔘 Buttons</a>\
         <a href='javascript:void(0)' data-page='forms' class='nav-item'>📝 Forms</a>\
         <a href='javascript:void(0)' data-page='feedback' class='nav-item'>⚡ Feedback</a>\
         <a href='javascript:void(0)' data-page='layout' class='nav-item'>📦 Layout</a>\
         <a href='javascript:void(0)' data-page='typography' class='nav-item'>📝 Typography</a>\
         <a href='javascript:void(0)' data-page='overlays' class='nav-item'>🪟 Overlays</a>\
         <a href='javascript:void(0)' data-page='service' class='nav-item'>⚙️ Service</a>",
    );
    container.append_child(&nav)?;

    // Create content div
    let content = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    content.set_id("page-content");
    container.append_child(&content)?;

    // Show home page
    show_page(&document, &content, "home")?;

    // Simple navigation handler using global click
    let document_clone = document.clone();
    let content_clone = content.clone();
    let nav_clone = nav.clone();

    let closure = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
        let target = _e.target().unwrap().dyn_into::<HtmlElement>();
        if let Some(elem) = target {
            if elem.class_list().contains("nav-item") {
                _e.prevent_default();
                let page = elem.get_attribute("data-page").unwrap_or_default();

                // Update active nav
                if let Ok(items) = nav_clone.query_selector_all(".nav-item") {
                    for i in 0..items.length() {
                        if let Some(item) = items.get(i) {
                            let el: HtmlElement = item.dyn_into().ok().unwrap();
                            el.class_list().remove("active");
                        }
                    }
                }
                elem.class_list().add("active");

                // Show page
                show_page(&document_clone, &content_clone, &page).ok();
            }
        }
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    container.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn show_page(
    document: &web_sys::Document,
    content: &HtmlElement,
    page: &str,
) -> Result<(), JsValue> {
    content.set_inner_html("");
    let page_elem = match page {
        "home" => pages::Home::new(document)?.element,
        "buttons" => pages::Buttons::new(document)?.element,
        "forms" => pages::Forms::new(document)?.element,
        "feedback" => pages::Feedback::new(document)?.element,
        "layout" => pages::Layout::new(document)?.element,
        "typography" => pages::Typography::new(document)?.element,
        "overlays" => pages::Overlays::new(document)?.element,
        "service" => pages::Service::new(document)?.element,
        _ => pages::Home::new(document)?.element,
    };
    content.append_child(&page_elem)?;
    Ok(())
}

pub trait Page {
    fn element(&self) -> &HtmlElement;
}
