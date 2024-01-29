use rand::Rng;
use cgmath::Vector3;

use std::f32::consts::PI;

pub struct IntersectionResult {}

pub struct Math {}
pub struct Geometry {}
pub struct SecondOrderDynamics { // make it so that input x yields in a smooth, natural output y
    xp: Vector3<f32>, // previous inputs
    y: Vector3<f32>, 
    yd: Vector3<f32>,

    //constants
    k1: f32,
    k2: f32, 
    k3: f32,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32, y: f32,
}

impl Math {
    pub fn random
        <T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>
        (l: T, u: T) -> T 
    {
        let mut rng = rand::thread_rng(); 

        return rng.gen_range(l..u);
    }

}

impl Geometry {
    pub fn in_point_inside_polygon2d(x: f32, y: f32, polygon: &Vec<f32>, w: f32, h: f32) -> bool {
        let mut inside = false;
        let num_points = polygon.len() / 2;

        let mut j = num_points - 1;

        for i in 0..num_points {
            let p1_y = polygon[i * 2] + h / 2.0;
            let p1_x = polygon[i * 2 + 1] + w / 2.0;

            let p2_y = polygon[j * 2] + h / 2.0;
            let p2_x = polygon[j * 2 + 1] + w / 2.0;

            if (p1_y > y) != (p2_y > y) && x < p1_x + (p2_x - p1_x) * (y - p1_y) / (p2_y - p1_y) {
                inside = !inside;
            }

            j = i;
        }

        inside
    }

    pub fn rotate_polygon2d(polygon: &mut Vec<f32>, angle: f32, pivot: Vec<f32>) -> &Vec<f32> {
        let angle = angle + 1.57079633;

        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        for i in 0..polygon.len() / 2 {
            let x = polygon[i * 2] - pivot[0];
            let y = polygon[i * 2 + 1] - pivot[1];

            let rot_x = x * cos_angle - y * sin_angle;
            let rot_y = x * sin_angle + y * cos_angle;

            polygon[i * 2] = rot_x - pivot[1];
            polygon[i * 2 + 1] = rot_y + pivot[0];
        }

        polygon
    }

    pub fn line_intersects_polygon2d() -> Option<IntersectionResult> {
        todo!();
    }
}

impl SecondOrderDynamics {
    pub fn new(f: f32, z: f32, r: f32, x0: Vector3<f32>) -> Self {
        let k1 = z / (PI * f);
        let k2 = 1.0 / ((2.0 * PI * f) * (2.0 * PI * f));
        let k3 = r * z / (2.0 * PI * f);
        
        let xp = x0;
        let y = x0;
        let yd = cgmath::vec3(0.0, 0.0, 0.0);

        Self {
            k1,
            k2,
            k3,

            xp,
            y,
            yd,
        }
    }

    pub fn update(&mut self, timestep: f32, x: Vector3<f32>) -> Vector3<f32> {
        // if xd == None {
        //     xd = (x - self.xp) / timestep;
        //     self.xp = x;
        // } 
        let xd = (x - self.xp) / timestep;
        self.xp = x;

        self.y = self.y + timestep * self.yd;
        self.yd = self.yd + timestep * (x + self.k3*xd - self.y - self.k1*self.yd) / self.k2;

        self.y
    }


    pub fn set_starting_point(&mut self, x: Vector3<f32>) {
        todo!();
    }
}
