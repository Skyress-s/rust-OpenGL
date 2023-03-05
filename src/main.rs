
extern crate glfw;
mod Shader;

use cgmath::num_traits::ToPrimitive;
use cgmath::SquareMatrix;
use glfw::{Action, Context, Key, WindowEvent};

const SCREEN_WIDTH : u32 = 1200u32;
const SCREEN_HEIGHT : u32 = 600u32;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    // glfw::WindowHint(gl::PROFILE)

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // setup callback
    window.set_size_polling(true);

    // loading opengl function pointers
    gl::load_with(|s| glfw.get_proc_address_raw(s));
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    // user

    type Vertex = [f32; 3];

    const VERTICES: [Vertex; 6] =
    [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0],
        [-0.5, -0.0, 0.0], [0.5, -0.0, 0.0], [0.0, 1.0, 0.0]];


    unsafe {
        // load_gl_with(|f_name| win.get_proc_address(f_name));
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);

        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // glBindBuffer(GL_ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            core::mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            core::mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
    }

    let mainShader = Shader::Shader::build(&"vertex.txt".to_string(), &"fragment.txt".to_string());
    mainShader.use_shader();
    let mut model = cgmath::Matrix4::from_value(1f32);
    mainShader.set_mat4x4_String("model".to_string(), model);
    mainShader.set_mat4x4_String("view".to_string(), model);
    mainShader.set_mat4x4_String("projection".to_string(), model);

    // Loop until the user closes the window
    'mainLoop: while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(1.0,1.0,1.0,1.0);
        }
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_glfw_events(&event, & mut window);
            println!("{:?}", event);
        }
        model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0001f32,0f32,0f32)) * model;
        mainShader.set_mat4x4_String("model".to_string(), model);

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn handle_glfw_events(events : &WindowEvent,  window : &mut glfw::Window){
    match events {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            window.set_size(SCREEN_WIDTH.to_i32().expect(""), (SCREEN_HEIGHT + 20u32).to_i32().expect(""))
        },
        glfw::WindowEvent::Size(x,y) => unsafe {
            gl::Viewport(0,0,*x,*y); // TODO mabye dereferencing isnt the best idea here?
            println!("{0} : {1}", x, y)
        },
        glfw::WindowEvent::ContentScale(x,y) =>{
            println!("{}", "YEAH")
        },

        _ => {},
    }
}

// mod shader;
//
// extern crate glutin;
// extern crate gl;
//
// use cgmath::{Matrix4, SquareMatrix, Vector3};
// use glutin::event::{Event, VirtualKeyCode, WindowEvent};
// use glutin::event_loop::{ControlFlow, EventLoop};
// use glutin::window::WindowBuilder;
// use glutin::ContextBuilder;
// use gl::types::*;
// use crate::shader::Shader;
//
// fn main() {
//     // creates the context and version
//     let event_loop = EventLoop::new();
//     let window_builder = WindowBuilder::new()
//         .with_title("OpenGL Window")
//         .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0));
//     let context_builder = ContextBuilder::new();
//     let windowed_context = context_builder
//         .build_windowed(window_builder, &event_loop)
//         .unwrap();
//
//     let windowed_context = unsafe { windowed_context.make_current().unwrap() };
//     gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
//     unsafe {
//         gl::Viewport(0, 0, 800, 600);
//     }
//     // user setup
//
//     type Vertex = [f32; 3];
//
//     const VERTICES: [Vertex; 6] =
//     [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0],
//         [-0.5, -0.0, 0.0], [0.5, -0.0, 0.0], [0.0, 1.0, 0.0]];
//
//
//     unsafe {
//         // load_gl_with(|f_name| win.get_proc_address(f_name));
//         gl::ClearColor(0.2, 0.3, 0.3, 1.0);
//
//         let mut vao = 0;
//         gl::GenVertexArrays(1, &mut vao);
//         assert_ne!(vao, 0);
//         gl::BindVertexArray(vao);
//
//         let mut vbo = 0;
//         gl::GenBuffers(1, &mut vbo);
//         assert_ne!(vbo, 0);
//
//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         // glBindBuffer(GL_ARRAY_BUFFER, vbo);
//
//         gl::BufferData(
//             gl::ARRAY_BUFFER,
//             core::mem::size_of_val(&VERTICES) as isize,
//             VERTICES.as_ptr().cast(),
//             gl::STATIC_DRAW,
//         );
//         gl::VertexAttribPointer(
//             0,
//             3,
//             gl::FLOAT,
//             gl::FALSE,
//             core::mem::size_of::<Vertex>().try_into().unwrap(),
//             0 as *const _,
//         );
//         gl::EnableVertexAttribArray(0);
//     }
//
//     let mainShader = Shader::build(&"vertex.txt".to_string(), &"fragment.txt".to_string());
//     mainShader.use_shader();
//     let mut model = cgmath::Matrix4::from_value(1f32);
//     mainShader.set_mat4x4_String("u_matrix".to_string(), model);
//
//     // Render loop
//
//     event_loop.run(move |event, _, control_flow| {
//         // polling and events
//         *control_flow = ControlFlow::Wait;
//         match event {
//             Event::WindowEvent { event, .. } => match event {
//                 WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
//                 WindowEvent::KeyboardInput { input, .. } => {
//                     if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
//                         *control_flow = ControlFlow::Exit;
//                     }
//                 }
//                 _ => (),
//             },
//             Event::LoopDestroyed => return,
//             _ => (),
//         }
//         unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) }
//
//         model = Matrix4::from_translation(Vector3::new(0.0001f32,0f32,0f32)) * model;
//         mainShader.set_mat4x4_String("u_matrix".to_string(), model);
//         unsafe {
//             gl::DrawArrays(gl::TRIANGLES, 0, 3);
//         }
//
//         windowed_context.swap_buffers().unwrap();
//     });
// }