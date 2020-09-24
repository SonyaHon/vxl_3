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
use resource::DeltaTime;
use specs::prelude::*;

use component::{material::Material, mesh::Mesh, transform::Transform};
use system::TestSys;
use vxl_gl::gl;

const RFPS: f32 = 120.0;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("VXL")
        .with_resizable(false)
        .with_inner_size(Size::new(LogicalSize::new(1280, 720)));

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

    let mut dispatcher = dispatcher_builder.with(TestSys, "test", &[]).build();
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

            let (material, transform, mesh): (
                ReadStorage<Material>,
                ReadStorage<Transform>,
                ReadStorage<Mesh>,
            ) = world.system_data();
            for (material, transform, mesh) in (&material, &transform, &mesh).join() {
                let pid = material.get_program_id();
                gl.bind_program(pid);
                let tloc = gl.get_uniform_location(pid, "trans_mat");
                // let trans_mat = transform.get_transform_matrix();
                gl.add_uniform_matrix4f(tloc, transform.get_transform_matrix());

                gl.bind_vao(mesh.get_vao_id());
                gl.enable_vertex_attrib_arrays(vec![0]); // @todo gather this from mesh or material

                gl.draw_elements(mesh.get_vertex_count());

                gl.disable_vertex_attrib_arrays(vec![0]);
                gl.unbind_vao();
                gl.unbind_program();
            }

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
