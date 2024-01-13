use imgui::Ui;
use imgui::DrawListMut;
use crate::{environment::Mirror, util::Math};

const NUM_ITERATIONS: i32 = 256;

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

        // let (tx, rx) = std::sync::mpsc::channel();
        // crate::GLOBAL_POOL.execute(move || {
            // tx.send(Self::check_collision(mirrors, start_pos, previous_mirror, end_pos))
                // .expect("channel will be waiting for pool");
        // });

        // let c = rx.recv().unwrap();

        let c = Self::check_collision(&self.mirrors, start_pos, previous_mirror, end_pos);

        match c.col_type {
            CollisionType::Mirror => {
                let mirror = c.mirror.unwrap();
                let x = c.end_pos.0;
                let y = c.end_pos.1;

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
                let x = c.end_pos.0;
                let y = c.end_pos.1;

                let line = fdl.add_line(
                    [start_pos.0, start_pos.1],
                    [x, y],
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
        (mirrors: &Vec<Mirror>,
        start_pos: (f32, f32), 
        previous_mirror: Option<Mirror>,
        end_pos: (f32, f32)) -> CollisionResult 
    {
        for mirror in mirrors {
            if previous_mirror.is_some() {
                if *mirror == previous_mirror.unwrap() {
                    return CollisionResult { 
                        col_type: CollisionType::Void, 
                        mirror: previous_mirror, 
                        end_pos, 
                    };
                }
            }
            for i in 0..NUM_ITERATIONS {
                let c_pos = Self::lerp(start_pos, end_pos, i as f32 / NUM_ITERATIONS as f32);
                let x = c_pos.0;
                let y = c_pos.1;
                if mirror.in_bounds(x, y, mirror.pos) {
                    return CollisionResult {
                        col_type: CollisionType::Mirror, 
                        mirror: Some(*mirror), 
                        end_pos: (x, y)
                    };
                }
                
            }
            
        }

        CollisionResult { 
            col_type: CollisionType::Void, 
            mirror: None, 
            end_pos,
        }
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

pub struct CollisionResult {
    col_type: CollisionType,
    mirror: Option<Mirror>,
    end_pos: (f32, f32),
}
