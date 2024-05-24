use std::{error::Error, fs, io::BufReader};

use serde::{Deserialize, Serialize};
use xml_dom::level2::{convert::*, Document, Element, RefNode};

use crate::{Path, Point, Rect, Size};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[clap()]
    src: String,

    #[clap()]
    dst: String,
    // region: geo::Rect,
}

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

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let r = fs::File::open(&args.src)?;
    let r = BufReader::new(r);
    let doc = xml_dom::parser::read_reader(r)?;
    let root = doc.document_element().ok_or("no root element")?;
    let view_box = get_viewbox(root.clone())?;

    let root = as_element(&root)?;
    let mut paths = root
        .get_elements_by_tag_name("path")
        .iter()
        .filter(|&n| is_contour(n.clone()))
        .map(|n| n.get_attribute("d").ok_or("no d")?.parse::<Path>())
        .collect::<Result<Vec<_>, _>>()?;

    for path in &mut paths {
        path.transform(|p| Point::from_xy(p.x() - view_box.x(), p.y() - view_box.y()));
    }

    let data = Data {
        size: Size::new(view_box.width(), view_box.height()),
        paths,
    };

    serde_json::to_writer(&mut fs::File::create(&args.dst)?, &data)?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub size: Size,
    pub paths: Vec<Path>,
}
