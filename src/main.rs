extern crate cgmath;
extern crate glutin;
extern crate image;
extern crate rand;
extern crate specs;

mod component;
mod loader;
mod render_functions;
mod resource;
mod system;
mod utils;
mod vxl_gl;

use cgmath::vec3;
use glutin::{
    dpi::LogicalSize, dpi::Size, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};
use resource::{DeltaTime, Task};
use specs::prelude::*;

use component::{
    camera::Camera, camera::MainCamera, material::Material, mesh::Mesh, transform::Transform,
};
use system::{SetMainCameraSys, SetRenderTaskSys};
use vxl_gl::gl;

const RFPS: f32 = 120.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_WIDTH: f32 = 1280.0;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("VXL")
        .with_resizable(false)
        .with_inner_size(Size::new(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT)));

    let windowed_context = ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .with_multisampling(4)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let mut gl = vxl_gl::load(&windowed_context.context());
    gl.clear_color = vec3(0.1, 0.1, 0.1);

    println!(
        "Pixel format of context is {:?}",
        windowed_context.get_pixel_format()
    );

    let mut world = specs::World::new();
    let shader_manager = loader::shaders::ShaderLoader::new(&gl)
        .add_shader_program(
            "default",
            vec![
                ("default/default.vert.glsl", gl::VERTEX_SHADER),
                ("default/default.frag.glsl", gl::FRAGMENT_SHADER),
            ],
        )
        .finish();

    world.register::<Mesh>();
    world.register::<Material>();
    world.register::<Transform>();
    world.register::<Camera>();
    world.register::<MainCamera>();

    let dispatcher_builder = specs::DispatcherBuilder::new();

    world
        .create_entity()
        .with(Mesh::from_data(
            &gl,
            vec![
                cgmath::vec3(0.0, 0.5, 0.0),
                cgmath::vec3(-0.5, -0.5, 0.0),
                cgmath::vec3(0.5, -0.5, 0.0),
            ],
            vec![0, 1, 2],
        ))
        .with(Transform::default())
        .with(Material::default(&shader_manager))
        .build();

    world
        .create_entity()
        .with(Transform::from_position(cgmath::vec3(0.0, 0.0, 2.0)))
        .with(Camera::new(
            45.0,
            WINDOW_WIDTH / WINDOW_HEIGHT,
            0.01,
            1000.0,
        ))
        .with(MainCamera)
        .build();

    let mut dispatcher = dispatcher_builder
        .with(SetMainCameraSys, "main_camera", &[])
        .with(SetRenderTaskSys, "render_task", &[])
        .build();
    dispatcher.setup(&mut world);

    let rfps_barrier = 1000000.0 / RFPS;
    let mut timer = std::time::Duration::new(0, 0);
    let mut diff_timer = std::time::Duration::new(0, 0);
    let mut second_timer = std::time::Duration::new(0, 0);

    let mut rfps = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let timer_start = std::time::Instant::now();
        world.insert(DeltaTime::new(diff_timer.as_micros() as u32));
        world.insert(Task::default());

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        };

        dispatcher.dispatch(&world);
        world.maintain();

        if (timer.as_micros() as f32) >= rfps_barrier {
            gl.clear_screen();

            let task_res = world.read_resource::<Task>();
            render_functions::render_simple(&gl, &task_res);

            windowed_context.swap_buffers().unwrap();

            gl.print_error();
            timer = std::time::Duration::new(0, 0);
            rfps += 1;
        }

        let elapsed = timer_start.elapsed();
        if (timer.as_micros() as f32) < rfps_barrier {
            timer = timer.checked_add(elapsed).unwrap();
        }
        diff_timer = elapsed;
        second_timer = second_timer.checked_add(elapsed).unwrap();

        if second_timer.as_secs() >= 1 {
            println!("RFPS: {}", rfps);
            rfps = 0;
            second_timer = std::time::Duration::new(0, 0);
        }
    });
}
