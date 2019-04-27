extern crate fnv;
extern crate rayon;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use fnv::FnvHashSet;
use rayon::prelude::*;

use crate::image::{Image, Tags};
use crate::slide::{get_union, FSlide, Slide};

pub mod cli {
    use structopt::StructOpt;

    #[derive(StructOpt)]
    pub struct Cli {
        #[structopt(parse(from_os_str))]
        pub path: std::path::PathBuf,
        pub chunk_size: usize,
    }
}

mod image {
    use std::cmp;

    use fnv::FnvHashSet;

    use crate::image::Image::{Horizontal, Vertical};

    pub type Tags = FnvHashSet<String>;

    #[derive(Clone, Debug)]
    pub enum Image {
        Horizontal {
            image_id: usize,
            tags: FnvHashSet<String>,
        },
        Vertical {
            image_id: usize,
            tags: FnvHashSet<String>,
        },
    }

    impl Image {
        pub fn new(image_id: usize, horizontal: bool, tags: Tags) -> Image {
            if horizontal {
                Horizontal { image_id, tags }
            } else {
                Vertical { image_id, tags }
            }
        }

        pub fn score(tags_set: &Tags, other_tags_set: &Tags) -> usize {
            let same = FnvHashSet::intersection(tags_set, other_tags_set).count();
            if same == 0 {
                return 0;
            };
            let unique = FnvHashSet::difference(tags_set, other_tags_set).count();
            if unique == 0 {
                return 0;
            };
            let other_unique = FnvHashSet::difference(other_tags_set, tags_set).count();
            if other_unique == 0 {
                return 0;
            };
            cmp::min(cmp::min(unique, other_unique), same)
        }

        pub fn get_id(&self) -> usize {
            match *self {
                Horizontal { image_id, .. } => image_id,
                Vertical { image_id, .. } => image_id,
            }
        }

        pub fn get_tags(&self) -> &Tags {
            match self {
                Horizontal { tags, .. } => tags,
                Vertical { tags, .. } => tags,
            }
        }
    }
}

mod slide {
    use std::iter::FromIterator;

    use fnv::FnvHashSet;

    use crate::image::{Image, Tags};

    pub enum Slide<'a> {
        H { h: &'a Image },
        V { v: &'a Image, other_v: &'a Image },
    }

    // only image id stored
    pub enum FSlide {
        H { h: usize },
        V { v: usize, other_v: usize },
    }

    pub fn get_union(tags: &Tags, other_tags: &Tags) -> Tags {
        let union = tags.union(other_tags);
        FnvHashSet::from_iter(union.cloned())
    }

