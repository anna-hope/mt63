use std::env::args;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<_>>();
    let wav_path = args.get(1).expect("Need a path to a wave file");
    let wav_path = wav_path.parse::<PathBuf>()?;

    let (channel, sample_rate) = mt63::get_first_channel_from_wav(&wav_path)?;
    let mut offset = 1_000_000;

    let num_samples = 44;
    let generated_samples = mt63::generate_samples(2500.0, sample_rate, num_samples);

    for _ in 0..10 {
        let samples = channel
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

fn augmented_hadamard_matrix() -> [u64; 128] {
    let mut matrix: [u64; 128] = [0; 128];
    for i in 0..matrix.len() {
        for j in 0..64 {
            let bits = i & j;
            if (bits.count_ones() & 0x1 == 0) == (i & 0x40 == 0) {
                matrix[i] = matrix[i] | (0x1 << (63 - j));
            }
        }
    }
    matrix
}
