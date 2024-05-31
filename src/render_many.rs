use std::{error::Error, fs, path::PathBuf};

use rand::Rng;

use crate::{extract, topo, Range, Seed, Size, ThemeRef};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[clap()]
    src: String,

    #[clap()]
    dst_dir: String,

    #[clap(long, default_value_t = 10)]
    n: usize,

    #[clap(long, default_value_t = Size::new(1600.0,600.0), value_parser = Size::from_arg)]
    size: Size,

    #[clap(long, value_parser=Range::from_arg, default_value_t=Range::from(1.0..8.0))]
    scale_range: Range,

    #[clap(long, value_parser=Range::from_arg, default_value_t=Range::from(2.0..4.0))]
    line_width_range: Range,

    #[clap(long, default_value_t=ThemeRef::from_path("themes.bin"), value_parser=ThemeRef::from_arg)]
    theme: ThemeRef,

    #[clap(long)]
    hide_location: bool,
}

struct Options<'a> {
    args: &'a Args,
    seed: Seed,
    dest: PathBuf,
}

impl<'a> topo::Options for Options<'a> {
    fn seed(&self) -> &Seed {
        &self.seed
    }

    fn size(&self) -> &Size {
        &self.args.size
    }

    fn scale_range(&self) -> std::ops::Range<f64> {
        self.args.scale_range.to_std()
    }

    fn line_width_range(&self) -> std::ops::Range<f64> {
        self.args.line_width_range.to_std()
    }

    fn theme(&self) -> &ThemeRef {
        &self.args.theme
    }

    fn hide_location(&self) -> bool {
        self.args.hide_location
    }

    fn dest(&self) -> PathBuf {
        self.dest.clone()
    }
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let data: extract::Data = serde_json::from_reader(&mut fs::File::open(&args.src)?)?;

    let dst = PathBuf::from(&args.dst_dir);
    if !dst.exists() {
        fs::create_dir_all(&dst)?;
    }

    let mut rng = Seed::default().rng();
    for _ in 0..args.n {
        let seed = Seed::new(rng.gen::<u64>());
        let options = Options {
            args,
            seed,
            dest: dst.join(format!("{}.png", seed)),
        };
        topo::render(&data, &options)?;
    }

    Ok(())
}
