/*
 * Copyright (C) 2025 Dustyn Gibb
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */
use crate::file_encoding_support::file_encoding_support;
use crate::file_encoding_support::file_encoding_support::{
    FileEncoding, FileEncodingFunctionDerivation, FileEncodingMethod, FileEncodingSupport,
    Operation,
};
use crate::file_encoding_support::pixel::Pixel;
use crate::filetype_support::bmp::BmpPixelType::{Rgb, Rgba};
use std::fs::File;
use std::io::{Read, Seek};
use std::process::exit;
use std::{io, mem};

const BMP_MAGIC: u16 = 0x4D42;
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct BitmapFileHeader {
    pub bf_type: u16,      // File type always BM (0x4D42)
    pub bf_size: u32,      // Size of the file (in bytes)
    pub bf_reserved1: u16, // Reserved - must be 0
    pub bf_reserved2: u16, // Reserved - must be 0
    pub bf_off_bits: u32,  // Offset to bitmap data
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct BitmapDIBHeader {
    pub bi_size: u32,             // Size of this header (40 bytes, for BITMAPINFOHEADER)
    pub bi_width: i32,            // Width of bitmap in pixels
    pub bi_height: i32, // Height of bitmap in pixels. If positive, bottom-up. If negative, top-down.
    pub bi_planes: u16, // Number of color planes (must be 1 for all bitmaps)
    pub bi_bit_count: u16, // Bits per pixel (e.g., 24 for RGB, 1 for monochrome)
    pub bi_compression: u32, // Compression type (e.g., 0 = none, 1 = BI_RLE8, 2 = BI_RLE4)
    pub bi_size_image: u32, // Image size in bytes (may be 0 for uncompressed images)
    pub bi_x_pels_per_meter: i32, // Horizontal resolution of the image (pixels per meter)
    pub bi_y_pels_per_meter: i32, // Vertical resolution of the image (pixels per meter)
    pub bi_clr_used: u32, // Number of colors in the color palette (0 = default, 2^bi_bit_count colors)
    pub bi_clr_important: u32, // Number of important colors (0 = all colors are important)
}
/*
   Presence is mandatory when bits per pixel is <= 8

   We likely won't need this

   The size of color table entries is 3 bytes if BITMAPCOREHEADER is
   substituted for BITMAPV5HEADER
*/
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct BitmapColorTable {
    pub blue: u8,     // Blue component (0-255)
    pub green: u8,    // Green component (0-255)
    pub red: u8,      // Red component (0-255)
    pub reserved: u8, // Reserved (must be 0)
}

// For a 24-bit BMP, a pixel is usually 3 bytes: B, G, R.
#[repr(C, packed)]
#[derive(Debug, Default, Clone)]
struct RgbPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

// For a 32-bit BMP, a pixel is 4 bytes: B, G, R, A (or reserved)
#[repr(C, packed)]
#[derive(Debug, Default, Clone)]
struct RgbaPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8, // Can be actual alpha or just padding (usually 0 or 255)
}

#[repr(C, packed)]
struct BmpBitmap<P> {
    pub width: u32,
    pub height: u32,
    pub pixel_map: Vec<P>,
}

pub struct BmpImageParser<P: Pixel> {
    bmp_header: BitmapFileHeader,
    bmp_dib_header: BitmapDIBHeader,
    pixel_size: u8,
    padding_size: u8,
    pixel_map: BmpBitmap<P>,
    image_file: File,
}

// For RGB pixel type
impl Pixel for RgbPixel {
    fn red(&self) -> u8 {
        self.red
    }
    fn green(&self) -> u8 {
        self.green
    }
    fn blue(&self) -> u8 {
        self.blue
    }
    fn alpha(&self) -> u8 {
        255
    } // No alpha in RGB, so always 255

    fn set_red(&mut self, value: u8) {
        self.red = value
    }
    fn set_green(&mut self, value: u8) {
        self.green = value
    }
    fn set_blue(&mut self, value: u8) {
        self.blue = value
    }
    fn set_alpha(&mut self, value: u8) { /* No-op for RGB */
    }
}

// For RGBA pixel type
impl Pixel for RgbaPixel {
    fn red(&self) -> u8 {
        self.red
    }
    fn green(&self) -> u8 {
        self.green
    }
    fn blue(&self) -> u8 {
        self.blue
    }
    fn alpha(&self) -> u8 {
        self.alpha
    }

    fn set_red(&mut self, value: u8) {
        self.red = value
    }
    fn set_green(&mut self, value: u8) {
        self.green = value
    }
    fn set_blue(&mut self, value: u8) {
        self.blue = value
    }
    fn set_alpha(&mut self, value: u8) {
        self.alpha = value
    }
}

/*
   We will just add support for 24 bit and 32 bit pixel sizes, will likely only encounter 24 bit pixels
*/

pub enum BmpPixelType {
    Rgb,
    Rgba,
}

