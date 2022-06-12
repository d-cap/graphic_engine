use glm::Vec3;

const YAW: f32 = -90.;
const PITCH: f32 = 0.;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.;

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::zeros(),    // initialized later
            right: Vec3::zeros(), // initialized later
            world_up: Vec3::new(0., 1., 0.),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        };
        camera.update();
        camera
    }
}

impl Camera {
    pub fn move_forward(&mut self, delta: f32) {
        self.position += self.movement_speed * delta * self.front;
    }
    pub fn move_backward(&mut self, delta: f32) {
        self.position -= self.movement_speed * delta * self.front;
    }

    pub fn move_left(&mut self, delta: f32) {
        self.position -= self.right * self.movement_speed * delta;
    }

    pub fn move_right(&mut self, delta: f32) {
        self.position += self.right * self.movement_speed * delta;
    }

    pub fn move_mouse(&mut self, mut x_offset: f32, mut y_offset: f32) {
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
        self.zoom -= y;

        if self.zoom < 1. {
            self.zoom = 1.;
        }
        if self.zoom > 75. {
            self.zoom = 75.
        }
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    fn update(&mut self) {
        self.front = glm::normalize(&glm::Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}
