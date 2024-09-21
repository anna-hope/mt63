use std::env::args;
use std::path::PathBuf;

use mt63::audio;

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<_>>();

    let cmd_mode = args.get(1).expect("need a subcommand");
    match cmd_mode.as_str() {
        "print-rate" => {
            let mut args = args;
            return print_rate(args.split_off(1));
        }
        "print-samples" => {
            let mut args = args;
            return print_samples(args.split_off(1));
        }
        _ => {
            println!("unknown subcommand");
        }
    }

    Ok(())
}

fn print_rate(args: Vec<String>) -> anyhow::Result<()> {
    let wav_path = args.get(1).expect("Need a path to a wave file");
    let wav_path = wav_path.parse::<PathBuf>()?;

    let audio = audio::get_audio(&wav_path)?;

    println!("{}", audio.sample_rate);

    Ok(())
}

fn print_samples(args: Vec<String>) -> anyhow::Result<()> {
    let wav_path = args.get(1).expect("Need a path to a wave file");
    let wav_path = wav_path.parse::<PathBuf>()?;

    let audio = audio::get_audio(&wav_path)?;

    for sample in audio.data {
        println!("{}", sample);
    }

    Ok(())
}
