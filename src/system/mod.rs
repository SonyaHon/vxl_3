use specs::prelude::*;

use crate::{
    component::camera::Camera, component::camera::MainCamera, component::material::Material,
    component::mesh::Mesh, component::transform::Transform, resource::tasks::MainCameraTask,
    resource::tasks::RenderTask, resource::Task,
};
pub struct SetMainCameraSys;
impl<'a> System<'a> for SetMainCameraSys {
    type SystemData = (
        ReadStorage<'a, MainCamera>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        WriteExpect<'a, Task>,
    );

    fn run(&mut self, (main_camera, camera, transform, mut task): Self::SystemData) {
        let (_, camera, transform) = (&main_camera, &camera, &transform).join().next().unwrap();

        let projection_mat = camera.get_projection_matrix();
        let view_mat = transform.get_view_matrix();
        task.set_main_camera_task(MainCameraTask::new(projection_mat, view_mat));
    }
}

pub struct SetRenderTaskSys;
impl<'a> System<'a> for SetRenderTaskSys {
    type SystemData = (
        ReadStorage<'a, Material>,
        ReadStorage<'a, Mesh>,
        ReadStorage<'a, Transform>,
        WriteExpect<'a, Task>,
    );

    fn run(&mut self, (material, mesh, transform, mut task): Self::SystemData) {
        for (material, mesh, transform) in (&material, &mesh, &transform).join() {
            let program_id = material.get_program_id();
            let vao_id = mesh.get_vao_id();
            let vertex_count = mesh.get_vertex_count();
            let attrib_arrays = vec![0];
            let mat4f_uniforms = vec![("trans_mat", transform.get_transform_matrix())];
            task.push_render_task(RenderTask::new(
                program_id,
                vao_id,
                vertex_count,
                attrib_arrays,
                mat4f_uniforms,
            ));
        }
    }
}
