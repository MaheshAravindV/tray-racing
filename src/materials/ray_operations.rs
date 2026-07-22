use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn reflected_direction(source_ray: &Ray, normal: Vec3) -> Vec3 {
    let source_direction = source_ray.direction();
    let reflected_direction = source_direction - 2.0 * (source_direction * normal) * normal;
    reflected_direction
}

pub fn refracted_direction(source_ray: &Ray,
                           normal: Vec3,
                           hit_front_face: bool,
                           refractive_index: f64) -> Option<Vec3> {
    let relative_refractive_index = if hit_front_face {
        source_ray.refractive_index_of_medium() / refractive_index
    } else {
        refractive_index / source_ray.refractive_index_of_medium()
    };

    let cos_theta = (-source_ray.direction() * normal).clamp(0.0, 1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let can_refract = (relative_refractive_index * sin_theta) <= 1.0;
    
    match can_refract {
        false => None,
        true => {
            let refracted_perpendicular =
            relative_refractive_index * (source_ray.direction() + cos_theta * normal);
            let refracted_perpendicular_magnitude = refracted_perpendicular.magnitude();
            let refracted_parallel_magnitude =
            (1.0 - refracted_perpendicular_magnitude * refracted_perpendicular_magnitude).sqrt();
            let refracted_parallel = refracted_parallel_magnitude * -normal;
    
            let refracted_direction = refracted_parallel + refracted_perpendicular;
            Some(refracted_direction)
        }
    }
}

pub fn diffused_direction(normal: Vec3, hit_front_face: bool) -> Vec3 {
    let mut diffused_direction = Vec3::random_unit_vector();

    let directed_normal = if hit_front_face {
        normal
    } else {
        -normal
    };

    let same_dir = diffused_direction * directed_normal;
    if same_dir < 0.0 {
        diffused_direction = -diffused_direction
    }

    diffused_direction += directed_normal;

    const EPSILON_MAGNITUDE: f64 = 1.73 * 1e-8;
    if diffused_direction.magnitude() < EPSILON_MAGNITUDE {
        diffused_direction = directed_normal
    }

    diffused_direction
}