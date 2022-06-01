use std::{collections::HashSet, path::{Path, PathBuf}, fs::File, io::{BufWriter, Write}};

mod sort;

fn main() {
    let width = 1000;
    let height = 500;
    let output_dir = PathBuf::from("images");

    let mut array = (0..width)
        .map(|x| x * height / width)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<i32>>();

    let mut frames = vec![array.clone()];

    let record_frame = |array: &[i32]| {
        if array != frames.last().unwrap() {
            frames.push(array.to_vec());
        }
    };

    sort::quicksort(&mut array, |a, b| a < b, record_frame);

    for (idx, frame) in frames.into_iter().enumerate() {
        dbg!(frame.len());
        let path = output_dir.join(format!("{}.ppm", idx));
        let data = draw_array(height as usize, &frame);
        dbg!(data.len());
        write_ppm(path, &data, frame.len() as _).unwrap();
    }
}

fn draw_array(height: usize, frame: &[i32]) -> Vec<u8> {
    let mut image = Vec::with_capacity(height * frame.len() * 3);

    for y in 0..height {
        for &elem in frame {
            if elem > y as i32 {
                image.push(0xff);
                image.push(0);
                image.push(0);
            } else {
                image.push(0);
                image.push(0);
                image.push(0);
            }
        }
    }

    image
}

pub fn write_ppm(path: impl AsRef<Path>, data: &[u8], width: usize) -> std::io::Result<()> {
    assert!(
        data.len() % 3 == 0,
        "Data length must be a multiple of 3 (RGB)"
    );
    let n_pixels = data.len() / 3;

    assert!(
        n_pixels % width == 0,
        "Pixel count must be a multiple of width"
    );
    let height = n_pixels / width;

    let file = File::create(path)?;
    let mut file = BufWriter::new(file);

    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "{}", u8::MAX)?;

    file.write_all(&data)?;

    Ok(())
}
