use std::io::Write;

// info from https://github.com/Cryptkeeper/go-fseq
#[derive(Debug, Clone, Copy)]
pub struct FSEQHeader {
    pub channel_data_start_offset: u16,
    pub minor_version: u8,
    pub major_version: u8,
    pub header_length: u16,
    pub channel_count: u32,
    pub frame_count: u32,
    pub step_time_ms: u8,
    pub flags: u8, // ignored, default is 0,
    pub universe_count: u16, // ignored
    pub universe_size: u16, // ignored
    pub gamma: u8, // ignored, default is 1
    pub color_encoding: u8, // ignored, default is 2 (RGB)
    pub reserved: [u8; 2], // ignored, default is 0
}

const HEADER_LENGTH: u16 = 28;

impl FSEQHeader {
    pub fn v1(step_time_ms: u8, frame_count: u32, pixels: u32) -> Self {
        Self {
            // https://gitlab.com/sharebear/fseq_parser/-/blob/master/fseq-nommer/src/lib.rs?ref_type=heads#L135
            channel_data_start_offset: HEADER_LENGTH,
            minor_version: 1,
            major_version: 1,
            header_length: HEADER_LENGTH,
            channel_count: pixels * 3,
            frame_count,
            step_time_ms,
            flags: 0,
            universe_count: 0,
            universe_size: 0,
            gamma: 1,
            color_encoding: 2,
            reserved: [0, 0]
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_all("PSEQ".as_bytes())?;
        writer.write_all(&self.channel_data_start_offset.to_le_bytes())?;
        writer.write_all(&self.minor_version.to_le_bytes())?;
        writer.write_all(&self.major_version.to_le_bytes())?;
        writer.write_all(&self.header_length.to_le_bytes())?;
        writer.write_all(&self.channel_count.to_le_bytes())?;
        writer.write_all(&self.frame_count.to_le_bytes())?;
        writer.write_all(&self.step_time_ms.to_le_bytes())?;
        writer.write_all(&self.flags.to_le_bytes())?;
        writer.write_all(&self.universe_count.to_le_bytes())?;
        writer.write_all(&self.universe_size.to_le_bytes())?;
        writer.write_all(&self.gamma.to_le_bytes())?;
        writer.write_all(&self.color_encoding.to_le_bytes())?;
        writer.write_all(&self.reserved)?;
        Ok(())
    }
}
