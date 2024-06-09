use nalgebra::{Matrix4, Perspective3, Point, Point3, Unit, UnitQuaternion, Vector3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseUtil;

use sdl2::video::{Window};
use crate::graphics::program::Program;

pub struct Camera {
    gl : gl::Gl,
    pub position: Point3<f32>,
    pub orientation: Vector3<f32>,
    pub up: Vector3<f32>,

    pub width : i32,
    pub height : i32,

    pub speed : f32,
    pub sensitivity : f32,
}

impl Camera {
    pub fn new(gl : &gl::Gl, width: i32, height: i32, position : Point3<f32>) -> Camera{

        Camera{
            gl : gl.clone(),
            position,
            orientation : Vector3::new(0.0,0.0,-1.0),
            up : Vector3::new(0.0,1.0,0.0),

            width,
            height,

            speed : 0.1,
            sensitivity : 100.0,
        }
    }

    // let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    // let fov = 45.0f32.to_radians();
    // proj = Perspective3::new(aspect_ratio, fov, 0.1, 100.0).to_homogeneous();

    pub fn matrix(&self, fov_deg : f32, near_plane : f32, far_plane : f32, shader_program : &Program, uniform : &str){
        let mut view : Matrix4<f32> = Matrix4::identity();
        let mut projection : Matrix4<f32> = Matrix4::identity();

        view = Matrix4::look_at_rh(&self.position, &(self.position + self.orientation), &self.up);

        let aspect_ratio: f32 = (self.width / self.height) as f32;
        projection = Perspective3::new(aspect_ratio, fov_deg.to_radians(), near_plane, far_plane).to_homogeneous();

        unsafe {
            self.gl.UniformMatrix4fv(
                shader_program.get_uniform_id(uniform),
                1,
                gl::FALSE,
                (projection * view).as_ptr()
            );
        }
    }

    pub fn keyboard_inputs(&mut self, event : &Event){
        match event {
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                self.position += self.speed * self.orientation
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                self.position -= self.speed * self.orientation
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                let direction = Vector3::cross(&self.orientation, &self.up).normalize();
                self.position += self.speed * -direction
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                let direction = Vector3::cross(&self.orientation, &self.up).normalize();
                self.position -= self.speed * -direction
            },
            Event::KeyDown { keycode: Some(Keycode::LShift), .. } => {
                self.position += self.speed * -self.up
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                self.position += self.speed * self.up
            },
            _ => { }
        }
    }

    pub fn mouse_inputs(&mut self, event: &Event, mouse : &MouseUtil, window: &Window){
            // println!("Mouse moved to ({}, {})", x, y);
            // window.update_mouse_center(x, y);
        match event {
            Event::MouseMotion { x, y, .. } => {
                println!("{} | {}", x, y);

                let rot_x = (self.sensitivity * (y - (self.height) / 2) as f32) / self.height as f32;
                let rot_y = (self.sensitivity * (x - (self.width) / 2) as f32) / self.width as f32;

                let rotation_axis = Vector3::cross(&self.orientation, &self.up);

                let x_rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(rotation_axis), -rot_x.to_radians());
                let y_rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(self.up), -rot_y.to_radians());

                let mut new_orientation = x_rotation * self.orientation;

                new_orientation = y_rotation * new_orientation;

                self.orientation = new_orientation;

                mouse.warp_mouse_in_window(&window, self.width / 2, self.height / 2)
            }
            _ => { }
        }

    }
}