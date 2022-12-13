#![allow(unused_must_use)]


use std::{collections::HashMap, fs::OpenOptions, io::Write};

use anyhow::Result;
use image::{ImageBuffer, Luma};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=font");
    let source = image::open("./font/original.png")?.to_luma8();
    // let h_offset = source.rows()
    //     .map(|row| row
    //         .take_while(|px| px.0[0] == 0)
    //         .count()
    //     )
    //     .min()
    //     .unwrap();
    // // should be ==7
    // assert!(h_offset < 15, "hmmmmmmm");
    // let v_offset = source.rows()
    //     .map(|row| row
    //         .take_while(|px| px.0[0] == 0)
    //         .count()
    //     )
    //     .take_while(|count| dbg!(*dbg!(count) > h_offset))
    //     .count();
    // TAKE NOTE: the bottom tri-color letters are ONE PIXEL TO THE LEFT, WHICH BREAKS THIS!
    // println!("v_offset: {v_offset}");
    // assert!(v_offset < 15, "hMMMMmmMMmMmMmM");
    let (h_offset, v_offset) = (8usize, 4usize); // pixels before getting to zero in the top left

    let off = Luma([0]);
    let on = Luma([255]);

    // captial `X` is the no-charecter char, as all charecters are lowercase.
    // each entry represents 1 row
    let map = &[
        "0123456789",
        "abcdefghij",
        "klmnopqrst",
        "uvwxyz!()?",
        "=\"+&XXXXX", //can omit X here, since it is at the end
        "XXXXXXXXX:", //cannot omit X here, since it is at the start
    ];

    let generated = generate_font(&source, (h_offset, v_offset), (on, off), map);
    println!("{generated:#?}");

    let output_map = "0123456789abcdefghijklmnopqrstuvwxyz ~`!@#%^&*_=+-(){}[]|\\:;\"\'<>?/,.";
    let mut code = String::from("pub const FONT: Font = Font::new([\n");
    for c in output_map.chars() {
        use std::fmt::Write;
        if let Some(bitmap) = generated.get(&c) {
            writeln!(&mut code, "make_char_bool_array({:?}),", bitmap);
        } else {
            writeln!(&mut code, "None,");
        }
    }
    code.push_str("]);");
    OpenOptions::new().create(true).truncate(true).write(true).open("gen/font.rs")?.write_all(code.as_bytes())?;
    Ok(())
}


fn generate_font(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    (h_offset, v_offset): (usize, usize),
    (on, off): (Luma<u8>, Luma<u8>),
    map: &[&str],
) -> HashMap<char, [[bool; 4]; 4]> {
    map
        .iter()
        .enumerate()
        .map(|(y, row)| row.chars().enumerate().map(move |(x, row)| (x, y, row)))
        .flatten()
        .map(|(x, y, c)| (x * 5, y * 5, c)/* letters are 4x4, so multiply */)
        .filter(|(_, _, c)| *c != 'X')
        .map(|(letter_x_offset, letter_y_offset, letter)| {
            (letter, (0..4)
                .map(|y| 
                    (0..4)
                        .map(|x| {
                            println!("x={h_offset}+{letter_x_offset}+{x}\ny={v_offset}+{letter_y_offset}+{y}\n");
                            let px = (h_offset + letter_x_offset + x) as u32;
                            let py = (v_offset + letter_y_offset + y) as u32;
                            assert!(px < 57);
                            assert!(py < 34);
                            match image.get_pixel_checked(px, py)
                            {
                                Some(&color) if color == on => { true }
                                Some(&color) if color == off => { false }
                                Some(other) => {
                                    panic!("Unexpected other pixel {other:?} (is not {on:?} or {off:?})")
                                }
                                None => panic!("Font map is too large for input image!"),
                            }
                        })
                        .collect::<Vec<bool>>()
                        .try_into()
                        .unwrap()
                )
                .collect::<Vec<[bool; 4]>>()
                .try_into()
                .unwrap())
        })
        .collect()
}
