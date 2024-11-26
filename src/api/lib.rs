use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    }
}

use snazzy::items;

/// Returns a large shirt of the specified color
pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }
}
