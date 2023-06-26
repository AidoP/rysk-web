use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use rysk::Hart;

mod vm;
use vm::{RegionRef, Vm, VmArgsType};

pub mod types {
    pub use rysk::Addressable;
    pub type Cause = rysk::Cause<u32>;
}

#[wasm_bindgen]
pub struct RyskVm(*mut Vm);
#[wasm_bindgen]
impl RyskVm {
    #[wasm_bindgen(constructor)]
    pub fn new(args: VmArgsType) -> Result<RyskVm, JsValue> {
        let args = serde_wasm_bindgen::from_value(args.into())?;
        Ok(Self(Box::into_raw(Vm::new(&args))))
    }
    pub fn regions(&mut self) -> Vec<JsValue> {
        self.hart.bus().regions().iter().map(RegionRef::to_js).collect()
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
