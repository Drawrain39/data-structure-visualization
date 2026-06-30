use wasm_bindgen::prelude::*;

// ── Trace generation ───────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn generate_trace_json(algorithm: &str, input_json: &str) -> Result<String, JsValue> {
    let values: Vec<i32> = serde_json::from_str(input_json)
        .map_err(|e| JsValue::from_str(&format!("invalid input JSON: {}", e)))?;

    let steps = visualizer_core::generate_trace(algorithm, &values)
        .map_err(|e| JsValue::from_str(&e))?;

    serde_json::to_string(&steps)
        .map_err(|e| JsValue::from_str(&format!("serialization error: {}", e)))
}

// ── Catalog ────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn get_algorithm_catalog_json() -> Result<String, JsValue> {
    let catalog = visualizer_catalog::get_catalog_json();
    serde_json::to_string(&catalog)
        .map_err(|e| JsValue::from_str(&format!("serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn get_algorithm_meta_json() -> Result<String, JsValue> {
    let metas = visualizer_catalog::get_algorithm_meta_json();
    serde_json::to_string(&metas)
        .map_err(|e| JsValue::from_str(&format!("serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn get_code_sample_json(algorithm: &str) -> Result<String, JsValue> {
    visualizer_catalog::get_code_sample_json(algorithm)
        .ok_or_else(|| JsValue::from_str(&format!("algorithm not found: {}", algorithm)))
}

#[wasm_bindgen]
pub fn get_single_code_sample(algorithm: &str, lang: &str) -> Result<String, JsValue> {
    visualizer_catalog::get_single_code_sample(algorithm, lang)
        .ok_or_else(|| JsValue::from_str(&format!("code sample not found for algorithm: {}, lang: {}", algorithm, lang)))
}

#[wasm_bindgen]
pub fn list_algorithms() -> Result<String, JsValue> {
    let catalog = visualizer_catalog::get_catalog_json();
    let algorithms: Vec<String> = catalog["algorithms"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v["algorithm"].as_str().map(String::from)).collect())
        .unwrap_or_default();
    serde_json::to_string(&algorithms)
        .map_err(|e| JsValue::from_str(&format!("serialization error: {}", e)))
}

// ── Init ───────────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
