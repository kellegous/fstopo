use std::{error::Error, fs};

use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use rand::Rng;

use crate::{extract, geo, Point, Range, Seed, Size, ThemeRef};

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

    #[clap(long, default_value_t=ThemeRef::from_path("themes.bin"), value_parser=ThemeRef::from_arg)]
    theme: ThemeRef,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let extract::Data {
        size,
        region: _region,
        mut paths,
    } = serde_json::from_reader(&mut fs::File::open(&args.src)?)?;

    let mut rng = args.seed.rng();

    let tx = rng.gen_range(0.0..size.width() - args.size.width());
    let ty = rng.gen_range(0.0..size.height() - args.size.height());
    let scale = rng.gen_range(args.scale_range.to_std().clone());
    let (theme, colors) = args.theme.pick(&mut rng)?;

    println!(
        "tx = {}, ty = {}, scale = {}, theme = {}",
        tx, ty, scale, theme
    );

    paths.iter_mut().for_each(|path| {
        path.transform(|p| Point::from_xy((p.x() - tx) * scale, (p.y() - ty) * scale));
    });

    // fix this
    let location = geo::LatLng::new(
        lerp(&(35.64643..35.48879), tx / size.width()),
        lerp(&(-80.04998..-79.85005), ty / size.height()),
    );

    let surface = ImageSurface::create(
        Format::ARgb32,
        args.size.width() as i32,
        args.size.height() as i32,
    )?;
    let ctx = Context::new(&surface)?;

    colors[0].set(&ctx);
    ctx.rectangle(0.0, 0.0, args.size.width(), args.size.height());
    ctx.fill()?;

    ctx.save()?;
    let lw = 2.0 * inv_lerp(args.scale_range.to_std(), scale);
    ctx.set_line_width(lw);
    for path in paths.iter() {
        ctx.new_path();
        path.draw(&ctx);
        colors[1].set(&ctx);
        ctx.stroke()?;
    }
    ctx.restore()?;

    ctx.save()?;
    let label = format!("{}", location);
    ctx.select_font_face("Helvetica Neue", FontSlant::Normal, FontWeight::Normal);
    ctx.set_font_size(24.0);
    let exts = ctx.text_extents(&label)?;
    colors[0].set(&ctx);
    ctx.rectangle(
        args.size.width() - exts.width() - 30.0,
        args.size.height() + exts.y_bearing() - 30.0,
        exts.width() + 20.0,
        exts.height() + 20.0,
    );
    ctx.fill()?;

    colors[1].set(&ctx);
    ctx.move_to(
        args.size.width() - exts.width() - 20.0,
        args.size.height() - 20.0,
    );
    ctx.show_text(&label)?;
    ctx.fill()?;
    ctx.restore()?;

    surface.write_to_png(&mut fs::File::create(&args.dst)?)?;
    Ok(())
}

fn lerp(r: &std::ops::Range<f64>, v: f64) -> f64 {
    r.start * (1.0 - v) + r.end * v
}

fn inv_lerp(r: &std::ops::Range<f64>, v: f64) -> f64 {
    (v - r.start) / (r.end - r.start)
}
