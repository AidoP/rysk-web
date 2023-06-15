use rysk;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn inspect(instruction: u32) -> String {
    format!("{:?}", rysk::Instruction::new(instruction))
}
