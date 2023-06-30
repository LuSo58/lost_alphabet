use std::collections::HashMap;
use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;

use clap::Parser;
use image::DynamicImage;
use image::imageops::FilterType;
use itertools::Itertools;
use serde_json::de::from_reader;

#[derive(Clone, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short)]
    alphabet_file: PathBuf,

    #[clap(long, short)]
    images_folder: PathBuf,

    #[clap(long, short)]
    width: u32,

    #[clap(long, short)]
    height: u32,

    #[clap(long, short = 'W')]
    image_width: u32,

    #[clap(long, short = 'H')]
    image_height: u32,

    #[clap(long, short)]
    message: String,

    #[clap(long, short)]
    output_folder: PathBuf,
}

fn main() {
    let args = Args::parse();
    let alphabet = from_reader::<_, HashMap<char, PathBuf>>(File::open(&args.alphabet_file).expect("Could not open alphabet file"))
        .expect("Alphabet file has bad contents").into_iter().map(|(letter, filename)| (
        Some(letter).filter(char::is_ascii_alphabetic).map(char::to_lowercase).map(|mut x| x.next()).flatten().expect("Letters must be ascii alphabetic"),
        image::open(args.images_folder.join(filename)).expect("Could not open image file").resize(args.image_width, args.image_height, FilterType::CatmullRom)
    )).collect::<HashMap<char, DynamicImage>>();
    args.message.chars()
        .filter(char::is_ascii_alphabetic)
        .map(char::to_lowercase).flatten()
        .map(|letter| alphabet.get(&letter).expect(format!("Letter '{letter}' is not in alphabet file").deref()))
        .chunks(((args.width / args.image_width) * (args.height / args.image_height)) as usize)
        .into_iter().enumerate().for_each(|(order, images)| {
        let mut page = image::RgbaImage::new(args.width, args.height);
        for (y, row) in images.chunks((args.width / args.image_width) as usize).into_iter().enumerate() {
            for (x, symbol) in row.enumerate() {
                image::imageops::overlay(&mut page, symbol, (x * args.image_width as usize) as i64, (y * args.image_height as usize) as i64)
            }
        }
        page.save(args.output_folder.join(format!("{order}.png")));
    });
}
