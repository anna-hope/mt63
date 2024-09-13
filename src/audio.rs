use std::path::Path;

use fundsp::math;
use fundsp::wave::Wave;

#[derive(Debug)]
pub struct Sample {
    pub sin: f64,
    pub cos: f64,
}

#[derive(Debug)]
pub struct Audio {
    pub data: Vec<f32>,
    pub sample_rate: u32,
}

impl Audio {
    fn new(channel: Vec<f32>, sample_rate: u32) -> Self {
        Self {
            data: channel,
            sample_rate,
        }
    }
}

impl From<Wave> for Audio {
    fn from(value: Wave) -> Self {
        let sample_rate = value.sample_rate() as u32;
        Self::new(value.channel(0).clone(), sample_rate)
    }
}

pub fn get_audio(path: &Path) -> anyhow::Result<Audio> {
    let wave = Wave::load(path)?;
    Ok(Audio::from(wave))
}

pub fn generate_samples(frequency_hz: f64, sample_rate: u32, samples: usize) -> Vec<Sample> {
    let mut sin_wave = Vec::with_capacity(samples);

    for sample_idx in 0..samples {
        let sample_idx = sample_idx as f64;
        let offset = sample_idx / f64::from(sample_rate);
        let sin = math::sin_hz(frequency_hz, offset);
        let cos = math::cos_hz(frequency_hz, offset);
        let sample = Sample { sin, cos };
        sin_wave.push(sample);
    }

    sin_wave
}
