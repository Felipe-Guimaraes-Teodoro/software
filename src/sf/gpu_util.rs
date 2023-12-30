use gl::{*, types::*};
use std::{mem, ptr, ffi::c_void};

pub struct Buffer {
    pub id: u32,
}

impl Buffer {
    pub unsafe fn new_vao(vertices: &Vec<f32>) -> Self {
        let mut vao = 0;
        
        GenVertexArrays(1, &mut vao);

        BindVertexArray(vao);

        VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
            (3 * mem::size_of::<GLfloat>()) as GLsizei, 
            ptr::null()
        );
        EnableVertexAttribArray(0);
        BindBuffer(ARRAY_BUFFER, 0);
        BindVertexArray(0);

        Self {
            id: vao,
        }
    }

    pub unsafe fn update_vao(&mut self, vertices: &Vec<f32>) {
        BindVertexArray(self.id);
        BindBuffer(ARRAY_BUFFER, self.id);

        BufferSubData(
            ARRAY_BUFFER, 
            0, 
            (vertices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
        );

        BindVertexArray(0);
        BindBuffer(ARRAY_BUFFER, 0);
    }
}

