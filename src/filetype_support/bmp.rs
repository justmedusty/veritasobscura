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
use crate::file_encoding_support::file_encoding_support::{
    FileEncoding, FileEncodingFunctionDerivation, FileEncodingMethod, FileEncodingSupport,
};
use crate::file_encoding_support::pixel::Pixel;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::mem;

const BMP_MAGIC: u16 = 0x4D42;
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BitmapFileHeader {
    pub bf_type: u16,      // File type always BM (0x4D42)
    pub bf_size: u32,      // Size of the file (in bytes)
    pub bf_reserved1: u16, // Reserved - must be 0
    pub bf_reserved2: u16, // Reserved - must be 0
    pub bf_off_bits: u32,  // Offset to bitmap data
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BitmapDIBHeader {
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
pub struct BitmapColorTable {
    pub blue: u8,     // Blue component (0-255)
    pub green: u8,    // Green component (0-255)
    pub red: u8,      // Red component (0-255)
    pub reserved: u8, // Reserved (must be 0)
}

// For a 24-bit BMP, a pixel is usually 3 bytes: B, G, R.
#[repr(C, packed)]
#[derive(Debug, Default, Clone)]
pub struct RgbPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

// For a 32-bit BMP, a pixel is 4 bytes: B, G, R, A (or reserved)
#[repr(C, packed)]
#[derive(Debug, Default, Clone)]
pub struct RgbaPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8, // Can be actual alpha or just padding (usually 0 or 255)
}

pub struct BmpBitmap {
    pub width: u32,
    pub height: u32,
    pub pixel_map_start: u64, // File offset where pixel map begins, will be indexed via file_data
}

pub struct BmpImageParser {
    pub bmp_header: BitmapFileHeader,
    pub bmp_dib_header: BitmapDIBHeader,
    pub pixel_size: u8,
    pub padding_size: u8,
    pub pixel_map: BmpBitmap,
    pub image_file: File,
    pub file_data: Box<Vec<u8>>,
    ready: bool
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
    }// No alpha in RGB, so always 255

    fn first(&self) -> u8 {
        self.blue
    }

    fn second(&self) -> u8 {
        self.green
    }
    fn third(&self) -> u8 {
        self.red
    }
    fn fourth(&self) -> u8 {
        255
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

    fn set_first(&mut self, value: u8) {
        self.blue = value
    }

    fn set_second(&mut self, value: u8) {
        self.green = value
    }

    fn set_third(&mut self, value: u8) {
        self.red = value
    }

    fn set_fourth(&mut self, value: u8) {
        println!("bmp.rs RgbPixel set_fourth called on a 3 byte pixel! This is a bug! Exiting");
        exit(1)
    }

    fn set_alpha(&mut self, value: u8) {
        println!("bmp.rs RgbPixel set_alpha called on a 3 byte pixel! This is a bug! Exiting");
        exit(1)
    }

    fn pixel_size(&self) -> usize {
        3
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

    fn first(&self) -> u8 {
        self.blue
    }

    fn second(&self) -> u8 {
        self.green
    }
    fn third(&self) -> u8 {
        self.red
    }
    fn fourth(&self) -> u8 {
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

    fn set_first(&mut self, value: u8) {
        self.blue = value
    }

    fn set_second(&mut self, value: u8) {
        self.green = value
    }

    fn set_third(&mut self, value: u8) {
        self.red = value
    }

    fn set_fourth(&mut self, value: u8) {
        self.alpha = value
    }

    fn set_alpha(&mut self, value: u8) {
        self.alpha = value
    }

    fn pixel_size(&self) -> usize {
        4
    }
}

/*
   We will just add support for 24 bit and 32 bit pixel sizes, will likely only encounter 24 bit pixels
*/

pub enum BmpPixelType {
    Rgb,
    Rgba,
}

impl FileEncodingSupport for BmpImageParser {
    fn new(filename : &str) -> Self {
        BmpImageParser{
            bmp_header: BitmapFileHeader {
                bf_type: 0,
                bf_size: 0,
                bf_reserved1: 0,
                bf_reserved2: 0,
                bf_off_bits: 0,
            },
            bmp_dib_header: BitmapDIBHeader {
                bi_size: 0,
                bi_width: 0,
                bi_height: 0,
                bi_planes: 0,
                bi_bit_count: 0,
                bi_compression: 0,
                bi_size_image: 0,
                bi_x_pels_per_meter: 0,
                bi_y_pels_per_meter: 0,
                bi_clr_used: 0,
                bi_clr_important: 0,
            },
            pixel_size: 0,
            padding_size: 0,
            pixel_map: BmpBitmap {
                width: 0,
                height: 0,
                pixel_map_start: 0,
            },
            image_file: match File::open(filename) {
                Ok(file) => {
                    file
                }
                Err(e) => {
                    println!("bmp.rs: new : failed to open file {}: {}",filename, e);
                    exit(1);
                }
            },
            file_data: Box::new(vec![]),
            ready: false,
        }
    }

    fn parse_file(&mut self) {
        let header_size = std::mem::size_of::<BitmapFileHeader>();
        let dib_header_size = std::mem::size_of::<BitmapDIBHeader>();

        let file: &mut File = &mut self.image_file;

        let file_size: u32 = match file.metadata() {
            Ok(metadata) => metadata.len() as u32,
            Err(_) => {
                println!(
                    "bmp.rs : parse_file: Error reading file metadata! Exiting...",
                );
                exit(1);
            }
        };

        self.file_data = Box::new(Vec::<u8>::with_capacity(file_size as usize));

        match file.read_to_end(&mut self.file_data) {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "bmp.rs: parse_file: Error reading file to end with err {}", e
                );
                exit(1);
            }
        }

        let header_size = std::mem::size_of::<BitmapFileHeader>();

        unsafe {
            let header_pointer: *mut BitmapFileHeader = &mut self.bmp_header;
            std::ptr::copy(
                &mut self.file_data[0] as *mut u8,
                header_pointer as *mut u8,
                header_size,
            );
        }
        let dib_header_size = mem::size_of::<BitmapDIBHeader>();

        unsafe {
            let dib_header_pointer: *mut BitmapDIBHeader = &mut self.bmp_dib_header;
            std::ptr::copy(
                &mut self.file_data[14] as *mut u8,
                dib_header_pointer as *mut u8,
                dib_header_size,
            );
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

        self.padding_size = ((self.pixel_map.width * self.pixel_size as u32) % 4) as u8;

        self.pixel_map.pixel_map_start = self.bmp_header.bf_off_bits as u64;

        self.ready = true;
    }

    fn embed_data(
        &mut self,
        data: &mut Vec<u8>,
        encoding: FileEncoding,
        encoding_method: FileEncodingMethod,
        file_encoding_function_derivation: FileEncodingFunctionDerivation,
    ) {
        if !self.ready {
            println!("bmp.rs: embed_data called with File Not Ready");
            exit(1);
        }
        match encoding_method {
            _ => todo!(),
        }
    }

    fn retrieve_data(
        &mut self,
        data: Vec<u8>,
        encoding: FileEncoding,
        encoding_method: FileEncodingMethod,
        file_encoding_function_derivation: FileEncodingFunctionDerivation,
    ) {
        if !self.ready {
            println!("bmp.rs: retrieve_data called with File Not Ready");
            exit(1);
        }
    }

    fn write_file(&mut self, file_location: &str) {
        if !self.ready {
            println!("bmp.rs: write_file called with File Not Ready");
            exit(1);
        }

        match File::create(file_location.to_string()) {
            Ok(_) => {}
            Err(e) => {
                println!("bmp.rs: write_file Error opening file {} {}", file_location,e);
                exit(1);
            }
        }
        let mut file = File::open(file_location.to_string()).expect("bmp.rs: Error opening file");

        match file.write_all(self.file_data.as_slice()) {
            Ok(_) => {
            }
            Err(e) => {
                println!("bmp.rs: write_file called with File Not Ready");
                exit(1);
            }
        }

    }


}
