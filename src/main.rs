use std::io::Write;

mod header;

pub struct Frame<const N: usize>(pub [u8; N]);

impl<const N: usize> Frame<N> {
    fn fill(&mut self, r: u8, g: u8, b: u8) {
        for i in 0..N {
            self.0[i] = if i % 3 == 0 { r } else if i % 3 == 1 { g } else { b };
        }
    }
}

fn main() {
    let step_time_ms = 20;
    let frame_count = 2;
    let pixels = 1000;

    let header = header::FSEQHeader::v1(step_time_ms, frame_count, pixels);

    let mut file = std::fs::File::create("output.fseq").unwrap();
    header.write(&mut file).unwrap();

    let mut frame = Frame([0; 6000]);
    frame.fill(255, 0, 0);
    file.write_all(&frame.0).unwrap();
}
