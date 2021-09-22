use gfs::canvas::Canvas;
use pixels::Error;

const VIEWPORT_SIZE: u32 = 1;
const PROJECTION_PLANE_Z: u32 = 1;
const BACKGROUND_COLOR: (u8, u8, u8) = (255, 255, 255);
const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const TITLE: &str = "Raytracer 01 - Graphics from Scratch";

struct Raytracer01 {
    camera_position: (i32, i32, i32),
    spheres: [Sphere; 3],
}

impl Raytracer01 {
    fn new() -> Raytracer01 {
        let camera_position = (0, 0, 0);
        let spheres: [Sphere; 3] = [
            Sphere::new((0, -1, 3), 1, (255, 0, 0)),
            Sphere::new((2, 0, 4), 1, (0, 0, 255)),
            Sphere::new((-2, 0, 4), 1, (0, 255, 0))
        ];

        Raytracer01 {
            camera_position,
            spheres,
        }
    }
}

impl gfs::harness::App for Raytracer01 {
    fn update(&self, canvas: &mut Canvas) {
        for x in -(canvas.width as i32 / 2)..(canvas.width as i32 / 2) {
            for y in -(canvas.height as i32 / 2)..(canvas.height as i32 / 2) {
                let direction = canvas_to_viewport(&canvas, (x, y));
                let color = trace_ray(&self.spheres, self.camera_position, direction, 1, i32::MAX);
                canvas.put_pixel((x, y), color);
            }
        }
    }
}


fn main() -> Result<(), Error> {
    let raytracer01 = Raytracer01::new();
    gfs::harness::run(WIDTH, HEIGHT, TITLE, raytracer01)
}

struct Sphere {
    center: (i32, i32, i32),
    radius: u32,
    color: (u8, u8, u8),
}

impl Sphere {
    fn new(center: (i32, i32, i32), radius: u32, color: (u8, u8, u8)) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

fn canvas_to_viewport(canvas: &Canvas, p2d: (i32, i32)) -> (f64, f64, f64) {
    (
        p2d.0 as f64 * VIEWPORT_SIZE as f64 / canvas.width as f64,
        p2d.1 as f64 * VIEWPORT_SIZE as f64 / canvas.height as f64,
        PROJECTION_PLANE_Z as f64
    )
}

fn trace_ray(spheres: &[Sphere], origin: (i32, i32, i32), direction: (f64, f64, f64), t_min: i32, t_max: i32) -> (u8, u8, u8) {
    let mut closest_t = f64::MAX;
    let mut closest_sphere = None;
    for sphere in spheres {
        let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);
        if t1 < closest_t && (t_min as f64) < t1 && t1 < (t_max as f64) {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if t2 < closest_t && (t_min as f64) < t2 && t2 < (t_max as f64) {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    match closest_sphere {
        Some(sphere) => sphere.color,
        None => BACKGROUND_COLOR
    }
}

fn intersect_ray_sphere(origin: (i32, i32, i32), direction: (f64, f64, f64), sphere: &Sphere) -> (f64, f64) {
    let oc = subtract(origin, sphere.center);

    let k1 = dot_product(direction, direction);
    let k2 = 2.0 * dot_product(oc, direction);
    let k3 = dot_product(oc, oc) - (sphere.radius as f64 * sphere.radius as f64);

    let discriminant = k2 * k2 - 4.0 * k1 * k3;
    if discriminant < 0.0 {
        return (f64::MAX, f64::MAX);
    }

    let t1 = (-k2 + discriminant.sqrt()) / (2.0 * k1);
    let t2 = (-k2 - discriminant.sqrt()) / (2.0 * k1);
    (t1, t2)
}

fn dot_product(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

fn subtract(v1: (i32, i32, i32), v2: (i32, i32, i32)) -> (f64, f64, f64) {
    (v1.0 as f64 - v2.0 as f64, v1.1 as f64 - v2.1 as f64, v1.2 as f64 - v2.2 as f64)
}
