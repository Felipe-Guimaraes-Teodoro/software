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
    draw_list: Vec<[f32; 4]>,
}
impl RayCaster {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            depth: 0,
            draw_list: vec![],
        }
    }

    pub fn can_draw(&mut self) -> bool {
        if self.draw_list.len() > 256 {
            false
        } else {
            true
        }
    }

    pub fn cast(&mut self, start_pos: (f32, f32), angle: f32, length: f32, d: u32, previous_mirror: Option<Mirror>) {
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
        if d > 20 { return } 

        let c = Self::check_collision(&self.mirrors, start_pos, previous_mirror, end_pos);

        match c.col_type {
            CollisionType::Mirror => {
                let mirror = c.mirror.unwrap();
                let x = c.end_pos.0;
                let y = c.end_pos.1;

                // maybe use vec.swap_remove() to add line
                self.draw_list.push([start_pos.0, start_pos.1, x, y]);

                let normal = -mirror.angle + 3.1415;
                let incident = mirror.angle + angle;

                self.cast((x, y), normal - incident, 400.0, d+1, Some(mirror));
            }

            CollisionType::Diffuse => {
            }
            
            CollisionType::Void => {
                // dbg!("VOID");
                let x = c.end_pos.0;
                let y = c.end_pos.1;

                self.draw_list.push([start_pos.0, start_pos.1, x, y]);

                self.cast((x, y), angle, 10.0, d+5, None);
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
                if mirror.in_bounds(x, y, mirror.pos.into()) {
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

    pub fn draw_lines(&mut self, fdl: &DrawListMut) {
        for line in &self.draw_list {
            let line = fdl.add_line(
                [line[0], line[1]],
                [line[2], line[3]],
                [Math::random(0.7, 1.0), Math::random(0.7, 1.0), Math::random(0.7, 1.0), 0.2] 
            ).thickness(1.0);
            line.build();
        }

        if self.draw_list.len() > 256 {
            for _i in 0..32 {
                self.draw_list.remove(0);
            }
        }
        // self.draw_list.clear();
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
