use serde::{ser::SerializeStruct, Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod bus;
mod hart;
mod mem;

pub use hart::{Hart, HartArgs};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "VmArgs")]
    pub type JsVmArgs;
    #[wasm_bindgen(typescript_type = "RegionRef")]
    pub type JsRegionRef;
}

#[wasm_bindgen(typescript_custom_section)]
const VM_ARGS_TYPE: &'static str = r#"
export class VmArgs {
    hart: HartArgs;
}
"#;

#[derive(Deserialize, Serialize)]
pub struct VmArgs {
    pub hart: HartArgs,
}

pub struct Vm {
    pub hart: Hart,
}
impl Vm {
    pub fn new(args: &VmArgs) -> Box<Self> {
        Box::new(Self {
            hart: Hart::new(&args.hart),
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Deserialize)]
#[repr(u32)]
pub enum RegionType {
    Dram,
    None,
    Io,
}
impl Serialize for RegionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}
#[wasm_bindgen]
pub fn region_type_to_string(ty: RegionType) -> String {
    ty.to_string()
}
impl std::fmt::Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dram => write!(f, "DRAM Region"),
            Self::None => write!(f, "None"),
            Self::Io => write!(f, "Memory Mapped I/O Region"),
        }
    }
}

#[wasm_bindgen]
pub struct MemoryRef {
    pub ptr: *const u8,
    pub len: u32,
}
impl MemoryRef {
    pub const fn null() -> Self {
        Self {
            ptr: core::ptr::null(),
            len: 0,
        }
    }
    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}
impl From<&[u8]> for MemoryRef {
    fn from(value: &[u8]) -> Self {
        Self {
            ptr: value.as_ptr(),
            len: value.len() as u32,
        }
    }
}
impl Serialize for MemoryRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Memory", 4)?;
        state.serialize_field("ptr", &(self.ptr as usize))?;
        state.serialize_field("len", &self.len)?;
        state.end()
    }
}
#[wasm_bindgen]
pub struct RegionRef {
    pub ty: RegionType,
    pub address: u32,
    pub len: u32,
}
impl RegionRef {
    pub fn none(address: u32, len: u32) -> Self {
        Self {
            ty: RegionType::None,
            address,
            len,
        }
    }
    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}
impl Serialize for RegionRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RegionRef", 4)?;
        state.serialize_field("ty", &self.ty)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("len", &self.len)?;
        state.end()
    }
}
