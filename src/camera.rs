use glm::Vec3;

use crate::utils::to_radians;

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    world_up: Vec3,
    speed: f32,
    prev_mouse_x: f32,
    prev_mouse_y: f32,
    yaw: f32,
    pitch: f32,
    mouse_sensitivity: f32,
    fov: f32,
    width: f32,
    height: f32,
}

impl Camera {
    pub fn new(
        initial_position: Vec3,
        direction: Vec3,
        world_up: Vec3,
        speed: f32,
        width: f32,
        height: f32,
        mouse_sensitivity: f32,
    ) -> Self {
        let mut s = Self {
            position: initial_position,
            direction,
            up: world_up.clone(),
            world_up,
            speed,
            prev_mouse_x: width / 2.,
            prev_mouse_y: height / 2.,
            yaw: -90.,
            pitch: 0.,
            mouse_sensitivity,
            fov: 45.,
            width,
            height,
        };
        s.update();
        s
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.position += self.speed * delta * self.direction;
    }
    pub fn move_backward(&mut self, delta: f32) {
        self.position -= self.speed * delta * self.direction;
    }

    pub fn move_left(&mut self, delta: f32) {
        self.position -=
            glm::normalize(&glm::cross(&self.direction, &self.world_up)) * self.speed * delta;
    }

    pub fn move_right(&mut self, delta: f32) {
        self.position +=
            glm::normalize(&glm::cross(&self.direction, &self.world_up)) * self.speed * delta;
    }

    pub fn move_mouse(&mut self, x: f32, y: f32) {
        let mut x_offset = x as f32 - self.prev_mouse_x;
        let mut y_offset = self.prev_mouse_y - y as f32;
        self.prev_mouse_x = x as f32;
        self.prev_mouse_y = y as f32;
        x_offset *= self.mouse_sensitivity;
        y_offset *= self.mouse_sensitivity;
        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch > 89. {
            self.pitch = 89.;
        }
        if self.pitch < -89. {
            self.pitch = -89.;
        }
        self.update();
    }

    pub fn change_fov(&mut self, y: f32) {
        self.fov -= y;

        if self.fov < 1. {
            self.fov = 1.;
        }
        if self.fov > 75. {
            self.fov = 75.
        }
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.direction), &self.up)
    }

    pub fn projection_matrix(&self) -> glm::Mat4 {
        glm::perspective(self.width / self.height, to_radians(self.fov), 0.1, 100.)
    }

    fn update(&mut self) {
        self.direction = glm::normalize(&glm::Vec3::new(
            to_radians(self.yaw).cos() * to_radians(self.pitch).cos(),
            to_radians(self.pitch).sin(),
            to_radians(self.yaw).sin() * to_radians(self.pitch).cos(),
        ));
        // Saving up and right we can update them once instead of having to recalculate for every move for position
    }
}
