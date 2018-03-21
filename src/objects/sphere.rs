use Material;
use Object;
use Ray;
use Vector3;

pub struct Sphere {
    pub radius: f32,
    pub center: Vector3,
    pub material: Material,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let ray_to_center = Vector3 {
            x: self.center.x - ray.origin.x,
            y: self.center.y - ray.origin.y,
            z: self.center.z - ray.origin.z,
        };
        let dot = ray_to_center.dot(&ray.direction);
        let length_to_center = ray_to_center.len();
        let squared_distance = (length_to_center * length_to_center) - (dot * dot);
        let squared_radius = self.radius * self.radius;

        if squared_distance > squared_radius {
            return None
        }

       let root = (squared_radius - squared_distance).sqrt();
       let solution1 = dot - root;
       let solution2 = dot + root;

       if solution1 < 0.0 && solution2 < 0.0 {
           return None;
       }

       Some(solution1.min(solution2))
    }

    fn normal(&self, point: &Vector3) -> Vector3 {
        (*point - self.center).normalize()
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}
