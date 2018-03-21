use Material;
use Object;
use Ray;
use Vector3;

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let demon = self.normal.dot(&ray.direction);
        if demon > 1e-6 {
            let v = Vector3 {
                x: self.point.x - ray.origin.x,
                y: self.point.y - ray.origin.y,
                z: self.point.z - ray.origin.z,
            };
            let distance = v.dot(&self.normal) / demon;
            if distance >= 0.0 {
                return Some(distance)
            }
        }
        None
    }

    fn normal(&self, _point: &Vector3) -> Vector3 {
        -self.normal
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}
