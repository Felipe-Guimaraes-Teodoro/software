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
    pub fn cast(&mut self, start_pos: (f32, f32), angle: f32, length: f32, fdl: &DrawListMut, d: u32, previous_mirror: Option<Mirror>) {
        let (ray_dir_x, ray_dir_y) = (angle.cos(), angle.sin());
        let end_x = start_pos.0 + length * ray_dir_x;
        let end_y = start_pos.1 + length * ray_dir_y;
        let end_pos = (end_x, end_y);

        let c = self.check_collision(start_pos, previous_mirror, end_pos);


        match c.0 {
            CollisionType::Mirror => {
                let mirror = c.1.unwrap();
                let x = c.2.0;
                let y = c.2.1;

                let line = fdl.add_line(
                    [start_pos.0, start_pos.1], 
                    [x, y], 
                    [Math::random(0.7, 1.0), Math::random(0.7, 1.0), Math::random(0.7, 1.0), 1.0]
                ).thickness(1.0);
                line.build();

                if d < 20 {
                    let normal = -mirror.angle + 3.1415;
                    let incident = mirror.angle + angle;

                    self.cast((x, y), normal - incident, 400.0, fdl, d+1, Some(mirror));
                }
            }

            CollisionType::Diffuse => {
            }
            
            CollisionType::Void => {
                // dbg!("VOID");
                let line = fdl.add_line(
                    [start_pos.0, start_pos.1],
                    [end_pos.0, end_pos.1],
                    [Math::random(0.7, 1.0), Math::random(0.7, 1.0), Math::random(0.7, 1.0), 1.0] 
                ).thickness(1.0);
                line.build();
            }
        }
    }


    pub fn update(&mut self, mirrors: &Vec<Mirror>) {
        self.mirrors = mirrors.to_vec();
    }
    pub fn check_collision
        (&mut self, 
        start_pos: (f32, f32), 
        previous_mirror: Option<Mirror>,
        end_pos: (f32, f32)) -> (CollisionType, Option<Mirror>, (f32, f32)) 
    {
        for mirror in &mut self.mirrors {
            if previous_mirror.is_some() {
                if *mirror == previous_mirror.unwrap() {
                    return (CollisionType::Void, previous_mirror, (0.0, 0.0));
                }
            }
            for i in 0..256 {
                let c_pos = Self::lerp(start_pos, end_pos, i as f32 / 256.0);
                let x = c_pos.0;
                let y = c_pos.1;
                if mirror.in_bounds(x, y, mirror.pos) {
                    return (CollisionType::Mirror, Some(*mirror), (x, y));
                }
                
            }
            // todo!();
            // return (CollisionType::Void, None, (0.0, 0.0));
        }
        (CollisionType::Void, None, (0.0, 0.0))
    }

    pub fn lerp(s: (f32, f32), e: (f32, f32), t: f32) -> (f32, f32) {
        let sx = s.0;
        let sy = s.1;
        let ex = e.0;
        let ey = e.1;
        let lx = sx + (ex - sx) * t;
        let ly = sy + (ey - sy) * t;
        (lx, ly)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CollisionType {
    Mirror,
    Void,
    Diffuse,
}
