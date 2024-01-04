enum CollisionType {
    Mirror,
    Void,
    Diffuse,
}

pub struct RayCaster {
    // raycaster should be able to read data from the world
    // maybe make it so raycaster is a member of world, and have
    // access to it
}

impl RayCaster {
    pub fn cast(start_pos: (f32, f32), angle: f32, length: f32) {
        // direction components

        let (ray_dir_x, ray_dir_y) = (angle.cos(), angle.sin());

        let end_x = start_pos.0 + length * ray_dir_x;
        let end_y = start_pos.1 + length * ray_dir_y;
        let end_pos = (end_x, end_y);

        //let c: enum::CollisionType = check_collision(start_pos, end_pos);
    
        // draw a line of the raycast
        
        /*
         * match c {
         *      Mirror => {
         *          // calculate normal and re-cast in other direction 
         *      },
         *
         *      Diffuse => {
         *         // re-cast in random direction, or re-cast according 
         *         to some bsdf
         *      }
         *
         *      Void => {
         *          // terminate ray tracing
         *      }
         * }
         *
         */
    }
}
