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
use std::fs::File;
use crate::file_encoding_support::file_encoding_support;
use crate::file_encoding_support::file_encoding_support::FileEncodingSupport;
const BMP_MAGIC : u16 = 0x4D42;
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
    pub bi_size: u32,               // Size of this header (40 bytes, for BITMAPINFOHEADER)
    pub bi_width: i32,              // Width of bitmap in pixels
    pub bi_height: i32,             // Height of bitmap in pixels. If positive, bottom-up. If negative, top-down.
    pub bi_planes: u16,             // Number of color planes (must be 1 for all bitmaps)
    pub bi_bit_count: u16,          // Bits per pixel (e.g., 24 for RGB, 1 for monochrome)
    pub bi_compression: u32,        // Compression type (e.g., 0 = none, 1 = BI_RLE8, 2 = BI_RLE4)
    pub bi_size_image: u32,         // Image size in bytes (may be 0 for uncompressed images)
    pub bi_x_pels_per_meter: i32,  // Horizontal resolution of the image (pixels per meter)
    pub bi_y_pels_per_meter: i32,  // Vertical resolution of the image (pixels per meter)
    pub bi_clr_used: u32,           // Number of colors in the color palette (0 = default, 2^bi_bit_count colors)
    pub bi_clr_important: u32,     // Number of important colors (0 = all colors are important)
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
    pub blue: u8,    // Blue component (0-255)
    pub green: u8,   // Green component (0-255)
    pub red: u8,     // Red component (0-255)
    pub reserved: u8, // Reserved (must be 0)
}


// For a 24-bit BMP, a pixel is usually 3 bytes: B, G, R.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct RgbPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

// For a 32-bit BMP, a pixel is 4 bytes: B, G, R, A (or reserved)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
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
    pub pixel_map : Vec<P>
}

pub struct BmpImageParser<P>{
    bmp_header : BitmapFileHeader,
    bmp_dib_header: BitmapDIBHeader,
    pixel_size: u8,
    pixel_map : BmpBitmap<P>
}


/*
    We will just add support for 24 bit and 32 bit pixel sizes, will likely only encounter 24 bit pixels
 */
impl BmpImageParser<RgbaPixel> {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_map = vec![RgbaPixel { red: 0, green: 0, blue: 0, alpha: 255 }; (width * height) as usize];

        BmpImageParser {
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
            pixel_size: 4,  // RgbPixel is 4 bytes (1 byte each for R, G, B, A)
            pixel_map: BmpBitmap {
                width,
                height,
                pixel_map,
            },
        }
    }
}

impl BmpImageParser<RgbPixel> {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_map = vec![RgbPixel { red: 0, green: 0, blue: 0 }; (width * height) as usize];

        BmpImageParser {
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
            pixel_size: 3,  // RgbPixel is 3 bytes (1 byte each for R, G, B)
            pixel_map: BmpBitmap {
                width,
                height,
                pixel_map,
            },
        }
    }
}



impl<P> FileEncodingSupport for BmpImageParser<P> {
    fn parse_file(&mut self, file_location: &str) {
        todo!()
    }

    fn embed_data(&mut self, data: &mut Vec<u8>) {
        todo!()
    }

    fn retrieve_data(&mut self) {
        todo!()
    }

    fn write_file(&mut self, file: &mut File, location: &str) {
        todo!()
    }
}


