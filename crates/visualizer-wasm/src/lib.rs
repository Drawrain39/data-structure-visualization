use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_trace_json(algorithm: &str, input_json: &str) -> Result<String, JsValue> {
    let values: Vec<i32> = serde_json::from_str(input_json)
        .map_err(|e| JsValue::from_str(&format!("invalid input JSON: {}", e)))?;

    let steps = visualizer_core::sorting::generate_trace(algorithm, &values)
        .map_err(|e| JsValue::from_str(&e))?;

    serde_json::to_string(&steps)
        .map_err(|e| JsValue::from_str(&format!("serialization error: {}", e)))
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
