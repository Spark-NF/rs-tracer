use Color;
use Light;
use Vector3;

pub struct PointLight {
    pub position: Vector3,
    pub color: Color,
    pub intensity: f32,
}

impl Light for PointLight {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_direction(&self, hit: &Vector3) -> Vector3 {
        (self.position - *hit).normalize()
    }

    fn get_intensity(&self, hit: &Vector3) -> f32 {
        let squared_distance = (self.position - *hit).norm() as f32;
        self.intensity / (4.0 * ::std::f32::consts::PI * squared_distance)
    }

    fn get_distance(&self, hit: &Vector3) -> f32 {
        (self.position - *hit).len()
    }
}
