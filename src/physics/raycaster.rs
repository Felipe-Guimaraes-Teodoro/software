use imgui::Ui;
use imgui::DrawListMut;
use crate::{environment::Mirror, util::Math};

use std::sync::mpsc::channel;

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

pub struct CollisionResult {
    collision_type: CollisionType,
    mirror: Option<Mirror>,
    x: f32,
    y: f32,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            depth: 0,
        }
    }
    pub fn cast(&mut self, start_pos: (f32, f32), angle: f32, length: f32, fdl: &DrawListMut, d: u32, ignore_mirror: Option<&Mirror>) {
        let (ray_dir_x, ray_dir_y) = (angle.cos(), angle.sin());

        let end_x = start_pos.0 + length * ray_dir_x;
        let end_y = start_pos.1 + length * ray_dir_y;
        let end_pos = (end_x, end_y);


        let c = check_collision(self.mirrors.clone(), start_pos, end_pos, ignore_mirror.copied());
                
        let line = fdl.add_line(
            [start_pos.0, start_pos.1], 
            [c.x, c.y], 
            [Math::random(0.7, 1.0), Math::random(0.7, 1.0), Math::random(0.7, 1.0), 1.0]
        ).thickness(1.0);
        line.build();
        
        match c.collision_type {
            CollisionType::Mirror => {
                let mirror = c.mirror.unwrap();
                let x = c.x;
                let y = c.y;

                if d < 20 {
                    self.cast((x, y), -mirror.angle + 3.1415, 400.0, fdl, d+1, Some(&mirror));
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

    
}

pub fn check_collision
    (
    mirrors: Vec<Mirror>,
    start_pos: (f32, f32), 
    end_pos: (f32, f32),
    ignore_mirror: Option<Mirror>
    ) -> CollisionResult 
{
    let (tx, rx) = channel();

    let num_iterations = 256;

    for mirror in mirrors {
    let tx = tx.clone();
    crate::GLOBAL_POOL.execute(move || {
        if ignore_mirror.is_some() {
            if mirror == ignore_mirror.unwrap() {
                tx.send(CollisionResult {collision_type: CollisionType::Void, mirror: None, x: end_pos.0, y: end_pos.1})
                    .expect("channel will be waiting for pool");

                return
                // return (CollisionType::Void, None, end_pos)
            }
        }

        for i in 0..num_iterations {
            let c_pos = lerp(start_pos, end_pos, i as f32 / num_iterations as f32);

            let x = c_pos.0;
            let y = c_pos.1;

            if mirror.in_bounds(x, y) {
                // return (CollisionType::Mirror, Some(*mirror), (x, y));
                
                tx.send(CollisionResult {collision_type: CollisionType::Mirror, mirror: Some(mirror), x, y})
                    .expect("channel will be waiting for pool");

                return
            }
            
        }
        // todo!();

    // return (CollisionType::Void, None, end_pos)
    
        tx.send(CollisionResult {collision_type: CollisionType::Void, mirror: None, x: end_pos.0, y: end_pos.1})
            .expect("channel will be waiting for pool");

    });
    } // for mirror
    
    let result = rx.recv().unwrap();

    return result
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

