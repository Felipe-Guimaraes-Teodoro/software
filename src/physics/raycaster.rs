use imgui::Ui;
use imgui::DrawListMut;
use crate::{environment::Mirror, util::Math};

pub struct RayCaster {
    // raycaster should be able to read data from the world
    // maybe make it so raycaster is a member of world, and have
    // access to it
    //
    // other suggestion is to make a reference of world be a member
    // of raycaster 

    mirrors: Vec<Mirror>,
    depth: u32,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            depth: 0,
        }
    }
    pub fn cast(&mut self, start_pos: (f32, f32), angle: f32, length: f32, fdl: &DrawListMut, d: u32) {
        // direction components
        //

        let (ray_dir_x, ray_dir_y) = (angle.cos(), angle.sin());

        let end_x = start_pos.0 + length * ray_dir_x;
        let end_y = start_pos.1 + length * ray_dir_y;
        let end_pos = (end_x, end_y);


        let c = self.check_collision(start_pos, end_pos);
    
                
        
        match c.0 {
            CollisionType::Mirror => {
                let mirror = c.1.unwrap();
                let x = c.2.0;
                let y = c.2.1;

                if d < 20 {
                    let line = fdl.add_line(
                        [start_pos.0 * 800.0, start_pos.1 * 800.0], 
                        [x * 800.0, y * 800.0], 
                        [Math::random(0.5, 1.0), Math::random(0.5, 1.0), Math::random(0.5, 1.0), 1.0]
                    ).thickness(1.0);

                    line.build();

                    self.cast((x + 0.125, y), -mirror.angle + 3.1415, 2.0, fdl, d+1);
                }
            }

            CollisionType::Diffuse => {
            }

            CollisionType::Void => {
            }
        }
        
    }

    pub fn update(&mut self, mirrors: &Vec<Mirror>) {
        self.mirrors = mirrors.to_vec();
    }

    pub fn check_collision
        (&mut self, 
        start_pos: (f32, f32), 
        end_pos: (f32, f32)) -> (CollisionType, Option<Mirror>, (f32, f32)) 
    {
        for mirror in &self.mirrors {
            for i in 0..32 {
                let c_pos = self.lerp(start_pos, end_pos, i as f32 / 32.0);

                let x = c_pos.0;
                let y = c_pos.1;

                if mirror.in_bounds(x, y) {
                    return (CollisionType::Mirror, Some(*mirror), (x, y));
                }
                
            }
            // todo!();
        }

        (CollisionType::Void, None, (0.0, 0.0))
    }

    pub fn lerp(&self, s: (f32, f32), e: (f32, f32), t: f32)
    -> (f32, f32) 
    {
        let sx = s.0;
        let sy = s.1;

        let ex = e.0;
        let ey = e.1;

        let lx = sx + (ex - sx) * t;
        let ly = sy + (ey - sy) * t;

        (lx, ly)
    }
}

pub enum CollisionType {
    Mirror,
    Void,
    Diffuse,
}

