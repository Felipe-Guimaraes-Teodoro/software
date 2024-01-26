use imgui::DrawListMut;
use crate::{environment::Mirror, util::Math};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

const NUM_ITERATIONS: i32 = 128;
const MAX_DEPTH: u32 = 32;
const MAX_LINES: usize = 64;

pub static GLOBAL_CASTER: Lazy<Arc<Mutex<RayCaster>>> = Lazy::new(|| {
    Arc::new(Mutex::new(RayCaster::new()))
});

pub fn run() {
    let global_caster = Arc::clone(&GLOBAL_CASTER);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(32));

            if let Ok(mut locked_caster) = global_caster.lock() {
                let height = locked_caster.height;
                // locked_caster.update(&vec![]);
                //
                if locked_caster.can_draw() {
                    locked_caster.cast((0.0, height / 2.0), 0.0, 400.0, 0, None);
                }
            }
        }
    });
}

pub struct RayCaster {
    mirrors: Vec<Mirror>,
    draw_list: Vec<[f32; 4]>,

    width: f32,
    height: f32,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            draw_list: vec![],

            width: 800.0,
            height: 800.0,
        }
    }

    pub fn set_framebuffer_size(&mut self, width: f32, height: f32) {
        self.height = height;
        self.width = width;
    }

    pub fn can_draw(&mut self) -> bool {
        if self.draw_list.len() > MAX_LINES {
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

        if d > MAX_DEPTH { return } 

        let c = Self::check_collision(self.width, self.height, &self.mirrors, start_pos, previous_mirror, end_pos);

        match c.col_type {
            CollisionType::Mirror => {
                let mirror = c.mirror.unwrap();

                let x = c.end_pos.0;
                let y = c.end_pos.1;

                self.draw_list.push([start_pos.0, start_pos.1, x, y]);

                let normal = -mirror.angle + 3.1415;
                let incident = mirror.angle + angle;

                self.cast((x, y), normal - incident, 400.0, d+1, Some(mirror));
            }

            CollisionType::Diffuse => {
            }
            
            CollisionType::Void => {
                let x = c.end_pos.0;
                let y = c.end_pos.1;

                self.draw_list.push([start_pos.0, start_pos.1, x, y]);

                // bigger depth penalty for when the cast hits void
                self.cast((x, y), angle, 400.0, d+5, c.mirror);
            }
        }
    }


    pub fn update(&mut self, mirrors: &Vec<Mirror>) {
        self.mirrors = mirrors.to_vec();
    }

    pub fn check_collision
        (w: f32, h: f32,
         mirrors: &Vec<Mirror>,
         start_pos: (f32, f32), 
         previous_mirror: Option<Mirror>,
         end_pos: (f32, f32)) -> CollisionResult 
    {
        // (CollisionResult, distance from ray origin);
        let mut results: Vec<(CollisionResult, i32)> = vec![];
        let iter_mirrors = &mut mirrors.clone();

        for i in 0..mirrors.len() {
            if previous_mirror.is_some() {
                if mirrors[i] == previous_mirror.unwrap() {
                    iter_mirrors.remove(i);   
                }
            }
        }

        //todo: 
        // instead of doing this super slow lerp iterator check on mirror collision 
        // maybe add line_intersect_poly2d() on util::geometry...
        for mirror in iter_mirrors {
            for i in 0..NUM_ITERATIONS {
                let c_pos = Self::lerp(start_pos, end_pos, i as f32 / NUM_ITERATIONS as f32);
                let x = c_pos.0;
                let y = c_pos.1;
                if mirror.in_bounds(x, y, mirror.pos.into(), w, h) {
                    results.push(
                        (CollisionResult {
                            col_type: CollisionType::Mirror,
                            mirror: Some(*mirror),
                            end_pos: (x, y)
                        }, 
                        i)
                    );
                    break;
                }
            }
        }
        
        let closest = results
            .iter()
            .min_by_key(|&(_, distance)| distance);


        match closest {
            Some(result) => {
                return result.0
            }
            None => {
                CollisionResult { 
                    col_type: CollisionType::Void, 
                    mirror: None, 
                    end_pos,
                }
            }

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

        if self.draw_list.len() > MAX_LINES {
            for _i in 0..32 {
                self.draw_list.remove(0);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CollisionType {
    Mirror,
    Void,
    Diffuse,
}

#[derive(Debug, Copy, Clone)]
pub struct CollisionResult {
    col_type: CollisionType,
    mirror: Option<Mirror>,
    end_pos: (f32, f32),
}
