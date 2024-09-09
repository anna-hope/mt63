use std::env::args;
use std::path::{Path, PathBuf};

use fundsp::wave::Wave;

#[cfg(feature = "analysis")]
use mt63::analysis::{signal_calculations, Calculation};
use mt63::Mode;

#[cfg(feature = "analysis")]
fn write_csv(calculations: Vec<Calculation>, path: &Path) -> anyhow::Result<()> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    writer.write_record(&[
        "sample_signal_power",
        "sample_symbol_count",
        "sample_sin",
        "sample_cos",
        "window_signal_power",
        "window_sin",
        "window_cos",
        "window_atan",
    ])?;
    for calculation in calculations {
        writer.serialize(calculation)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<_>>();
    let wav_path = args
        .get(1)
        .expect("Need a path to a wave file")
        .parse::<PathBuf>()?;

    let output_path = args
        .get(2)
        .expect("Need output CSV path")
        .parse::<PathBuf>()?;

    let wave = Wave::load(&wav_path)?;

    let mode = Mode::TwoK;

    let start = 1_000_000;

    #[cfg(feature = "analysis")]
    {
        println!("Doing calculations (per sample and per window)...");
        let calculations = signal_calculations(&wave, mode, start).expect("Got no calculations");
        write_csv(calculations, &output_path)?;
    }

    Ok(())
}
