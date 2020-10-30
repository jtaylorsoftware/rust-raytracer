//! Scene-level abstractions for objects that can intersect with raycasts

use crate::material::Material;
use crate::math::{Point3, Ray, Vec3};
use std::sync::Arc;

/// Maintains a record of a ray intersection with a [`Hittable`] object.
#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.,
            front_face: false,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

/// Scene trait for intersecting with rays.
pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
/// Stores a list of hittable scene objects.
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

pub type Scene = HittableList;

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit = Some(temp_rec);
            }
        }

        return hit;
    }
}
