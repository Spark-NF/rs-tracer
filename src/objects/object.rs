use Material;
use Ray;
use Vector3;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn normal(&self, point: &Vector3) -> Vector3;
    fn get_material(&self) -> &Material;
}
