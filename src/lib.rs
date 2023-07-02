#![allow(clippy::assertions_on_constants)]

use rysk::{Addressable, Hart};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;

mod component;

mod vm;
use vm::{JsRegionRef, JsVmArgs, RegionRef, Vm};

pub mod types {
    pub use rysk::Addressable;
    pub type Cause = rysk::Cause<u32>;
}

#[wasm_bindgen]
pub struct RyskVm(*mut Vm);
#[wasm_bindgen]
impl RyskVm {
    #[wasm_bindgen(constructor)]
    pub fn new(args: JsVmArgs) -> Result<RyskVm, JsValue> {
        let args = serde_wasm_bindgen::from_value(args.into())?;
        Ok(Self(Box::into_raw(Vm::new(&args))))
    }
    pub fn regions(&mut self) -> Vec<JsRegionRef> {
        self.hart
            .bus()
            .regions()
            .iter()
            .map(|v| RegionRef::to_js(v).into())
            .collect()
    }
    pub fn get_memory(&mut self, address: u32) -> JsRegionRef {
        self.hart.bus().memory(address).to_js().into()
    }
    pub fn set_memory(&mut self, address: u32, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            if self.hart.bus().write_u8(address + i as u32, *byte).is_err() {
                gloo_console::log!("Write fault");
            }
        }
    }
}
impl Deref for RyskVm {
    type Target = Vm;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
impl DerefMut for RyskVm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}
impl Drop for RyskVm {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.0) };
    }
}
