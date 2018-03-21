use Color;

pub enum SurfaceType {
    Diffuse,
    Reflective {
        reflectivity: f32,
    },
}

pub struct Material {
    pub reflection: f32,
    pub color: Color,
    pub surface: SurfaceType,
}
