use rand::Rng;
use cgmath::Vector3;

pub struct Math {}
pub struct Geometry {}

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

        // Iterate through each pair of consecutive vertices of the polygon
        for i in (0..polygon.len()).step_by(2) {
            let j = (i + 2) % polygon.len();
            let vertex_i_x = polygon[i];
            let vertex_i_y = polygon[i + 1];
            let vertex_j_x = polygon[j];
            let vertex_j_y = polygon[j + 1];

            // Check if the point is to the left of the edge formed by the vertices
            if (vertex_i_y <= y && y < vertex_j_y) || (vertex_j_y <= y && y < vertex_i_y) {
                if x < (vertex_j_x - vertex_i_x) * (y - vertex_i_y) / (vertex_j_y - vertex_i_y) + vertex_i_x {
                    inside = !inside;
                }
            }
        }

        inside
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
