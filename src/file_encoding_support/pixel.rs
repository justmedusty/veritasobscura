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
use crate::file_encoding_support::file_encoding_support::WaveFunction;
use std::ops::{AddAssign, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

pub trait Pixel {
    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;

    /*
       This should just return 255 if this particular pixel does not support alpha
    */
    fn alpha(&self) -> u8;

    /*
       These will be for embedding just the first , second etc color value irrespective of which order the colors are
    */
    fn first(&self) -> u8;
    fn second(&self) -> u8;
    fn third(&self) -> u8;

    fn fourth(&self) -> u8; // This one should just return 255 if not in use

    fn set_red(&mut self, value: u8);
    fn set_green(&mut self, value: u8);
    fn set_blue(&mut self, value: u8);

    fn set_first(&mut self, value: u8);
    fn set_second(&mut self, value: u8);
    fn set_third(&mut self, value: u8);

    // This should do nothing or exit / panic on invocation from a 3 byte pixel
    fn set_fourth(&mut self, value: u8);
    /*
       This should do nothing on a 3 byte pixel impl
    */
    fn set_alpha(&mut self, value: u8);

    fn pixel_size(&self) -> usize;
}

pub fn transform_pixels<P, F>(pixel_map: &mut Vec<P>, transform_function: F)
where
    P: Pixel,
    F: Fn(&mut P),
{
    for pixel in pixel_map.iter_mut() {
        transform_function(pixel);
    }
}

pub fn transform_pixel_quadrants<P, F>(
    pixel_map: &mut Vec<P>,
    transform_function: F,
    coordinates: (u64, u64),
    quadrant_size: u64,
) where
    P: Pixel + Sized,
    F: Fn(&mut [P]),
{
    let start_index = (coordinates.0 as usize * coordinates.1 as usize) as usize;
    let end_index = start_index + quadrant_size as usize;

    let quadrant_slice = &mut pixel_map[start_index..end_index];

    transform_function(quadrant_slice);
}

pub fn increment_bit_and_byte_counters(bit: &mut u32, byte: &mut u32) {
    *bit += 1;
    if *bit == 8 {
        *byte += 1;
        *bit = 0;
    }
}

fn embed_pixel_lsb<P: Pixel>(
    pixel: &mut P,
    current_bit: &mut u32,
    current_byte: &mut u32,
    data: &Vec<u8>,
    bits_to_embed: &mut usize,
) {
    let mut bit: u8 = data[*current_byte as usize] & (1 << *current_bit);

    if bit == 0 {
        pixel.set_first(pixel.first() & !1);
    } else {
        pixel.set_first(pixel.first() | 1);
    }

    increment_bit_and_byte_counters(current_bit, current_byte);
    bits_to_embed.sub_assign(1);

    if *bits_to_embed == 0 {
        return;
    }

    bit = data[*current_byte as usize] & (1 << *current_bit);

    if bit == 0 {
        pixel.set_second(pixel.second() & !1);
    } else {
        pixel.set_second(pixel.second() | 1);
    }

    increment_bit_and_byte_counters(current_bit, current_byte);
    bits_to_embed.sub_assign(1);

    if *bits_to_embed == 0 {
        return;
    }

    bit = data[*current_byte as usize] & (1 << *current_bit);

    if bit == 0 {
        pixel.set_third(pixel.third() & !1);
    } else {
        pixel.set_third(pixel.third() | 1);
    }
    increment_bit_and_byte_counters(current_bit, current_byte);
    bits_to_embed.sub_assign(1);

    if *bits_to_embed == 0 {
        return;
    }

    if pixel.pixel_size() == 4 {
        bit = data[*current_byte as usize] & (1 << *current_bit);
        if bit == 0 {
            pixel.set_fourth(pixel.fourth() & !1);
        } else {
            pixel.set_fourth(pixel.fourth() | 1);
        }

        increment_bit_and_byte_counters(current_bit, current_byte);
        bits_to_embed.sub_assign(1);
    }
}

fn extract_pixel_lsb<P: Pixel>(
    pixel: &P,
    bits: &mut u32,
    bytes: &mut u32,
    extracted_data: &mut Vec<u8>,
    embedded_bits: usize,
) {
    let mut current_bit = pixel.first() & 1;

    if current_bit == 0 {
        extracted_data[*bytes as usize] &= !(1 << *bits);
    } else {
        extracted_data[*bytes as usize] |= 1 << *bits;
    }

    increment_bit_and_byte_counters(bits, bytes);

    if *bits + (*bytes * 8) == embedded_bits as u32 {
        return;
    }

    current_bit = pixel.second() & 1;

    if current_bit == 0 {
        extracted_data[*bytes as usize] &= !(1 << *bits);
    } else {
        extracted_data[*bytes as usize] |= 1 << *bits;
    }

    increment_bit_and_byte_counters(bits, bytes);

    if *bits + (*bytes * 8) == embedded_bits as u32 {
        return;
    }

    current_bit = pixel.third() & 1;

    if current_bit == 0 {
        extracted_data[*bytes as usize] &= !(1 << *bits);
    } else {
        extracted_data[*bytes as usize] |= 1 << *bits;
    }

    increment_bit_and_byte_counters(&mut *bits, &mut *bytes);

    if *bits + (*bytes * 8) == embedded_bits as u32 {
        return;
    }

    if pixel.pixel_size() == 4 {
        current_bit = pixel.fourth() & 1;

        if current_bit == 0 {
            extracted_data[*bytes as usize] &= !(1 << *bits);
        } else {
            extracted_data[*bytes as usize] |= 1 << *bits;
        }

        increment_bit_and_byte_counters(bits, bytes);

        if *bits + (*bytes * 8) == embedded_bits as u32 {
            return;
        }
    }
}

fn embed_pixel_color<P: Pixel>(
    pixel: &mut P,
    current_bit: &mut u32,
    current_byte: &mut u32,
    data: &Vec<u8>,
    bits_to_embed: &mut usize,
) {

    let bit = data[*current_byte as usize] & (1 << *current_bit);
    let mut ones = 0;

    let first = pixel.first();
    let second = pixel.second();
    let third = pixel.third();
    let fourth = match pixel.pixel_size() {
        3 => 0,
        4 => pixel.fourth(),
        _ => unreachable!(),
    };

    for i in 0..8 {
        if(first & (1 << i)) != 0 {
            ones.add_assign(1);
        }
        if(second & (1 << i)) != 0 {
            ones.add_assign(1);
        }
        if(third & (1 << i)) != 0 {
            ones.add_assign(1);
        }

        if(fourth & (1 << i)) != 0 {
            ones.add_assign(1);
        }

    }

    if(bit == 0 && ones % 2 == 0){
        for i in 0..8 {
            if(first & (1 << i)) != 0 {
                ones.add_assign(1);
            }
            if(second & (1 << i)) != 0 {
                ones.add_assign(1);
            }
            if(third & (1 << i)) != 0 {
                ones.add_assign(1);
            }

            if(fourth & (1 << i)) != 0 {
                ones.add_assign(1);
            }

        }
    }else if(bit == 1 && ones % 2 != 0){
        for i in 0..8 {
            if(first & (1 << i)) != 0 {
                ones.add_assign(1);
            }
            if(second & (1 << i)) != 0 {
                ones.add_assign(1);
            }
            if(third & (1 << i)) != 0 {
                ones.add_assign(1);
            }

            if(fourth & (1 << i)) != 0 {
                ones.add_assign(1);
            }

        }
    }

    increment_bit_and_byte_counters(current_bit, current_byte);
    bits_to_embed.sub_assign(1);
}

fn extract_pixel_color<P: Pixel>(
    pixel: &P,
    bits: &mut u32,
    bytes: &mut u32,
    extracted_data: &mut Vec<u8>,
    embedded_bits: usize,
) {


    if current_bit {
        extracted_data[*bytes as usize] |= 1 << *bits;
    } else {
        extracted_data[*bytes as usize] &= !(1 << *bits);
    }

    increment_bit_and_byte_counters(bits, bytes);
    if *bits + (*bytes * 8) == embedded_bits as u32 {
        return;
    }
}

pub fn embed_color_data_left_right<P: Pixel>(
    data: &Vec<u8>,
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
) {
    let total_length = (width + padding) * length;

    let mut bits_to_embed = data.len() * 8;

    let mut current_byte: u32 = 0;
    let mut current_bit: u32 = 0;

    if bits_to_embed > (width * length) as usize {
        panic!(
            "Not enough space in the image to embed {bits_to_embed} bits! Only have {} bits available!",
            width * length * pixel_size_bytes
        )
    }

    for row in 0..length as usize {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &mut [P] =
            unsafe { std::slice::from_raw_parts_mut(row_pixels_ptr, width as usize) };

        for pixel in row_pixels.iter_mut() {
            embed_pixel_color(
                pixel,
                &mut current_bit,
                &mut current_byte,
                &data,
                &mut bits_to_embed,
            );

            if bits_to_embed == 0 {
                return;
            }
        }
    }
}

pub fn extract_color_data_left_right<P: Pixel>(
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
    embedded_bits: u64,
) -> Vec<u8> {
    let total_length = (width + padding) * length;

    let mut bytes: u32 = 0;
    let mut bits: u32 = 0;

    //The plus one is just in case we have a sub byte number of bits , we would lose those few bits or write into an invalid offset
    let mut extracted_data: Vec<u8> = vec![0u8; ((embedded_bits as usize / 8) + 1) as usize];

    for row in 0..length as usize {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &[P] =
            unsafe { std::slice::from_raw_parts(row_pixels_ptr, width as usize) };

        let mut current_bit: u8 = 0;

        for pixel in row_pixels.iter() {
            extract_pixel_color(
                pixel,
                &mut bits,
                &mut bytes,
                &mut extracted_data,
                embedded_bits as usize,
            );

            if bits + (bytes * 8) == embedded_bits as u32 {
                break;
            }
        }

        if bits + (bytes * 8) == embedded_bits as u32 {
            break;
        }
    }

    extracted_data
}

pub fn embed_color_data_right_left<P: Pixel>(
    data: &Vec<u8>,
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
) {
    let total_length = (width + padding) * length;

    let mut bits_to_embed = data.len() * 8;

    let mut current_byte: u32 = 0;
    let mut current_bit: u32 = 0;

    if bits_to_embed > (width * length) as usize {
        panic!(
            "Not enough space in the image to embed {bits_to_embed} bits! Only have {} bits available!",
            width * length * pixel_size_bytes
        )
    }

    for row in (0..length).rev() {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &mut [P] =
            unsafe { std::slice::from_raw_parts_mut(row_pixels_ptr, width as usize) };

        for pixel in row_pixels.iter_mut().rev() {
            embed_pixel_color(
                pixel,
                &mut current_bit,
                &mut current_byte,
                &data,
                &mut bits_to_embed,
            );

            if bits_to_embed == 0 {
                return;
            }
        }
    }
}

pub fn extract_color_data_right_left<P: Pixel>(
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
    embedded_bits: u64,
) -> Vec<u8> {
    let total_length = (width + padding) * length;

    let mut bytes: u32 = 0;
    let mut bits: u32 = 0;

    //The plus one is just in case we have a sub byte number of bits , we would lose those few bits or write into an invalid offset
    let mut extracted_data: Vec<u8> = vec![0u8; ((embedded_bits as usize / 8) + 1) as usize];

    for row in (0..length).rev() {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &[P] =
            unsafe { std::slice::from_raw_parts(row_pixels_ptr, width as usize) };

        let mut current_bit: u8 = 0;

        for pixel in row_pixels.iter().rev() {
            extract_pixel_color(
                pixel,
                &mut bits,
                &mut bytes,
                &mut extracted_data,
                embedded_bits as usize,
            );

            if bits + (bytes * 8) == embedded_bits as u32 {
                break;
            }
        }

        if bits + (bytes * 8) == embedded_bits as u32 {
            break;
        }
    }

    extracted_data
}
pub fn embed_lsb_data_left_right<P: Pixel>(
    data: &Vec<u8>,
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
) {
    let total_length = (width + padding) * length;

    let mut bits_to_embed = data.len() * 8;

    let mut current_byte: u32 = 0;
    let mut current_bit: u32 = 0;

    if bits_to_embed > (width * length * pixel_size_bytes) as usize {
        panic!(
            "Not enough space in the image to embed {bits_to_embed} bits! Only have {} bits available!",
            width * length * pixel_size_bytes
        )
    }

    for row in 0..length as usize {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &mut [P] =
            unsafe { std::slice::from_raw_parts_mut(row_pixels_ptr, width as usize) };

        for pixel in row_pixels.iter_mut() {
            embed_pixel_lsb(
                pixel,
                &mut current_bit,
                &mut current_byte,
                &data,
                &mut bits_to_embed,
            );

            if bits_to_embed == 0 {
                return;
            }
        }
    }
}

pub fn extract_lsb_data_left_right<P: Pixel>(
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
    embedded_bits: u64,
) -> Vec<u8> {
    let total_length = (width + padding) * length;

    let mut bytes: u32 = 0;
    let mut bits: u32 = 0;

    //The plus one is just in case we have a sub byte number of bits , we would lose those few bits or write into an invalid offset
    let mut extracted_data: Vec<u8> = vec![0u8; ((embedded_bits as usize / 8) + 1) as usize];

    for row in 0..length as usize {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &[P] =
            unsafe { std::slice::from_raw_parts(row_pixels_ptr, width as usize) };

        let mut current_bit: u8 = 0;

        for pixel in row_pixels.iter() {
            extract_pixel_lsb(
                pixel,
                &mut bits,
                &mut bytes,
                &mut extracted_data,
                embedded_bits as usize,
            );

            if bits + (bytes * 8) == embedded_bits as u32 {
                break;
            }
        }

        if bits + (bytes * 8) == embedded_bits as u32 {
            break;
        }
    }

    extracted_data
}

pub fn embed_lsb_data_right_left<P: Pixel>(
    data: &Vec<u8>,
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
) {
    let total_length = (width + padding) * length;

    let mut bits_to_embed = data.len() * 8;

    let mut current_byte: u32 = 0;
    let mut current_bit: u32 = 0;

    if bits_to_embed > (width * length * pixel_size_bytes) as usize {
        panic!(
            "Not enough space in the image to embed {bits_to_embed} bits! Only have {} bits available!",
            width * length * pixel_size_bytes
        )
    }

    for row in (0..length).rev() {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &mut [P] =
            unsafe { std::slice::from_raw_parts_mut(row_pixels_ptr, width as usize) };

        for pixel in row_pixels.iter_mut().rev() {
            embed_pixel_lsb(
                pixel,
                &mut current_bit,
                &mut current_byte,
                &data,
                &mut bits_to_embed,
            );

            if bits_to_embed == 0 {
                return;
            }
        }
    }
}

pub fn extract_lsb_data_right_left<P: Pixel>(
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
    embedded_bits: u64,
) -> Vec<u8> {
    let total_length = (width + padding) * length;

    let mut bytes: u32 = 0;
    let mut bits: u32 = 0;

    //The plus one is just in case we have a sub byte number of bits , we would lose those few bits or write into an invalid offset
    let mut extracted_data: Vec<u8> = vec![0u8; ((embedded_bits as usize / 8) + 1) as usize];

    for row in (0..length).rev() {
        let start = (width + padding) * pixel_size_bytes * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { pixel_map.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &[P] =
            unsafe { std::slice::from_raw_parts(row_pixels_ptr, width as usize) };

        for pixel in row_pixels.iter().rev() {
            extract_pixel_lsb(
                pixel,
                &mut bits,
                &mut bytes,
                &mut extracted_data,
                embedded_bits as usize,
            );
            if bits + (bytes * 8) == embedded_bits as u32 {
                break;
            }
        }

        if bits + (bytes * 8) == embedded_bits as u32 {
            break;
        }
    }

    extracted_data
}

pub fn embed_lsb_wave_function_left_right<P: Pixel>(
    pixel_map: &mut [u8],
    width: u64,
    length: u64,
    padding: u64,
    pixel_size_bytes: u64,
    embedded_bits: u64,
    wave_function: WaveFunction,
) {
    let points = WaveFunction::traverse(&wave_function, width as usize, length as usize);

    for point in points {
        let x: u64 = point.0 as u64;
        let y: u64 = point.1 as u64;

        let offset = unsafe {
            pixel_map
                .as_mut_ptr()
                .add((((width + padding) * x) + (y * length)) as usize)
        };
    }
}
