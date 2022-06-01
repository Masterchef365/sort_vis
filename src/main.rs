use std::{collections::HashSet, path::{Path, PathBuf}, fs::File, io::Write};

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

    let full_array_ptr = array.as_ptr();

    let record_frame = |view: &[i32]| {
        let last = frames.last().unwrap();
        if view != last {
            let off = unsafe { view.as_ptr().offset_from(full_array_ptr) };
            let off = off as usize;
            let mut frame = last.to_vec();

            frame[off..][..view.len()].copy_from_slice(view);

            frames.push(frame);
        }
    };

    sort::quicksort(&mut array, |a, b| a > b, record_frame);

    for (idx, frame) in frames.into_iter().enumerate() {
        let path = output_dir.join(format!("{:04}.ppm", idx));
        let data = draw_array(height as usize, &frame);
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

    let mut file = File::create(path)?;

    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "{}", u8::MAX)?;

    file.write_all(&data)?;

    Ok(())
}
