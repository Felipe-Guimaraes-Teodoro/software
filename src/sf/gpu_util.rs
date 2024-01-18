use gl::{*, types::*};
use std::{mem, ptr, ffi::c_void};

pub trait Buffer<D> {
    fn new(data: D) -> Self;
    fn update(&mut self, new_verts: D);
    fn clear(&mut self);
}

#[derive(Copy,Clone)]
pub struct RVertexBuffer {
    pub vao_id: u32,
    pub vbo_id: u32,
}

#[derive(Copy,Clone,Debug)]
pub struct RVertexBufferIndexed {
    pub vao_id: u32,
    pub vbo_id: u32,
    pub ebo_id: u32,
}

#[derive(Copy,Clone,Debug)]
pub struct RVertexBufferTextured {
    pub vao_id:     u32,
    pub vbo_id:     u32,
    pub ebo_id:     u32,
    pub texture_id: u32,
}

impl Buffer<&Vec<f32>> for RVertexBuffer {
    fn new(data: &Vec<f32>) -> Self {
        let (mut vbo, mut vao) = (0, 0);

        unsafe {
            GenVertexArrays(1, &mut vao);
            GenBuffers(1, &mut vbo);
            BindVertexArray(vao);

            BindBuffer(gl::ARRAY_BUFFER, vbo);
            BufferData(gl::ARRAY_BUFFER,
                           (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &data[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);

            VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            EnableVertexAttribArray(0);

            BindBuffer(gl::ARRAY_BUFFER, 0);

            BindVertexArray(0);
        }

        Self {
            vao_id: vao,
            vbo_id: vbo,
        }
    }

    fn update(&mut self, new_verts: &Vec<f32>) {
        unsafe {
            BindVertexArray(self.vao_id);
            BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);

            BufferSubData(gl::ARRAY_BUFFER,
                          0,
                          (new_verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                          &new_verts[0] as *const f32 as *const c_void);

            BindBuffer(gl::ARRAY_BUFFER, 0);
            BindVertexArray(0);
        }
    }

    fn clear(&mut self) {
        unsafe {
            DeleteBuffers(1, &self.vbo_id);
            DeleteVertexArrays(1, &self.vao_id);

            self.vbo_id = 0;
            self.vao_id = 0;
        }
    }
}

// index 0 is vertices, index 1 is indices
impl Buffer<(&Vec<f32>, &Vec<i32>)> for RVertexBufferIndexed {
    fn new(data: (&Vec<f32>, &Vec<i32>)) -> Self {
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        unsafe {
            GenVertexArrays(1, &mut vao);
            GenBuffers(1, &mut vbo);
            GenBuffers(1, &mut ebo);

            BindVertexArray(vao);

            BindBuffer(gl::ARRAY_BUFFER, vbo);
            BufferData(gl::ARRAY_BUFFER,
                           (data.0.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &data.0[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);

            BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
            BufferData(ELEMENT_ARRAY_BUFFER,
                           (data.1.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &data.1[0] as *const i32 as *const c_void,
                           gl::STATIC_DRAW);


            VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            EnableVertexAttribArray(0);

            // BindBuffer(gl::ARRAY_BUFFER, 0);

            BindVertexArray(0);
        }

        Self {
            vao_id: vao,
            vbo_id: vbo,
            ebo_id: ebo,
        }
    }

    fn update(&mut self, new_data: (&Vec<f32>, &Vec<i32>)) {
        unsafe {
            BindVertexArray(self.vao_id);
            BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);

            BufferSubData(gl::ARRAY_BUFFER,
                          0,
                          (new_data.0.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                          &new_data.0[0] as *const f32 as *const c_void);

            BindBuffer(gl::ARRAY_BUFFER, 0);

            BindBuffer(ELEMENT_ARRAY_BUFFER, self.ebo_id);
            BufferSubData(gl::ELEMENT_ARRAY_BUFFER,
                          0,
                          (new_data.1.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                          &new_data.1[0] as *const i32 as *const c_void);

            BindBuffer(ELEMENT_ARRAY_BUFFER, 0);

            BindVertexArray(0);
        }
    }

    fn clear(&mut self) {
        unsafe {
            DeleteBuffers(1, &self.vbo_id);
            DeleteVertexArrays(1, &self.vao_id);

            self.vbo_id = 0;
            self.vao_id = 0;
        }
    }
}

impl Buffer<(&Vec<f32>, &Vec<i32>, &[u32])> for RVertexBufferTextured {
    fn new(data: (&Vec<f32>, &Vec<i32>, &[u32])) -> Self {
        todo!();
    }

    fn update(&mut self, new_verts: (&Vec<f32>, &Vec<i32>, &[u32])) {
        todo!();
    }

    fn clear(&mut self) {
        todo!();
    }
} 

#[macro_export]
macro_rules! cstr {
    ($str: expr) => {
        &std::ffi::CString::new($str).unwrap()
    }
}
