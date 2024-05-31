use std::{error::Error, fs, path::PathBuf};

use crate::{extract, topo, Range, Seed, Size, ThemeRef};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[clap()]
    src: String,

    #[clap()]
    dst: String,

    #[clap(long, default_value_t = Default::default(), value_parser = Seed::from_arg)]
    seed: Seed,

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

impl topo::Options for Args {
    fn seed(&self) -> &Seed {
        &self.seed
    }

    fn size(&self) -> &Size {
        &self.size
    }

    fn scale_range(&self) -> std::ops::Range<f64> {
        self.scale_range.to_std()
    }

    fn line_width_range(&self) -> std::ops::Range<f64> {
        self.line_width_range.to_std()
    }

    fn theme(&self) -> &ThemeRef {
        &self.theme
    }

    fn hide_location(&self) -> bool {
        self.hide_location
    }

    fn dest(&self) -> PathBuf {
        PathBuf::from(&self.dst)
    }
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let data: extract::Data = serde_json::from_reader(&mut fs::File::open(&args.src)?)?;
    topo::render(&data, args, |theme, origin, scale, seed| {
        println!(
            "theme = {}, origin = ({:0.2}, {:0.2}), scale = {:0.2}, seed = {}",
            theme,
            origin.x(),
            origin.y(),
            scale,
            seed
        );
        Ok(())
    })
}
