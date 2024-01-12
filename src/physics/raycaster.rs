use imgui::Ui;
use imgui::DrawListMut;
use crate::{environment::Mirror, util::Math};

use std::sync::mpsc::channel;

#[derive(Clone)]
pub struct RayCaster {
    // raycaster should be able to read data from the world
    // maybe make it so raycaster is a member of world, and have
    // access to it
    //
    // other suggestion is to make a reference of world be a member
    // of raycaster 

    mirrors: Vec<Mirror>,
    pub lines: Vec<[f32; 4]>,
}

#[derive(Debug)]
pub struct CollisionResult {
    collision_type: CollisionType,
    mirror: Option<Mirror>,
    x: f32,
    y: f32,
}

pub struct CastResult {
    pub start_pos: (f32, f32),
    pub angle: f32,
    pub length: f32,
    pub ignore_mirror: Option<Mirror>,
    pub previous_lines: Vec<[f32; 4]>,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            lines: vec![],
        }
    }
    pub fn cast(&mut self, mut r: CastResult, depth: u32) -> Option<(CastResult, u32)> {
        let (ray_dir_x, ray_dir_y) = (r.angle.cos(), r.angle.sin());

        let end_x = r.start_pos.0 + r.length * ray_dir_x;
        let end_y = r.start_pos.1 + r.length * ray_dir_y;
        let end_pos = (end_x, end_y);

        let c = check_collision(self.mirrors.clone(), r.start_pos, end_pos, r.ignore_mirror.clone());
        
        match c.collision_type {
            CollisionType::Mirror => {
                let mirror = c.mirror.unwrap();
                let x = c.x;
                let y = c.y;

                let line = [r.start_pos.0, r.start_pos.1, x, y];
                self.lines.push(line);

                r.previous_lines.push(line);

                if depth < 20 {
                    return Some((CastResult {
                        start_pos: r.start_pos,
                        angle: -mirror.angle + 3.1415,
                        length: 400.0,
                        ignore_mirror: Some(mirror),
                        previous_lines: r.previous_lines,
                    }, depth + 1));
                }
            }

            CollisionType::Diffuse => {
                return None;
            }

            CollisionType::Void => {
                let x = c.x;
                let y = c.y;

                let line = [r.start_pos.0, r.start_pos.1, x, y];
                self.lines.push(line);

                r.previous_lines.push(line);
            }
        }

        None
    }

    pub fn clear_lines(&mut self) {
        self.lines.clear();
    }

    pub fn draw_lines(&mut self, fdl: &DrawListMut) {
        for line in &self.lines {
            let (x0, y0, x1, y1) = (line[0], line[1], line[2], line[3]);
            let line = fdl.add_line([x0, y0], [x1, y1], [1.0, 1.0, 1.0, 1.0]);
            // fdl.add_text([x1, y1], [1.0, 1.0, 1.0], format!("{:?}", [x1, y1]));
            line.build();
        }
        
    }

    pub fn update(&mut self, mirrors: &Vec<Mirror>) {
        self.mirrors = mirrors.to_vec();
    }
}

pub fn check_collision
    (
    mirrors: Vec<Mirror>,
    start_pos: (f32, f32), 
    end_pos: (f32, f32),
    ignore_mirror: Option<Mirror>
    ) -> CollisionResult 
{
    let num_iterations = 256;

    for mut mirror in mirrors {
        // in the case the collision we're checking if it's with the same mirror than before
        if ignore_mirror.is_some() {
            if mirror == ignore_mirror.unwrap() {
                // return (CollisionType::Void, None, end_pos)
                return CollisionResult {
                    collision_type: CollisionType::Void,
                    mirror: None,
                    x: end_pos.0,
                    y: end_pos.1,
                };
            }
        }

        for i in 0..num_iterations {
            let c_pos = lerp(start_pos, end_pos, i as f32 / num_iterations as f32);

            let x = c_pos.0;
            let y = c_pos.1;

            if mirror.in_bounds(x, y, mirror.pos) {
                // we hit a mirror
                return CollisionResult {
                    collision_type: CollisionType::Mirror,
                    mirror: Some(mirror),
                    x, y,
                };
            }
        }

        // // we didn't hit anything
        // return CollisionResult {
        //     collision_type: CollisionType::Void,
        //     mirror: None,
        //     x: end_pos.0,
        //     y: end_pos.1,
        // };
    

    } // for mirror
    
    // return the collision result
    return CollisionResult {
        collision_type: CollisionType::Void,
        mirror: None,
        x: end_pos.0,
        y: end_pos.1,
    }
}

pub fn lerp(s: (f32, f32), e: (f32, f32), t: f32)
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

#[derive(Debug, Copy, Clone)]
pub enum CollisionType {
    Mirror,
    Void,
    Diffuse,
}

