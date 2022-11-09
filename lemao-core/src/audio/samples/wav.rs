use super::*;
use crate::utils::binary;
use std::fs::File;
use std::io::Read;

pub fn load(path: &str) -> Result<Sample, String> {
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    // WAV specification: https://sites.google.com/site/musicgapi/technical-documents/wav-file-format //
    ////////////////////////////////////////////////////////////////////////////////////////////////////

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("File {} not found", path)),
    };

    let mut wav = Vec::new();
    if let Err(message) = file.read_to_end(&mut wav) {
        return Err(format!("Error while reading file: {}", message));
    }

    if binary::read_le_u32(&wav, 0) != 0x46464952 {
        return Err("Invalid signature, not recognized as WAV file".to_string());
    }

    let file_size = binary::read_le_u32(&wav, 4) as usize;
    if file_size + 8 != wav.len() {
        return Err("Invalid file, expected size doesn't match the real one".to_string());
    }

    if binary::read_le_u32(&wav, 8) != 0x45564157 {
        return Err("Invalid signature, not recognized as WAV file".to_string());
    }

    let mut channels_count = 0;
    let mut frequency = 0;
    let mut bits_per_sample = 0;
    let mut data: &[u8] = &[];

    let mut index = 12;
    while index < wav.len() - 3 {
        match binary::read_le_u32(&wav, index) {
            // Format chunk
            0x20746D66 => {
                let chunk_size = binary::read_le_u32(&wav, index + 4);

                let compression_code = binary::read_le_u16(&wav, index + 8);
                if compression_code != 1 {
                    return Err("Only PCM/uncompressed WAV is supported".to_string());
                }

                channels_count = binary::read_le_u16(&wav, index + 10) as u32;
                if channels_count != 1 && channels_count != 2 {
                    return Err("Only mono and stereo WAV is supported".to_string());
                }

                frequency = binary::read_le_u32(&wav, index + 12);

                bits_per_sample = binary::read_le_u16(&wav, index + 22) as u32;
                if bits_per_sample != 8 && bits_per_sample != 16 {
                    return Err("Only 8 and 16 bits per sample WAVs are supported".to_string());
                }

                index += chunk_size as usize;
            }
            // Data chunk
            0x61746164 => {
                let chunk_size = binary::read_le_u32(&wav, index + 4);

                data = &wav[index + 8..];
                index += chunk_size as usize;
            }
            _ => {
                index += 1;
            }
        }
    }

    Ok(Sample::new(channels_count, frequency, bits_per_sample, data.to_vec()))
}
