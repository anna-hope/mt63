use std::borrow::Cow;
use std::ops::Sub;
use std::path::Path;
use std::time::Duration;

use fundsp::math;
use fundsp::wave::Wave;

const WINDOW_SIZE_SECONDS: f64 = 0.032;

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

#[derive(Debug)]
pub struct AudioWindow<'audio> {
    data: Cow<'audio, [f32]>,
    sample_rate: f64,
}

impl<'audio> AudioWindow<'audio> {
    fn from_audio(audio: &'audio Audio) -> Self {
        Self {
            data: Cow::from(&audio.data),
            sample_rate: audio.sample_rate,
        }
    }
}

impl<'audio> Sub<AudioWindow<'audio>> for AudioWindow<'audio> {
    type Output = Self;

    fn sub(self, rhs: AudioWindow) -> Self::Output {
        todo!()
    }
}

#[derive(Debug)]
pub struct Audio {
    pub data: Vec<f32>,
    pub sample_rate: f64,
}

impl Audio {
    fn new(data: Vec<f32>, sample_rate: f64) -> Self {
        Self { data, sample_rate }
    }

    pub fn sin_cos_at_freq(&self, freq_hz: f64) -> Vec<Sample> {
        todo!()
    }

    pub fn window_at(&self, sample_offset: usize) -> AudioWindow<'_> {
        todo!()
    }

    pub fn power_at(&self, freq_hz: f64, time_secs: f64) -> f64 {
        todo!()
    }

    pub fn shift(&self) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl From<Wave> for Audio {
    fn from(value: Wave) -> Self {
        Self::new(value.channel(0).clone(), value.sample_rate())
    }
}

pub fn get_audio(path: impl AsRef<Path>) -> anyhow::Result<Audio> {
    let wave = Wave::load(path)?;
    Ok(Audio::from(wave))
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AUDIO_PATH: &str = "data/generate-ak6ba.wav";

    #[test]
    fn can_load_audio() -> anyhow::Result<()> {
        let audio = get_audio(TEST_AUDIO_PATH)?;
        assert!(audio.len() > 0);
        Ok(())
    }
}
