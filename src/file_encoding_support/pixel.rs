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

pub fn transform_pixel_quadrants<P, F>(pixel_map: &mut Vec<P>, transform_function: F, coordinates: (u64, u64), quadrant_size: u64)
where
    P: Pixel + Sized,
    F: Fn(&mut [P]),
{
    let start_index = (coordinates.0 as usize * coordinates.1 as usize) as usize;
    let end_index = start_index + quadrant_size as usize;

    let quadrant_slice = &mut pixel_map[start_index..end_index];

    transform_function(quadrant_slice);
}

pub fn increment_bit_and_byte_counters(bit: &mut u32, byte : &mut u32, total_bytes : u64){
    *bit += 1;
    if(*bit == 8){
        *byte += 1;
        *bit = 0;
    }
}
pub fn embed_lsb_data<P: Pixel>(data: &Vec<u8>, pixel_map: &mut [P], width : u64, length : u64, padding : u64, pixel_size_bytes : u64){
    let total_length = (width + padding) * length;

    let bits_to_embed = data.len() * 8;
    let bytes: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(
            pixel_map.as_mut_ptr() as *mut u8,
            total_length as usize,
        )
    };
    let mut current_byte : u32 = 0;
    let mut current_bit : u32 = 0;

    for row in 0..length as usize{
        let start = (width + padding) * row as u64;
        let end = start + (width * pixel_size_bytes);
        let row_start_ptr = unsafe { bytes.as_mut_ptr().add(start as usize) };
        let row_pixels_ptr = row_start_ptr as *mut P;
        let row_pixels: &mut [P] = unsafe {
            std::slice::from_raw_parts_mut(row_pixels_ptr, width as usize)
        };

        for pixel in row_pixels.iter_mut(){
            let mut bit: u8 = data[current_byte as usize] & (1 << current_bit);
            increment_bit_and_byte_counters(&mut current_bit, &mut current_byte, bits_to_embed as u64);
            pixel.set_first(pixel.first() & bit);

            bit = data[current_byte as usize] & (1 << current_bit);
            increment_bit_and_byte_counters(&mut current_bit, &mut current_byte, bits_to_embed as u64);
            pixel.set_second(pixel.second() & bit);


        }
    }




}
/*
    Ret val is how many bits were embedded so it is known how many more need to be embedded in the next pixel transformation

    under construction this is not usable at all yet
 */
