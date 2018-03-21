use Material;
use Ray;
use Vector3;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub radius: f32,
    pub center: Vector3,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        match *self {
            Object::Sphere(ref sphere) => {
                let ray_to_center = Vector3 {
                    x: sphere.center.x - ray.origin.x,
                    y: sphere.center.y - ray.origin.y,
                    z: sphere.center.z - ray.origin.z,
                };
                let dot = ray_to_center.dot(&ray.direction);
                let length_to_center = ray_to_center.len();
                let squared_distance = (length_to_center * length_to_center) - (dot * dot);
                let squared_radius = sphere.radius * sphere.radius;

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
            },
            Object::Plane(ref plane) => {
                let demon = plane.normal.dot(&ray.direction);
                if demon > 1e-6 {
                    let v = Vector3 {
                        x: plane.point.x - ray.origin.x,
                        y: plane.point.y - ray.origin.y,
                        z: plane.point.z - ray.origin.z,
                    };
                    let distance = v.dot(&plane.normal) / demon;
                    if distance >= 0.0 {
                        return Some(distance)
                    }
                }
                None
            },
        }
    }

    pub fn normal(&self, point: &Vector3) -> Vector3 {
        match *self {
            Object::Sphere(ref sphere) => (*point - sphere.center).normalize(),
            Object::Plane(ref plane) => -plane.normal,
        }
    }

    pub fn get_material(&self) -> &Material {
        match *self {
            Object::Sphere(ref sphere) => &sphere.material,
            Object::Plane(ref plane) => &plane.material,
        }
    }
}

