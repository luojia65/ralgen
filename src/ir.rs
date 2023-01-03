use std::collections::HashMap;

pub struct Ir {
    cores: HashMap<String, Core>,
    address_space: HashMap<String, Peripheral>,
    structs: HashMap<String, Struct>,
}

/// Logical processor. On RISC-V, it core
pub struct Core {

}

pub struct Peripheral {
    pub file: Option<String>,
    pub name: String,
    pub docs: Option<String>,
    pub address: String,
    pub struct_name: String,
}

pub struct Struct {

}
