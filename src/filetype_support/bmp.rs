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
use crate::file_encoding_support::file_encoding_support::*;


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
    pub bi_size: u32,          // Size of this header (40 bytes)
    pub bi_width: i32,         // Width of bitmap in pixels
    pub bi_height: i32,        // Height of bitmap in pixels
    pub bi_planes: u16,        // Number of color planes (must be 1)
    pub bi_bit_count: u16,     // Bits per pixel (e.g., 24 = RGB)
    pub bi_compression: u32,   // Compression type (0 = none)
    pub bi_size_image: u32,    // Image size (may be 0 for uncompressed)
    pub bi_x_pels_per_meter: i32, // Horizontal resolution
    pub bi_y_pels_per_meter: i32, // Vertical resolution
    pub bi_clr_used: u32,      // Number of colors in the color palette
    pub bi_clr_important: u32, // Number of important colors used
    pub bi_clr_space_endpoints : [u8;36],
    pub bi_gamma_red_channel : u32,
    pub bi_gamma_green_channel : u32,
    pub bi_gamma_blue_channel : u32,
    pub bi_intent : u32,
    pub bi_icc_profile_data : u32,
    pub bi_icc_profile_size : u32,
    pub bi_reserved : u32,
}


/*
    Presence is mandatory when bits per pixel is <= 8

    The size of color table entries is 3 bytes if BITMAPCOREHEADER is
    substituted for BITMAPV5HEADER
 */
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct BitmapColorTable {

}

// For a 24-bit BMP, a pixel is usually 3 bytes: B, G, R.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct BgrPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