/*
pub fn dmp_derive_pixel_type(file_location: &str) -> BmpPixelType {
    let mut file: File = match File::open(file_location) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "bmp.rs: derive_pixel_type: Failed to open file {} , exiting ...",
                file_location
            );
            exit(1);
        }
    };

    match file.seek_relative(14) {
        Ok(x) => x,
        Err(_) => {
            println!(
                "bmp.rs: parse_file: Failed to seek in file {} , exiting ...",
                file_location
            );
            exit(1);
        }
    };

    let mut dib_header_bytes = [0u8; std::mem::size_of::<BitmapDIBHeader>()];

    match file.read(&mut dib_header_bytes) {
        Ok(i) => unsafe {
            let dib_header = dib_header_bytes.as_ptr() as *const BitmapDIBHeader;

            match (*(dib_header.bi_bit_count)) / 8){
                3 => {return Rgb},

                4 => {return Rgba},

                _ => {
                    println!(
                        "bmp.rs: dmp_derive_pixel_type: Pixel size does not make sense, exiting ..."
                    );
                    exit(1);
                }
            }
        },

        Err(e) => {
            println!(
                "bmp.rs: derive_pixel_type: Failed to read file header, exiting ... {}",
                e
            );
            exit(1);
        }
    }
}
*/
impl<P: Pixel> FileEncodingSupport for BmpImageParser<P> {
    fn parse_file(&mut self, file_location: &str) {
        let header_size = std::mem::size_of::<BitmapFileHeader>();
        let dib_header_size = std::mem::size_of::<BitmapDIBHeader>();

        let mut file: File = match File::open(file_location) {
            Ok(file) => file,
            Err(e) => {
                println!(
                    "bmp.rs: parse_file: Failed to open file {} , exiting ...",
                    file_location
                );
                exit(1);
            }
        };

        let mut header_bytes = [0u8; std::mem::size_of::<BitmapFileHeader>()];

        match file.read(&mut header_bytes) {
            Ok(i) => {
                if i != header_size {
                    println!("bmp.rs: parse_file: File Header Size Mismatch, exiting ...");
                    exit(1);
                }
                unsafe {
                    let header_pointer: *mut BitmapFileHeader = &mut self.bmp_header;
                    std::ptr::copy(
                        header_bytes.as_ptr(),
                        header_pointer as *mut u8,
                        header_bytes.len(),
                    );
                }
            }
            Err(e) => {
                println!(
                    "bmp.rs: parse_file: Failed to read from file {} , exiting ...",
                    file_location
                );
                exit(1);
            }
        }

        match file.seek_relative(14) {
            Ok(x) => x,
            Err(_) => {
                println!(
                    "bmp.rs: parse_file: Failed to seek in file {} , exiting ...",
                    file_location
                );
                exit(1);
            }
        };

        let mut dib_header_bytes = [0u8; std::mem::size_of::<BitmapDIBHeader>()];

        match file.read(&mut dib_header_bytes) {
            Ok(i) => {
                if i != dib_header_size {
                    println!("bmp.rs: parse_file: File DIB Header Size Mismatch, exiting ...");
                    exit(1);
                }

                unsafe {
                    let dib_header_pointer: *mut BitmapDIBHeader = &mut self.bmp_dib_header;
                    std::ptr::copy(
                        dib_header_bytes.as_ptr(),
                        dib_header_pointer as *mut u8,
                        dib_header_bytes.len(),
                    );
                }
            }
            Err(e) => {
                println!(
                    "bmp.rs: parse_file: Failed to read from file {} , exiting ...",
                    file_location
                );
                exit(1);
            }
        }

        self.pixel_map.width = self.bmp_dib_header.bi_width as u32;
        self.pixel_map.height = self.bmp_dib_header.bi_height as u32;

        if self.bmp_dib_header.bi_size != 40 {
            /*
               Bit extreme, we'll keep it for now and see.
               If V5 headers are common I'll remove this
            */
            println!("bmp.rs: parse_file: File Size Mismatch, exiting ...");
            exit(1);
        }

        self.padding_size = ((self.pixel_map.width as u64 * self.pixel_size as u64) % 4) as u8;
        self.pixel_size = (self.bmp_dib_header.bi_bit_count / 8) as u8;
    }

    fn embed_data(
        &mut self,
        data: &mut Vec<u8>,
        encoding: FileEncoding,
        encoding_method: FileEncodingMethod,
        file_encoding_function_derivation: FileEncodingFunctionDerivation,
    ) {
        todo!()
    }

    fn retrieve_data(
        &mut self,
        encoding: FileEncoding,
        encoding_method: FileEncodingMethod,
        file_encoding_function_derivation: FileEncodingFunctionDerivation,
    ) {
        todo!()
    }

    fn write_file(&mut self, file: &mut File, location: &str) {
        todo!()
    }

    fn validate_state(&mut self) {
        todo!()
    }
}
