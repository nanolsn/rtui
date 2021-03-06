use crate::common::Rect;

#[derive(Debug)]
pub struct RectRender {
    vao: u32,
    vbo: u32,
}

impl RectRender {
    pub fn new(pos_index: u32, st_index: u32) -> Self {
        let (mut vbo, mut vao) = (0, 0);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<glm::Vec4>() * 4) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::EnableVertexAttribArray(pos_index);
            gl::VertexAttribPointer(
                pos_index,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<glm::Vec4>() as i32,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(st_index);
            gl::VertexAttribPointer(
                st_index,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<glm::Vec4>() as i32,
                (2 * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        };

        RectRender { vao, vbo }
    }

    pub fn draw(&self, rect: Rect<f32>, st: Option<Rect<f32>>, flip_v: bool) {
        let st = st.unwrap_or(Rect::new((0.0, 0.0), (1.0, 1.0)));

        let (top, bot) = if flip_v {
            (st.bot(), st.top())
        } else {
            (st.top(), st.bot())
        };

        let points: [glm::Vec4; 4] = [
            glm::vec4(rect.left(), rect.top(), st.left(), top),
            glm::vec4(rect.left(), rect.bot(), st.left(), bot),
            glm::vec4(rect.right(), rect.bot(), st.right(), bot),
            glm::vec4(rect.right(), rect.top(), st.right(), top),
        ];

        self.draw_points(&points);
    }

    fn draw_points(&self, points: &[glm::Vec4; 4]) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                std::mem::size_of::<[glm::Vec4; 4]>() as isize,
                points.as_ptr() as *const std::ffi::c_void,
            );

            gl::DrawArrays(gl::TRIANGLE_FAN, 0, points.len() as i32);
        }
    }
}

impl Drop for RectRender {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
