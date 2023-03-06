use std;
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::BufRead;
use cgmath::Matrix;
use gl;

#[derive(Debug, Eq, PartialEq)]
pub struct Shader {
    shader_program: gl::types::GLuint,
}

#[derive(Debug)]
enum ShaderType{
    Vertex, Geometry, Fragment
}


impl Shader {
    fn check_shader_compile_errors(id: gl::types::GLuint, shader_type: ShaderType, file_path: &String) {
        unsafe {
            let mut success = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                    id,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("{0:?} Compile Error: {1}", shader_type, String::from_utf8_lossy(&v));
            }
        }
    }

    fn check_program_compile_errors(id: gl::types::GLuint){
        let mut success = 0;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(
                    id,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }

    // constructor
    pub fn build(vertex_file_path: &String, fragment_file_path: &String) -> Self {
        let mut inst = Self {
            shader_program: 0
        };

        let vert_shader_string = fs::read_to_string(vertex_file_path).expect(
            "Should have been able to read file");
        // println!("{}", vert_shader_string);
        let frag_shader_string = fs::read_to_string(fragment_file_path).expect(
            "Should have been able to read file");
        // println!("{}", frag_shader_string);

        unsafe {

            // VERTEX SHADER
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vert_shader_string.as_bytes().as_ptr().cast()),
                &(vert_shader_string.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex_shader);
            Self::check_shader_compile_errors(vertex_shader, ShaderType::Vertex, vertex_file_path);

            // FRAGMENT SHADER
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(frag_shader_string.as_bytes().as_ptr().cast()),
                &(frag_shader_string.len().try_into().unwrap()),
            );
            gl::CompileShader(fragment_shader);
            Self::check_shader_compile_errors(fragment_shader, ShaderType::Fragment, fragment_file_path);

            // PROGRAM
            inst.shader_program = gl::CreateProgram();
            // let shader_program = glCreateProgram();
            assert_ne!(inst.shader_program, 0);
            gl::AttachShader(inst.shader_program, vertex_shader);
            gl::AttachShader(inst.shader_program, fragment_shader);
            gl::LinkProgram(inst.shader_program);
            Self::check_program_compile_errors(inst.shader_program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            // Self.use_shader();
            // glUseProgram(inst.shader_program);
        }
        inst
    }

    pub fn print_info(&self){
        println!("{}", "test!");
    }
}


impl Shader {
    pub fn use_shader(&self){
        unsafe{
            gl::UseProgram(self.shader_program);
        }
    }

    pub fn set_mat4x4(&self,name : CString, mat4x4: cgmath::Matrix4<f32>){
        unsafe{
            let location = gl::GetUniformLocation(self.shader_program, name.as_bytes_with_nul().as_ptr() as *const i8);
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat4x4.as_ptr() as *const f32)
        }
    }

    pub fn set_mat4x4_String(&self, name : String, mat4x4 : cgmath::Matrix4<f32>){
        let name2 = CString::new(name).expect("CString::Failed");
        Self::set_mat4x4(&self, name2, mat4x4);
        // let location = glGetUniformLocation(self.shader_program, name.as_bytes_with_nul().as_ptr() as *const i8);
        // glUniformMatrix4fv(location, 1, GL_FALSE, mat4x4.as_ptr() as *const f32)
    }

    pub fn set_mat4x4_glm(&self, name : String, mat4x4: nalgebra_glm::Mat4x4){
        unsafe {
            let name2 = CString::new(name).expect("Cstring::Failed");
            let location = gl::GetUniformLocation(self.shader_program, name2.as_bytes_with_nul().as_ptr() as *const i8);
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat4x4.as_ptr() as *const f32)
        }
    }
}

impl Drop for Shader{
    fn drop(&mut self) {
        unsafe{
            gl::DeleteProgram(self.shader_program);
        }
    }
}