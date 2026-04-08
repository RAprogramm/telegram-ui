use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

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
    nav.set_id("main-nav");
    nav.set_inner_html(
        "<a href='#' class='nav-item active' data-page='home'>🏠 Home</a>\
         <a href='#' class='nav-item' data-page='buttons'>🔘 Buttons</a>\
         <a href='#' class='nav-item' data-page='forms'>📝 Forms</a>\
         <a href='#' class='nav-item' data-page='feedback'>⚡ Feedback</a>\
         <a href='#' class='nav-item' data-page='layout'>📦 Layout</a>\
         <a href='#' class='nav-item' data-page='typography'>📝 Typography</a>\
         <a href='#' class='nav-item' data-page='overlays'>🪟 Overlays</a>\
         <a href='#' class='nav-item' data-page='service'>⚙️ Service</a>",
    );
    container.append_child(&nav)?;

    // Create content div
    let content = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    content.set_id("page-content");
    container.append_child(&content)?;

    // Show home page
    show_page(&document, &content, "home")?;

    // Navigation click handler
    let nav_clone = nav.clone();
    let doc_clone = document.clone();
    let content_clone = content.clone();

    let active_arr = Array::new();
    active_arr.push(&"active".into());

    let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        e.prevent_default();
        if let Some(target) = e.target() {
            if let Ok(elem) = target.dyn_into::<HtmlElement>() {
                if elem.class_list().contains("nav-item") {
                    // Remove active from all
                    if let Ok(items) = nav_clone.query_selector_all(".nav-item") {
                        for idx in 0..items.length() {
                            if let Some(node) = items.get(idx) {
                                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                                    el.class_list().remove(&active_arr);
                                }
                            }
                        }
                    }
                    // Add active to clicked
                    elem.class_list().add(&active_arr);

                    // Show page
                    if let Some(page) = elem.get_attribute("data-page") {
                        show_page(&doc_clone, &content_clone, &page).ok();
                    }
                }
            }
        }
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    nav.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
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
