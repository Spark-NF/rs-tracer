use Color;
use Light;
use Vector3;

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

impl Light for DirectionalLight {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_direction(&self, _hit: &Vector3) -> Vector3 {
        -self.direction
    }

    fn get_intensity(&self, _hit: &Vector3) -> f32 {
        self.intensity
    }

    fn get_distance(&self, _hit: &Vector3) -> f32 {
        ::std::f32::INFINITY
    }
}
