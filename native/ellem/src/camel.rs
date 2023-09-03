use rustler::{NifStruct, ResourceArc};

#[derive(Debug, NifStruct)]
#[module = "Camel"]
pub struct Camel {}

#[rustler::nif]
pub fn camel() -> ResourceArc<Camel> {
    ResourceArc::new(Camel {})
}

#[rustler::nif]
pub fn model_path(camel_resource: ResourceArc<Camel>, path: String) -> ResourceArc<Camel> {
    camel_resource
}

#[rustler::nif]
pub fn generate(camel_resource: ResourceArc<Camel>, prompt: String) -> ResourceArc<Camel> {
    camel_resource
}
