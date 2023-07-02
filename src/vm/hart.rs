use super::bus::Bus;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const HART_ARGS_TYPE: &'static str = r#"
export class HartArgs {
    memory_pages: number;
}
"#;

#[derive(Deserialize, Serialize)]
pub struct HartArgs {
    pub memory_pages: u32,
}

pub struct Hart {
    registers: [u32; 32],
    bus: Bus,
}
impl Hart {
    pub fn new(args: &HartArgs) -> Self {
        Self {
            registers: [0; 32],
            bus: Bus::new(args.memory_pages),
        }
    }
}
impl rysk::Hart<u32, Bus> for Hart {
    #[inline]
    fn load_register(&self, r: rysk::Register) -> u32 {
        self.registers[r as usize]
    }
    #[inline]
    fn store_register(&mut self, r: rysk::Register, v: u32) {
        self.registers[r as usize] = v
    }
    #[inline]
    fn bus(&mut self) -> &mut Bus {
        &mut self.bus
    }
}
