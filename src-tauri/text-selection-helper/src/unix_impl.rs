use std::any::type_name;

use accessibility::{AXAttribute, AXUIElement};
use accessibility_sys::{
    kAXFocusedApplicationAttribute, kAXFocusedUIElementAttribute, kAXSelectedTextAttribute,
    kAXTrustedCheckOptionPrompt, AXIsProcessTrustedWithOptions,
};
use core_foundation::{
    base::TCFType, boolean::CFBoolean, dictionary::CFDictionary, string::CFString, ConcreteCFType,
};

use crate::types::Result;

pub fn get_selected_text() -> Result<String> {
    let sys_element = AXUIElement::system_wide();

    let focused_app: AXUIElement = get_element_attr(&sys_element, kAXFocusedApplicationAttribute)?;

    let focused_element: AXUIElement =
        get_element_attr(&focused_app, kAXFocusedUIElementAttribute)?;

    let selected_text = get_element_attr::<CFString>(&focused_element, kAXSelectedTextAttribute)?;

    return Ok(selected_text.to_string());
}

fn get_element_attr<T: ConcreteCFType>(element: &AXUIElement, attr: &str) -> Result<T> {
    let attr_value = element.attribute(&AXAttribute::new(&CFString::new(&attr)));

    match attr_value {
        Ok(attr_value) => {
            let raw_value = attr_value.clone();

            if let Some(value) = attr_value.downcast::<T>() {
                return Ok(value);
            };

            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Convert {:?} to type {:?} failed",
                    raw_value,
                    type_name::<T>()
                ),
            )));
        }
        Err(err) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("get attr {} failed: {:?}", attr, err),
            )));
        }
    }
}

pub fn request_accessibility_access() -> bool {
    unsafe {
        // request accessibility permission
        let prompt_conf_key = CFString::wrap_under_create_rule(kAXTrustedCheckOptionPrompt);
        let conf_dict =
            CFDictionary::from_CFType_pairs(&[(prompt_conf_key, CFBoolean::from(true))]);
        let result = AXIsProcessTrustedWithOptions(conf_dict.as_concrete_TypeRef());

        log::info!("prompt: {:?}", result);

        return result;
    }
}
