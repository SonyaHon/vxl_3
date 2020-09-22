extern crate cgmath;
extern crate glutin;
extern crate rand;
extern crate specs;

use cgmath::vec3;
use glutin::{
    dpi::LogicalSize, dpi::Size, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};
use specs::prelude::*;
use system::TestSystem;

mod component;
mod resource;
mod system;
mod vxl_gl;

use component::mesh::Mesh;

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

    let dispatcher_builder = specs::DispatcherBuilder::new().with(TestSystem, "test_sys", &[]);

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

        windowed_context.swap_buffers().unwrap();
    });
}
