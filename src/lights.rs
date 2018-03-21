use Color;
use Vector3;

#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PointLight {
    pub position: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub enum Light {
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
    AmbientLight(AmbientLight),
}

impl Light {
    pub fn get_color(&self) -> &Color {
        match *self {
            Light::DirectionalLight(ref directional) => &directional.color,
            Light::PointLight(ref point) => &point.color,
            Light::AmbientLight(ref ambient) => &ambient.color,
        }
    }

    pub fn get_direction(&self, hit: &Vector3) -> Vector3 {
        match *self {
            Light::DirectionalLight(ref directional) => -directional.direction,
            Light::PointLight(ref point) => (point.position - *hit).normalize(),
            Light::AmbientLight(_) => Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    pub fn get_intensity(&self, hit: &Vector3) -> f32 {
        match *self {
            Light::DirectionalLight(ref directional) => directional.intensity,
            Light::PointLight(ref point) => {
                let squared_distance = (point.position - *hit).norm() as f32;
                point.intensity / (4.0 * ::std::f32::consts::PI * squared_distance)
            },
            Light::AmbientLight(ref ambient) => ambient.intensity,
        }
    }

    pub fn get_distance(&self, hit: &Vector3) -> f32 {
        match *self {
            Light::DirectionalLight(_) => ::std::f32::INFINITY,
            Light::PointLight(ref point) => (point.position - *hit).len(),
            Light::AmbientLight(_) => ::std::f32::INFINITY,
        }
    }
}