    impl<'a> Slide<'a> {
        pub fn get_score_slide(previous_tags: &Tags, s: &Slide) -> usize {
            match s {
                Slide::H { h } => Image::score(previous_tags, &h.get_tags()),
                Slide::V { v, other_v } => {
                    Image::score(previous_tags, &get_union(v.get_tags(), other_v.get_tags()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn test_score() {
        let tag_vec1 = vec!["fizz", "buzz"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        ;
        let tag_vec2 = vec!["fizz", "ferris"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        ;
        let tag1: Tags = FnvHashSet::from_iter(tag_vec1);
        let tag2: Tags = FnvHashSet::from_iter(tag_vec2);
        assert_eq!(Image::score(&tag1, &tag1), 0);
        assert_eq!(Image::score(&tag1, &tag2), 1);
    }
}

/// Parse problem statement file.
pub fn parse_input_file(path: std::path::PathBuf) -> Vec<image::Image> {
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
        let mut tags_set: Tags = FnvHashSet::default();
        for tag in iter {
            tags_set.insert(tag.to_string());
        }
        let image = image::Image::new(image_id - 1, orientation == "H", tags_set);
        images.push(image);
    }
    images
}

/// Dump `slides` to file at `path`.
pub fn dump(path: std::path::PathBuf, slides: Vec<FSlide>) {
    let file = File::create(path).expect("file could not be opened");
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", slides.len()).expect("io error");
    for slide in slides {
        match slide {
            FSlide::H { h } => {
                writeln!(writer, "{}", h).expect("io error");
            }
            FSlide::V { v, other_v } => {
                writeln!(writer, "{} {}", v, other_v).expect("io error");
            }
        }
    }
}

fn get_best_horizontal(previous_tags: &Tags, horizontals: &[Image]) -> (usize, Option<Image>) {
    match horizontals
        .into_par_iter()
        .max_by_key(|&image| Slide::get_score_slide(previous_tags, &Slide::H { h: image }))
    {
        None => (0, None),
        Some(best_h) => (
            Slide::get_score_slide(previous_tags, &Slide::H { h: best_h }),
            Some(best_h.clone()),
        ),
    }
}

fn get_best_vertical(previous_tags: &Tags, verticals: &[Image]) -> (usize, Option<(Image, Image)>) {
    match verticals.split_first() {
        None => (0, None),
        Some((first_v, other_verticals)) => {
            match other_verticals.into_par_iter().max_by_key(|&image| {
                Slide::get_score_slide(
                    previous_tags,
                    &Slide::V {
                        v: first_v,
                        other_v: image,
                    },
                )
            }) {
                None => (0, None),
                Some(best_v) => (
                    Slide::get_score_slide(
                        previous_tags,
                        &Slide::V {
                            v: first_v,
                            other_v: best_v,
                        },
                    ),
                    Some((first_v.clone(), best_v.clone())),
                ),
            }
        }
    }
}

/// Greedy algorithm on a subset (slice) of images:
/// Each slide is paired with the next _almost-best_ slide within the subset considered.
///
/// _almost-best_ means it uses an heuristic for vertical slides: the first vertical image
/// considered is paired with every other remaining vertical images (`O(n)`) instead of
/// going through all possible pairs (`O(n^2)`).
///
/// Overall complexity is approximately `O(n^2)` with `n` being the size of the slice considered.
pub fn solve(images: &[Image]) -> Vec<FSlide> {
    let mut slides: Vec<FSlide> = vec![];
    let mut horizontals: Vec<Image> = vec![];
    let mut verticals: Vec<Image> = vec![];
    for image in images {
        match image {
            Image::Horizontal { .. } => horizontals.push(image.clone()),
            Image::Vertical { .. } => verticals.push(image.clone()),
        }
    }
    let mut first_h: Image;
    let mut first_v: Image;
    let mut first_other_v: Image;
    let mut union;
    let mut previous_tags = if !horizontals.is_empty() {
        first_h = horizontals.pop().expect("horizontals empty");
        first_h.get_tags()
    } else {
        first_v = verticals.pop().expect("verticals empty");
        first_other_v = verticals.pop().expect("verticals empty");
        union = get_union(first_v.get_tags(), first_other_v.get_tags());
        &union
    };
    let mut previous_h: Image;
    loop {
        let (best_score_h, best_image_h) = get_best_horizontal(previous_tags, &horizontals);
        let (best_score_v, best_images_v) = get_best_vertical(previous_tags, &verticals);
        let use_horizontal = match (&best_image_h, &best_images_v) {
            (None, None) => Option::None,
            (Some(..), None) => Some(true),
            (None, Some(..)) => Some(false),
            (Some(..), Some(..)) => {
                if best_score_h >= best_score_v {
                    Some(true)
                } else {
                    Some(false)
                }
            }
        };
        match use_horizontal {
            None => {
                break;
            }
            Some(true) => {
                let h = best_image_h.expect("previous filter should prevent that");
                previous_h = h;
                previous_tags = previous_h.get_tags();
                slides.push(FSlide::H {
                    h: previous_h.get_id(),
                });
                horizontals.retain(|img| img.get_id() != previous_h.get_id())
            }
            Some(false) => {
                let (v0, v1) = best_images_v.expect("previous filter should prevent that");
                union = get_union(v0.get_tags(), v1.get_tags());
                previous_tags = &union;
                slides.push(FSlide::V {
                    v: v0.get_id(),
                    other_v: v1.get_id(),
                });
                verticals.retain(|img| img.get_id() != v0.get_id() && img.get_id() != v1.get_id());
            }
        }
    }
    slides
}
