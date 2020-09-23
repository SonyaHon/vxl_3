extern crate cgmath;
extern crate glutin;
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
use specs::prelude::*;

use component::{material::Material, mesh::Mesh, transform::Transform};

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("VXL")
        .with_resizable(false)
        .with_inner_size(Size::new(LogicalSize::new(1280, 720)));

    let windowed_context = ContextBuilder::new()
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
        .with(Material::default())
        .build();

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut world);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        };

        gl.clear_screen();

        dispatcher.dispatch(&world);
        world.maintain();

        let (material, mesh): (ReadStorage<Material>, ReadStorage<Mesh>) = world.system_data();

        for (_material, mesh) in (&material, &mesh).join() {
            println!("Mesh VAO id is: {}", mesh.get_vao_id());
        }

        windowed_context.swap_buffers().unwrap();
    });
}
