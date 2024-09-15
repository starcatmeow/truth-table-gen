use tools::truth_table_gen::TruthTableEntry;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod prop;
mod tools;

pub struct JsError {
    err: anyhow::Error
}

impl Into<JsValue> for JsError {
    fn into(self) -> JsValue {
        JsValue::from_str(self.err.to_string().as_str())
    }
}

#[wasm_bindgen]
pub fn generate_truth_table(prop: &str) -> Result<Vec<TruthTableEntry>, JsError> {
    let prop1 = prop::parse_prop(prop).map_err(|e| JsError { err: e })?;
    let result = tools::truth_table_gen::truth_table_gen(&prop1);
    Ok(result)
}
