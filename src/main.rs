extern crate image;
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use clap::{App, Arg};
use image::{DynamicImage, GenericImage};

mod color;
mod lights;
mod material;
mod objects;
mod vector3;

use color::Color;
use lights::Light;
use material::{Material, SurfaceType};
use objects::Object;
use vector3::Vector3;


#[derive(Serialize, Deserialize)]
pub struct Scene {
    width: u32,
    height: u32,
    lights: Vec<Light>,
    items: Vec<Object>,
}

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

fn create_ray(x: u32, y: u32, width: u32, height: u32) -> Ray {
    Ray {
        origin: Vector3::zero(),
        direction: Vector3 {
            x: ((x as f32) / (width as f32)) * 2.0 - 1.0, // 0..w -> -1.0..1.0
            y: ((y as f32) / (height as f32)) * 2.0 - 1.0, // 0..w -> -1.0..1.0
            z: -1.0
        }.normalize()
    }
}

pub struct Intersection<'a> {
    distance: f32,
    item: &'a Object,
}

impl Scene {
    fn send_ray(&self, ray: &Ray) -> Option<Intersection> {
        self.items
            .iter()
            .filter_map(|i| i.intersect(&ray).map(|d| Intersection { distance: d, item: i }))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

fn diffuse_color(scene: &Scene, hit: &Vector3, normal: &Vector3, material: &Material) -> Color {
    let mut color = Color { r: 0.0, g: 0.0, b: 0.0 };
    for light in &scene.lights {
        let light_direction = light.get_direction(&hit);
        let shadow_ray = Ray {
            origin: *hit + (*normal * 1e-6),
            direction: light_direction,
        };
        let between_light_and_object = scene.send_ray(&shadow_ray);
        let light_power = if between_light_and_object.is_none() || between_light_and_object.unwrap().distance > light.get_distance(&hit) {
            (normal.dot(&light_direction) as f32).max(0.0) * light.get_intensity(&hit)
        } else {
            0.0
        };
        let light_color = *light.get_color() * light_power * material.reflection;
        color = color + (material.color * light_color);
    }
    color
}

fn get_color(scene: &Scene, ray: &Ray, i: &Intersection, depth: u32) -> Color {
    let material = i.item.get_material();
    let hit = ray.origin + (ray.direction * i.distance);
    let normal = i.item.normal(&hit);

    // Always add diffuse
    let mut color = diffuse_color(scene, &hit, &normal, &material);

    // Add reflection depending on the surface type
    if let SurfaceType::Reflective { reflectivity } = material.surface {
        let reflection_ray = Ray {
            origin: hit + (normal * 1e-6),
            direction: ray.direction - (normal * ray.direction.dot(&normal) * 2.0),
        };
        color = color * (1.0 - reflectivity) + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
    }

    Color {
        r: color.r.max(0.0).min(1.0),
        g: color.g.max(0.0).min(1.0),
        b: color.b.max(0.0).min(1.0),
    }
}

fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= 5 {
        return Color { r: 0.0, g: 0.0, b: 0.0 };
    }

    scene.send_ray(&ray)
        .map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0 })
}

fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Color { r: 0.0, g: 0.0, b: 0.0 }.to_pixel();

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = create_ray(x, y, scene.width, scene.height);
            let intersection = scene.send_ray(&ray);

            match intersection {
                Some(i) => img.put_pixel(x, y, get_color(scene, &ray, &i, 0).to_pixel()),
                None => img.put_pixel(x, y, black),
            }
        }
    }

    img
}

fn main() {
    // CLI usage configuration
    let matches = App::new("raytracer")
        .version("0.1.0")
        .about("Basic raytracer in Rust.")
        .author("Spark-NF")
        .arg(Arg::with_name("SCENE")
            .help("Which scene file to load")
            .required(true)
            .index(1))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("The destination file for the raytracing output")
            .takes_value(true)
            .value_name("FILE"))
        .get_matches();

    // Parse scene file
    let scene_path = matches.value_of("SCENE").unwrap();
    let scene_file = File::open(scene_path).unwrap();
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    // Render scene to file
    let img = render(&scene);
    let out_path = matches.value_of("out").unwrap_or("out.png");
    let mut out = File::create(&Path::new(out_path)).unwrap();
    img.save(&mut out, image::ImageFormat::PNG).unwrap();
}
