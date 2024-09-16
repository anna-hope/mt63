use std::env::args;
use std::path::PathBuf;

use mt63::audio;

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<_>>();
    let wav_path = args.get(1).expect("Need a path to a wave file");
    let wav_path = wav_path.parse::<PathBuf>()?;

    let audio = audio::get_audio(&wav_path)?;
    let mut offset = 1_000_000;

    let num_samples = 44;
    let generated_samples = audio::generate_samples(2500.0, audio.sample_rate, num_samples);

    for _ in 0..10 {
        let samples = audio
            .data
            .iter()
            .skip(offset)
            .take(num_samples)
            .copied()
            .collect::<Vec<_>>();
        let (sin, cos) = samples
            .iter()
            .zip(generated_samples.iter())
            .map(|(audio_sample, generated_sample)| {
                let audio_sample = f64::from(*audio_sample);
                (
                    f64::from(audio_sample) * generated_sample.sin,
                    f64::from(audio_sample) * generated_sample.cos,
                )
            })
            .reduce(|(first_sin, second_sin), (first_cos, second_cos)| {
                (first_sin + second_sin, first_cos + second_cos)
            })
            .expect("Got no samples");
        let result = sin.powi(2) + cos.powi(2);
        dbg!(result);
        offset += samples.len();
    }
    // dbg!(&sin_wave);
    Ok(())
}
