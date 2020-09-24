use crate::{resource::Task, vxl_gl::Gl};

pub fn render_simple<'a>(gl: &'a Gl, task_res: &'a Task) {
    let main_cam = task_res.get_main_camera_task();

    let projection_mat = main_cam.get_projection_mat();
    let view_mat = main_cam.get_view_mat();

    let render_tasks = task_res.get_render_tasks();
    for render_task in render_tasks {
        let pid = render_task.get_pid();
        let attrib_arrays = render_task.get_attri_arrays();

        gl.bind_program(pid);

        let ploc = gl.get_uniform_location(pid, "proj_mat");
        gl.add_uniform_matrix4f(ploc, projection_mat);
        let vloc = gl.get_uniform_location(pid, "view_mat");
        gl.add_uniform_matrix4f(vloc, view_mat);

        for (name, value) in render_task.get_mat4f_unifroms() {
            gl.add_uniform_matrix4f(gl.get_uniform_location(pid, *name), *value);
        }

        gl.bind_vao(render_task.get_vao_id());
        gl.enable_vertex_attrib_arrays(attrib_arrays);
        gl.draw_elements(render_task.get_vertex_count());
        gl.disable_vertex_attrib_arrays(attrib_arrays);
        gl.unbind_vao();
        gl.unbind_program();
    }
}
