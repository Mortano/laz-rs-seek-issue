use anyhow::{anyhow, bail, Result};
use las::{self, point::Format, Builder, Point, Read, Write};
use std::io::Cursor;

fn main() -> Result<()> {
    // Changing these two variables illustrates that the issue only happens when seeking into the last chunk
    // e.g. COUNT = 100k, SEEK_POSITION = 10 works just fine
    const COUNT: usize = 20;
    const SEEK_POSITION: usize = 10;

    let mut memory_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::default());
    // Write a LAZ file in point format 6 with some default points
    {
        let mut header_builder = Builder::from((1, 4));
        header_builder.point_format = Format::new(6)?;
        header_builder.point_format.is_compressed = true;
        let mut laz_writer = las::Writer::new(memory_buffer, header_builder.into_header()?)?;
        for idx in 0..COUNT {
            let point = Point {
                // Use the intensities to store point index
                intensity: idx as u16,
                gps_time: Some(0.0),
                ..Default::default()
            };
            laz_writer.write(point)?;
        }
        laz_writer.close()?;
        memory_buffer = laz_writer.into_inner()?;
    }

    memory_buffer.set_position(0);

    // Seek to some later position within the file and read the remaining points. Assert that the correct
    // points have been read
    {
        let mut laz_reader = las::Reader::new(&mut memory_buffer)?;
        laz_reader.seek(SEEK_POSITION as u64)?;

        for idx in SEEK_POSITION..COUNT {
            let point = laz_reader
                .read()
                .ok_or(anyhow!("No point could be read at index {idx}"))??;
            if point.intensity != idx as u16 {
                bail!(
                    "Point at index {idx} has wrong ID. Expected {idx} but got {}",
                    point.intensity
                );
            }
        }
    }

    Ok(())
}
