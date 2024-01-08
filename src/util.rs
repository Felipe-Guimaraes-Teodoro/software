use rand::Rng;
use cgmath::Vector3;

use std::f32::consts::PI;

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
    pub fn in_point_inside_polygon2d(x: f32, y: f32, polygon: &Vec<f32>) -> bool {
        let mut inside = false;

        let points: Vec<Point> = polygon
            .chunks(2)
            .map(|points| Point {
                y: points[0] + 400.0,
                x: points[1] + 400.0,
            })
            .collect();

        let mut j = points.len() - 1;

        for i in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            if (p1.y > y) != (p2.y > y)
                && x < p1.x + (p2.x - p1.x) * (y - p1.y) / (p2.y - p1.y)
            {
                inside = !inside;
            }

            j = i;
        }
        

        return inside;
    }

    pub fn rotate_polygon2d(polygon: &mut Vec<f32>, angle: f32) -> &Vec<f32> {
        let angle = angle + 1.57079633;

        let points: Vec<Point> = polygon
            .chunks(2)
            .map(|points| Point {
                x: points[0], // + pivot_x
                y: points[1], // + pivot_y
            })
            .collect();

        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        // Iterate over each pair of (x, y) coordinates and rotate them
        for i in 0..points.len() {
            let rot_x = points[i].x * cos_angle - points[i].y * sin_angle;
            let rot_y = points[i].x * sin_angle + points[i].y * cos_angle;

            polygon[i * 2] = rot_x; // - pivot_x ;
            polygon[i * 2 + 1] = rot_y; // - pivot_y;
        }

        polygon
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

    pub fn update(&mut self, timestep: f32, x: Vector3<f32>, xd: Vector3<f32>) -> Vector3<f32> {
        // if xd == None {
        //     xd = (x - self.xp) / timestep;
        //     self.xp = x;
        // } 

        self.y = self.y + timestep * self.yd;
        self.yd = self.yd + timestep * (x + self.k3*xd - self.y - self.k1*self.yd) / self.k2;

        self.y
    }
}
