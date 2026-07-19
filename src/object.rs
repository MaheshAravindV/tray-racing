use crate::hittables::Hittable;
use crate::materials::Material;

struct Object<S, M> where S: Hittable, M: Material {
    shape: S,
    material: M
}