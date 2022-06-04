use std::f32::consts::PI;

#[inline]
pub fn to_radians(angle: f32) -> f32 {
    angle * (PI / 180.)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ButtonState {
    pub half_transition_count: i32,
    pub ended_down: bool,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,

    pub left_bracket: ButtonState,
    pub right_bracket: ButtonState,

    pub mouse_left: ButtonState,

    pub mouse: glm::Vec2,

    pub mouse_scroll: f32,

    pub delta_time: f32,
}

impl Input {
    pub fn new(mouse_x: f32, mouse_y: f32) -> Self {
        Self {
            up: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            down: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            left: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            right: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            left_bracket: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            right_bracket: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            mouse_left: ButtonState {
                half_transition_count: 0,
                ended_down: false,
            },
            mouse: glm::vec2(mouse_x, mouse_y),
            mouse_scroll: 0.,
            delta_time: 0.,
        }
    }
}
