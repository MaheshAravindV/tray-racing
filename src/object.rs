use crate::hittables::Hittable;
use crate::materials::Material;

pub trait Object {
    fn get_hittable(&self) -> &dyn Hittable;
    fn get_material(&self) -> &dyn Material;
}

pub struct StructObject {
    hittable: Box<dyn Hittable>,
    material: Box<dyn Material>
}

impl StructObject {
    pub fn new(hittable: Box<dyn Hittable>, material: Box<dyn Material>) -> Self {
        Self { hittable, material }
    }
}

impl Object for StructObject {
    fn get_hittable(&self) -> &dyn Hittable {
        self.hittable.as_ref()
    }

    fn get_material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}