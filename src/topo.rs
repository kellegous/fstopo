use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use rand::{Rng, RngCore};

use crate::{extract, geo, Color, Point, Seed, Size, ThemeRef};

pub fn render<O>(data: &extract::Data, opts: &O) -> Result<(), Box<dyn Error>>
where
    O: Options,
{
    let extract::Data {
        size,
        region,
        paths,
    } = data;

    let mut rng = opts.seed().rng();

    let tx = rng.gen_range(0.0..size.width() - opts.size().width());
    let ty = rng.gen_range(0.0..size.height() - opts.size().height());
    let scale = rng.gen_range(opts.scale_range());
    let (theme, colors) = opts.theme().pick(&mut rng)?;

    let (bg, fg) = select_color_pair(&mut rng, &colors);

    let paths = paths
        .iter()
        .map(|path| path.transform(|p| Point::from_xy((p.x() - tx) * scale, (p.y() - ty) * scale)));

    // yolo, don't care if it's a little wrong.
    let location = geo::LatLng::new(
        lerp(&(region.se.lat..region.nw.lat), ty / size.height()),
        lerp(&(region.nw.lng..region.se.lng), tx / size.width()),
    );

    let surface = ImageSurface::create(
        Format::ARgb32,
        opts.size().width() as i32,
        opts.size().height() as i32,
    )?;
    let ctx = Context::new(&surface)?;

    bg.set(&ctx);
    ctx.rectangle(0.0, 0.0, opts.size().width(), opts.size().height());
    ctx.fill()?;

    ctx.save()?;
    let lw = lerp(
        &opts.line_width_range(),
        inv_lerp(&opts.scale_range(), scale),
    );
    ctx.set_line_width(lw);
    for path in paths {
        ctx.new_path();
        path.draw(&ctx);
        fg.set(&ctx);
        ctx.stroke()?;
    }
    ctx.restore()?;

    if !opts.hide_location() {
        ctx.save()?;
        let label = format!("{}", location);
        ctx.select_font_face("Helvetica Neue", FontSlant::Normal, FontWeight::Normal);
        ctx.set_font_size(24.0);
        let exts = ctx.text_extents(&label)?;
        bg.set(&ctx);
        ctx.rectangle(
            opts.size().width() - exts.width() - 30.0,
            opts.size().height() + exts.y_bearing() - 30.0,
            exts.width() + 20.0,
            exts.height() + 20.0,
        );
        ctx.fill()?;

        fg.set(&ctx);
        ctx.move_to(
            opts.size().width() - exts.width() - 20.0,
            opts.size().height() - 20.0,
        );
        ctx.show_text(&label)?;
        ctx.fill()?;
        ctx.restore()?;
    }

    surface.write_to_png(&mut fs::File::create(opts.dest())?)?;
    Ok(())
}

pub trait Options {
    fn seed(&self) -> &Seed;

    fn size(&self) -> &Size;

    fn scale_range(&self) -> std::ops::Range<f64>;

    fn line_width_range(&self) -> std::ops::Range<f64>;

    fn theme(&self) -> &ThemeRef;

    fn hide_location(&self) -> bool;

    fn dest(&self) -> PathBuf;
}

fn select_color_pair(rng: &mut dyn RngCore, colors: &[Color]) -> (Color, Color) {
    let min_ix = colors
        .iter()
        .map(|c| c.luminance())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;
    let max_ix = colors
        .iter()
        .map(|c| c.luminance())
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;
    if rng.gen::<bool>() {
        (colors[min_ix], colors[max_ix])
    } else {
        (colors[max_ix], colors[min_ix])
    }
}

fn lerp(r: &std::ops::Range<f64>, v: f64) -> f64 {
    r.start * (1.0 - v) + r.end * v
}

fn inv_lerp(r: &std::ops::Range<f64>, v: f64) -> f64 {
    (v - r.start) / (r.end - r.start)
}
