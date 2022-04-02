use glm::vec3;
use nalgebra_glm as glm;
use nalgebra_glm::Vec3;

/// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Clone, Copy)]
pub enum MovementEvent {
    Forward,
    Backward,
    Left,
    Right,
}

// Default camera values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    fps_mode: bool,
    custom_look_at: bool,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub world_up: Vec3,
    pub right: Vec3,
    // euler angles
    pub yaw: f32,
    pub pitch: f32,
    // camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let mut cam = Self {
            fps_mode: false,
            custom_look_at: false,
            position: Vec3::zeros(),
            front: vec3(0.0, 0.0, -1.0),
            up: Vec3::zeros(), // initialized later
            world_up: vec3(0.0, 1.0, 0.0),
            right: Vec3::zeros(), // initialized later
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM,
        };
        cam.update_camera_vectors();
        cam
    }
}

impl Camera {
    pub fn new(position: Vec3, fps_mode: bool) -> Camera {
        let mut cam = Self {
            position,
            fps_mode,
            ..Default::default()
        };
        cam.update_camera_vectors();
        cam
    }

    pub fn process_keyboard(&mut self, movement: MovementEvent, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        match movement {
            MovementEvent::Forward => self.position += self.front * velocity,
            MovementEvent::Backward => self.position -= self.front * velocity,
            MovementEvent::Left => self.position -= self.right * velocity,
            MovementEvent::Right => self.position += self.right * velocity,
        }
        if self.fps_mode {
            self.position[1] = 0.0;
        }
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        if !self.custom_look_at {
            return glm::look_at(&self.position, &(self.position + self.front), &self.up);
        }
        let target = self.front + self.position;
        let direction = (self.position - target).normalize();
        let right = glm::cross(&self.up, &direction).normalize();
        let true_up = glm::cross(&direction, &right).normalize();

        let base = glm::Mat4::new(
            right[0],
            right[1],
            right[2],
            0.0,
            true_up[0],
            true_up[1],
            true_up[2],
            0.0,
            direction[0],
            direction[1],
            direction[2],
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let trans = glm::Mat4::identity();
        base * glm::translate(&trans, &(-self.position))
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_camera_vectors();
    }
    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.zoom = (self.zoom - y_offset).clamp(1.0, 45.0);
    }

    /// Calculate the front vector from the Camera's (updated) euler angles
    fn update_camera_vectors(&mut self) {
        // Calculate the new front vector
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );

        self.front = front.normalize();
        self.right = self.front.cross(&self.world_up);
        self.up = self.right.cross(&self.front);
    }
}
