use std::collections::BTreeSet;
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::image::Image;

mod cli {
    use structopt::StructOpt;

    #[derive(StructOpt)]
    pub struct Cli {
        #[structopt(parse(from_os_str))]
        pub path: std::path::PathBuf,
    }
}

mod image {
    use std::collections::BTreeSet;
    use std::cmp;

    #[derive(Debug)]
    pub struct Image {
        image_id: usize,
        horizontal: bool,
        tags: BTreeSet<String>,
    }

    impl Image {
        pub fn new(image_id: usize, horizontal: bool, tags: BTreeSet<String>) -> Image {
            Image { image_id, horizontal, tags }
        }

        pub fn score(tags_set: &BTreeSet<String>, other_tags_set: &BTreeSet<String>) -> usize {
            let unique = BTreeSet::difference(tags_set, other_tags_set).count();
            let other_unique = BTreeSet::difference(other_tags_set, tags_set).count();
            let same = BTreeSet::intersection(tags_set, other_tags_set).count();
            return cmp::min(cmp::min(unique, other_unique), same);
        }

        pub fn get_id(self) -> usize {
            self.image_id
        }

        pub fn get_tags(self) -> BTreeSet<String> {
            self.tags
        }
    }
}

enum Slide {
    Horizontal(Image),
    Vertical(Image, Image)
}

fn parse_input_file(path: std::path::PathBuf) -> Option<Vec<image::Image>> {
    let file = File::open(path).expect("file could not be opened");
    let mut images: Vec<image::Image> = vec![];
    let reader = BufReader::new(file);
    for (image_id, line) in reader.lines().enumerate() {
        if image_id == 0 {
            continue; // images nb
        }
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let orientation = iter.next().unwrap();
        let orientation = orientation.to_string();
        let _ = iter.next(); // tags nb
        let mut tags_set = BTreeSet::new();
        for tag in iter {
            tags_set.insert(tag.to_string());
        }
        let image = image::Image::new(image_id - 1, orientation == "H", tags_set);
        images.push(image);
    }
    Option::Some(images)
}


fn main() -> std::io::Result<()> {
    let args = cli::Cli::from_args();
    let images = parse_input_file(args.path).expect("could not load images");
    for image in images {
        println!("{:?}", image);
    }
    Ok(())
}