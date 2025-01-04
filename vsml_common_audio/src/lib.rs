#[derive(Debug)]
pub struct Audio {
    pub samples: Vec<[f32; 2]>,
    pub sampling_rate: u32,
}
