use std::ops::Sub;
use std::path::Path;
use std::time::Duration;

use fundsp::read::WaveError;
use fundsp::wave::Wave;
use fundsp::{math, Float};

const WINDOW_SIZE_SECONDS: f64 = 0.032;

#[derive(Debug)]
pub struct Sample {
    pub sin: f64,
    pub cos: f64,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Mode {
    OneK,
    #[default]
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

/// An `AudioWindow` does not own its data.
#[derive(Debug, Copy, Clone)]
pub struct AudioWindow<'audio> {
    data: &'audio [f32],
    sample_rate: f64,
}

impl<'audio> AudioWindow<'audio> {
    fn new(data: &'audio [f32], sample_rate: f64) -> Self {
        Self { data, sample_rate }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_owned(&self) -> OwnedAudioWindow {
        OwnedAudioWindow {
            data: self.data.to_vec(),
            sample_rate: self.sample_rate,
        }
    }
}

impl<'audio> Sub<AudioWindow<'audio>> for AudioWindow<'audio> {
    type Output = OwnedAudioWindow;

    fn sub(self, rhs: AudioWindow) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(this, &other)| this - other)
            .collect::<Vec<_>>();
        OwnedAudioWindow {
            data,
            sample_rate: self.sample_rate,
        }
    }
}

/// An OwnedAudioWindow owns its data.
#[derive(Debug, Clone)]
pub struct OwnedAudioWindow {
    data: Vec<f32>,
    sample_rate: f64,
}

impl OwnedAudioWindow {
    pub fn as_audio_window(&self) -> AudioWindow<'_> {
        AudioWindow::new(&self.data, self.sample_rate)
    }
}

#[derive(Debug)]
pub struct Audio {
    pub data: Vec<f32>,
    pub sample_rate: f64,
    mode: Mode,
}

impl Audio {
    fn new(data: Vec<f32>, sample_rate: f64) -> Self {
        Self {
            data,
            sample_rate,
            mode: Default::default(),
        }
    }

    fn window_size(&self) -> usize {
        (self.sample_rate / self.mode.baud_rate()).floor() as usize
    }

    pub fn sin_cos_at_freq(&self, freq_hz: f64) -> Vec<Sample> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, &sample)| {
                let timestamp = (index as f64) / self.sample_rate;
                let phase = timestamp * 2.0 * f64::PI * freq_hz;
                let sin = phase.sin() * f64::from(sample);
                let cos = phase.cos() * f64::from(sample);
                Sample { sin, cos }
            })
            .collect::<Vec<_>>()
    }

    pub fn power_at(&self, freq_hz: f64, time_secs: f64) -> f64 {
        todo!()
    }

    pub fn shift(&self) -> Self {
        todo!()
    }

    pub fn window_at(&self, sample_offset: usize) -> Option<AudioWindow<'_>> {
        let data_window = self
            .data
            .get(sample_offset..sample_offset + self.window_size())?;
        Some(AudioWindow::new(data_window, self.sample_rate))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl From<Wave> for Audio {
    fn from(value: Wave) -> Self {
        Self::new(value.channel(0).clone(), value.sample_rate())
    }
}

pub fn get_audio(path: impl AsRef<Path>) -> Result<Audio, WaveError> {
    Wave::load(path).map(|w| w.into())
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
    fn window_at_offset() -> anyhow::Result<()> {
        let audio = get_audio(TEST_AUDIO_PATH)?;
        let offset = 500;
        let window = audio.window_at(offset).expect("Should get window");
        // Sample rate = 8000, baud rate = 20, thus sample rate / baud rate
        let expected_len = 400;
        assert_eq!(window.data.len(), expected_len);
        Ok(())
    }

    #[test]
    fn sin_cos_at_freq() -> anyhow::Result<()> {
        let audio = get_audio(TEST_AUDIO_PATH)?;
        let sin_cos = audio.sin_cos_at_freq(781.25);
        assert_eq!(sin_cos.len(), audio.len());
        // TODO: Actually test the values
        Ok(())
    }

    #[test]
    fn audio_window_sub() -> anyhow::Result<()> {
        let audio = get_audio(TEST_AUDIO_PATH)?;
        let offset = 500;
        let window = audio.window_at(offset).expect("Should get window");
        let new_window = window - window;
        assert!(new_window.data.iter().all(|&x| x == 0.));
        Ok(())
    }
}
