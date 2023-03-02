use web_sys::{Document, Window};

pub fn get_safe_area() -> Result<SafeArea, &'static str> {
    let window = web_sys::window().ok_or("Could not get window")?;
    let document = window.document().ok_or("Could not get document")?;
    let de = document
        .document_element()
        .ok_or("Could not get document element")?;
    let style = Window::get_computed_style(&window, &de)
        .map_err(|_| "error getting style")?
        .ok_or("No Style")?;

    let top = style
        .get_property_value("--safe-area-inset-top")
        .map_err(|_| "Could not get property value")?;
    let bottom = style
        .get_property_value("--safe-area-inset-bottom")
        .map_err(|_| "Could not get property value")?;
    let left = style
        .get_property_value("--safe-area-inset-left")
        .map_err(|_| "Could not get property value")?;
    let right = style
        .get_property_value("--safe-area-inset-right")
        .map_err(|_| "Could not get property value")?;

    Ok(SafeArea {
        top,
        bottom,
        left,
        right,
    })
}

#[derive(Debug)]
pub struct SafeArea {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
}

pub fn get_safe_area_response() -> String {
    match get_safe_area() {
        Ok(s) => {
            log::info!("{s:?}");
            format!("{s:?}")
        }
        Err(e) => {
            log::error!("{e}");
            e.to_string()
        }
    }
}
