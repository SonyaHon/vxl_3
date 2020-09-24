use cgmath::prelude::*;
use tasks::{MainCameraTask, RenderTask};

pub mod tasks;

pub struct DeltaTime {
    raw_time: u32,
    delta: f32,
}

impl DeltaTime {
    pub fn new(raw_time: u32) -> Self {
        DeltaTime {
            raw_time,
            delta: raw_time as f32 / 16700.0,
        }
    }

    pub fn get_raw(&self) -> u32 {
        self.raw_time
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }
}

pub struct Task {
    render: Vec<RenderTask>,
    main_camera: MainCameraTask,
}
impl Task {
    pub fn push_render_task(&mut self, task: RenderTask) {
        self.render.push(task);
    }

    pub fn get_render_tasks(&self) -> &Vec<RenderTask> {
        &self.render
    }

    pub fn set_main_camera_task(&mut self, task: MainCameraTask) {
        self.main_camera = task
    }

    pub fn get_main_camera_task(&self) -> &MainCameraTask {
        &self.main_camera
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            render: vec![],
            main_camera: MainCameraTask::new(
                cgmath::Matrix4::identity(),
                cgmath::Matrix4::identity(),
            ),
        }
    }
}
