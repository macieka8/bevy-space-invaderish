pub fn square_wave(input: f32) -> f32 {
    if input % 1.0 > 0.5 {
        1.0
    } else {
        -1.0
    }
}

pub fn saw_wave(input: f32) -> f32 {
    (((input + 0.5) % 1.0) - 0.5) * 2.0
}

pub fn normalized_sin(input: f32) -> f32 {
    f32::sin(2.0 * std::f32::consts::PI * input)
}

/// linearlu go from 1 to 0 in input range 0.9s - 1s
pub fn fade_out(input: f32) -> f32 {
    if input > 0.9 {
        1.0 - (input - 0.9) / 0.1
    } else {
        1.0
    }
}
