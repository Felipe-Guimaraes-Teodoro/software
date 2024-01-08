use rand::Rng;
use cgmath::Vector3;

pub struct Math {}
pub struct Geometry {}

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
                x: points[0],
                y: points[1],
            })
            .collect();

        let p1 = &points[0];

        for i in 0..points.len() {
            let p2 = &points[i % points.len()]; 

            if y > f32::min(p1.y, p2.y) {
                if y <= f32::max(p1.y, p2.y) {
                    if x <= f32::max(p1.x, p2.x) {
                        let x_intersection
                            = (y - p1.y) * (p2.x - p1.x)
                                / (p2.y - p1.y)
                            + p1.x;

                        if p1.x == p2.x || x <= x_intersection {
                            inside = !inside;
                        }
                    }
                }
            }
        }

        return inside;
    }

    pub fn rotate_polygon2d(polygon: &mut Vec<f32>, angle: f32) -> &Vec<f32> {
        // Iterate over each pair of (x, y) coordinates and rotate them
        for i in (0..polygon.len()).step_by(2) {
            let x = polygon[i];
            let y = polygon[i + 1];

            // Rotate the point using the 2D rotation matrix
            let new_x = x * angle.cos() + y * -angle.sin();
            let new_y = x * angle.sin() + y * angle.cos();

            // Update the polygon with the rotated coordinates
            polygon[i] = new_x;
            polygon[i + 1] = new_y;
        }

        polygon
    }
}
