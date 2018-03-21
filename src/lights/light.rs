use Color;
use Vector3;

pub trait Light {
    fn get_color(&self) -> &Color;
    fn get_direction(&self, hit: &Vector3) -> Vector3;
    fn get_intensity(&self, hit: &Vector3) -> f32;
    fn get_distance(&self, hit: &Vector3) -> f32;
}
