use crate::vec3::Vec3;

pub struct Scene {
    camera: Vec3,
    focal_len: f32,
    width: isize,  // px
    height: isize, // px
    vp_width: f32,
    vp_height: f32,
}

impl Scene {
    fn new(width: isize, height: isize) -> Self {
        let vp_height = 2f32;
        Self {
            camera: Vec3::from(0.0, 0.0, 0.0),
            focal_len: 1.0,
            width,
            height,
            vp_height,
            vp_width: (width as f32) / (height as f32) * vp_height,
        }
    }
}
