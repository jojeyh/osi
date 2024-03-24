pub fn convert_to_f32(input: Vec<i16>) -> Vec<f32> {
    input.into_iter().map(|x| x as f32).collect()
}