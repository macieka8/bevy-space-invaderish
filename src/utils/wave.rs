pub fn square_wave(input: f32) -> f32 {
    if input > 0.5 {
        1.0
    } else {
        -1.0
    }
}

pub fn saw_wave(input: f32) -> f32 {
    (((input + 0.5) % 1.0) - 0.5) * 2.0
}