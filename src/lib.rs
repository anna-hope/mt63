use std::path::Path;
use std::time::Duration;

use fundsp::math;
use fundsp::wave::Wave;

#[cfg(feature = "analysis")]
pub mod analysis;

#[derive(Debug)]
pub struct Sample {
    pub sin: f64,
    pub cos: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    OneK,
    TwoK,
}

impl Mode {
    pub fn baud_rate(&self) -> f64 {
        match self {
            Self::OneK => 10.0,
            Self::TwoK => 20.0,
        }
    }

    pub fn symbol_interval(&self) -> Duration {
        Duration::from_secs(1).div_f64(self.baud_rate())
    }
}

pub fn get_first_channel_from_wav(path: &Path) -> anyhow::Result<(Vec<f32>, f64)> {
    let wave = Wave::load(path)?;
    let channels = wave.channels();
    let sample_rate = wave.sample_rate();
    println!("channels: {channels} sample rate: {sample_rate}");
    let channel = wave.channel(0);

    Ok((channel.clone(), sample_rate))
}

pub fn generate_samples(frequency_hz: f64, sample_rate: f64, samples: usize) -> Vec<Sample> {
    let mut sin_wave = Vec::with_capacity(samples);

    for sample_idx in 0..samples {
        let sample_idx = sample_idx as f64;
        let offset = sample_idx / sample_rate;
        let sin = math::sin_hz(frequency_hz, offset);
        let cos = math::cos_hz(frequency_hz, offset);
        let sample = Sample { sin, cos };
        sin_wave.push(sample);
    }

    sin_wave
}
