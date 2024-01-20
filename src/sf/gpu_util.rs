use gl::{*, types::*};
use std::{mem, ptr, ffi::c_void};
use std::path::Path;

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

impl Buffer<(&Vec<f32>, &Vec<i32>, &str)> for RVertexBufferTextured {
    fn new(data: (&Vec<f32>, &Vec<i32>, &str)) -> Self {
        let (mut VBO, mut VAO, mut EBO, mut TEX_ID) = (0, 0, 0, 0);

        unsafe {

        GenVertexArrays(1, &mut VAO);
        GenBuffers(1, &mut VBO);
        GenBuffers(1, &mut EBO);

        BindVertexArray(VAO);

        BindBuffer(gl::ARRAY_BUFFER, VBO);
        BufferData(gl::ARRAY_BUFFER,
                       (data.0.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &data.0[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (data.1.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &data.1[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        EnableVertexAttribArray(0);
        // color attribute
        VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        EnableVertexAttribArray(1);
        // texture coord attribute
        VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        EnableVertexAttribArray(2);

        GenTextures(1, &mut TEX_ID);
        BindTexture(TEXTURE_2D, TEX_ID); 
        TexParameteri(TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); 
        TexParameteri(TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        TexParameteri(TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        TexParameteri(TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        let img = image::open(&Path::new(data.2)).expect("failed to load image");
        let data = img.as_bytes().to_vec();

        TexImage2D(gl::TEXTURE_2D,
           0,
           RGB as i32,
           img.width() as i32,
           img.height() as i32,
           0,
           RGB,
           UNSIGNED_BYTE,
           &data[0] as *const u8 as *const c_void);
        GenerateMipmap(TEXTURE_2D);

        } // unsafe 

        Self {
            vao_id: VAO,
            vbo_id: VBO,
            ebo_id: EBO,
            texture_id: TEX_ID, 
        }
    }

    fn update(&mut self, new_data: (&Vec<f32>, &Vec<i32>, &str)) {
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
