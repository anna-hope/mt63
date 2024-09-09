use std::f64::consts::PI;
use std::ops::Add;

use fundsp::wave::Wave;
use serde::Serialize;

use indicatif::ProgressIterator;

use crate::Mode;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct SampleCalculation {
    pub signal_power: f64,
    pub symbol_count: f64,
    pub sin: f64,
    pub cos: f64,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct WindowCalculation {
    pub signal_power: f64,
    pub sin: f64,
    pub cos: f64,
    pub atan: f64,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct Calculation {
    pub sample: SampleCalculation,
    pub window: WindowCalculation,
}

impl Calculation {
    fn new(sample: SampleCalculation, window: WindowCalculation) -> Self {
        Self { sample, window }
    }
}

pub fn signal_calculations(wave: &Wave, mode: Mode, start: usize) -> Option<Vec<Calculation>> {
    let relative_offset = 500;
    let absolute_offset = start + relative_offset;

    let channel_at_offset = wave
        .channel(0)
        .get(absolute_offset..)
        .expect("Channel is not long enough");
    let carrier = (relative_offset as f64) + (2000.0 / 64.0) * 21.0;
    dbg!(carrier);

    let mut sample_calculations = Vec::with_capacity(channel_at_offset.len());

    for (sample_idx, sample) in channel_at_offset.iter().copied().enumerate().progress() {
        let timestamp = (sample_idx as f64) / wave.sample_rate();
        let sin = (timestamp * 2.0 * PI * carrier).sin() * f64::from(sample);
        let cos = (timestamp * 2.0 * PI * carrier).cos() * f64::from(sample);

        let signal_power = (sin.powi(2) + cos.powi(2)).powf(0.5);
        let symbol_count = timestamp * mode.baud_rate();
        sample_calculations.push(SampleCalculation {
            sin,
            cos,
            signal_power,
            symbol_count,
        });
    }

    let window_size = (wave.sample_rate() / mode.baud_rate()).floor() as usize;
    dbg!(window_size);

    let window_calculations = sample_calculations
        .windows(window_size)
        .progress()
        .map(|window| {
            let sin_sum = window
                .iter()
                .map(|&sc| sc.sin)
                .reduce(f64::add)
                .expect("No values in window");
            let cos_sum = window
                .iter()
                .map(|&sc| sc.cos)
                .reduce(f64::add)
                .expect("No values in window");
            let signal_power = (sin_sum.powi(2) + cos_sum.powi(2)).powf(0.5);
            let window_atan = (sin_sum / cos_sum).atan();
            WindowCalculation {
                sin: sin_sum,
                cos: cos_sum,
                signal_power,
                atan: window_atan,
            }
        });

    let calculations = window_calculations
        .zip(&sample_calculations)
        .map(|(wc, sc)| Calculation::new(*sc, wc))
        .collect::<Vec<_>>();
    Some(calculations)
}
