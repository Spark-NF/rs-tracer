use Color;

#[derive(Serialize, Deserialize)]
pub enum SurfaceType {
    Diffuse,
    Reflective {
        reflectivity: f32,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub reflection: f32,
    pub color: Color,
    pub surface: SurfaceType,
}
