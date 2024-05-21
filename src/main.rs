use std::{error::Error, fs, io::BufReader};

use cairo::{Context, Format, ImageSurface};
use clap::Parser;
use rand::Rng;
use xml_dom::level2::{convert::*, RefNode};
use xml_dom::level2::{Document, Element};

use topo::{Path, Point, Polyline, Range, Rect, Seed, Size};

fn get_viewbox(root: RefNode) -> Result<Rect, Box<dyn Error>> {
    let root = as_element(&root)?;
    let viewbox = root.get_attribute("viewBox").ok_or("no viewbox")?;
    viewbox.parse()
}

fn is_contour(path: RefNode) -> bool {
    path.get_attribute("stroke") == Some(String::from("rgb(69.802856%, 69.802856%, 69.802856%)"))
        && path.get_attribute("fill") == Some(String::from("none"))
        && path.get_attribute("stroke-width") == Some(String::from("0.99001"))
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value_t = Default::default(), value_parser = Seed::from_arg)]
    seed: Seed,

    #[clap(long, default_value_t = Size::new(1600.0,600.0), value_parser = Size::from_arg)]
    size: Size,

    #[clap(long, default_value_t=String::from("topo.png"))]
    dest: String,

    #[clap(long, value_parser=Range::from_arg, default_value_t=Range::from(1.0..8.0))]
    scale_range: Range,

    #[clap()]
    src: String,
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("{:?}", args);
    let mut rng = args.seed.rng();

    let scale = rng.gen_range(args.scale_range.to_std().clone());

    let r = fs::File::open(args.src)?;
    let mut br = BufReader::new(r);
    let doc = xml_dom::parser::read_reader(&mut br)?;
    let root = doc.document_element().ok_or("no root element")?;
    let view_box = get_viewbox(root.clone())?;

    let surface = ImageSurface::create(
        Format::ARgb32,
        args.size.width() as i32,
        args.size.height() as i32,
    )?;
    let ctx = Context::new(&surface)?;

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.rectangle(0.0, 0.0, view_box.width(), view_box.height());
    ctx.fill()?;

    ctx.translate(view_box.x(), view_box.y());

    let tx = rng.gen_range(view_box.x()..view_box.width() - args.size.width());
    let ty = rng.gen_range(view_box.y()..view_box.height() - args.size.height());
    println!("tx = {}, ty = {}, scale = {}", tx, ty, scale);

    let root = as_element(&root)?;
    let paths = root.get_elements_by_tag_name("path");
    let paths = paths.iter().filter(|&n| is_contour(n.clone()));

    let mut count = 0;
    for path in paths {
        count += 1;
        let d = path.get_attribute("d").ok_or("no d")?;
        let mut path = d.parse::<Path>()?;
        path.transform(|x, y| ((x - tx) * scale, (y - ty) * scale));

        ctx.new_path();
        path.draw(&ctx);
        ctx.set_source_rgb(1.0, 0.0, 0.4);
        ctx.set_line_width(2.0);
        ctx.stroke()?;

        let mut poly = d.parse::<Polyline>()?;
        if poly.len() >= 2 {
            poly.transform(|p| Point::from_xy((p.x() - tx) * scale, (p.y() - ty) * scale));
            ctx.new_path();
            path.draw(&ctx);
            ctx.set_source_rgba(1.0, 1.0, 1.0, 0.5);
            ctx.set_line_width(3.0);
            ctx.stroke()?;
        }
    }

    println!("count = {}", count);
    surface.write_to_png(&mut fs::File::create(args.dest)?)?;

    Ok(())
}
