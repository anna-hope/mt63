use std::env::args;
use std::path::PathBuf;
use mt63::audio;

fn main() -> anyhow::Result<()> {
    let args = args().collect::<Vec<_>>();
    
    let (program_name, args) = args.split_first().ok_or_else(|| anyhow::anyhow!("Need arguments"))?;
    
    if args.len() < 2 {
        return Err(anyhow::anyhow!("Usage: {program_name} [subcommand] [wav_file]"));
    }
    
    // Unwrap will always work here since we know args.len >= 2
    let cmd_mode = args.first().unwrap();
    let wav_path = args.get(1).unwrap();
    
    match cmd_mode.as_str() {
        "print-rate" => {
            print_rate(wav_path)
        }
        "print-samples" => {
            print_samples(wav_path)
        }
        _ => {
            Err(anyhow::anyhow!("unknown subcommand: {}", cmd_mode))
        }
    }
}

fn print_rate(wav_path: &str) -> anyhow::Result<()> {
    let wav_path = wav_path.parse::<PathBuf>()?;
    let audio = audio::get_audio(&wav_path)?;

    println!("{}", audio.sample_rate);

    Ok(())
}

fn print_samples(wav_path: &str) -> anyhow::Result<()> {
    let wav_path = wav_path.parse::<PathBuf>()?;
    let audio = audio::get_audio(&wav_path)?;

    for sample in audio.data {
        println!("{}", sample);
    }

    Ok(())
}
