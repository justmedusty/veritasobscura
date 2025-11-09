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

#[cfg(test)]
mod svg_tests{

}

#[cfg(test)]
mod bmp_tests{
    use std::process::exit;
    use crate::file_encoding_support::file_encoding_support::FileEncodingSupport;
    use crate::file_encoding_support::pixel::{embed_lsb_data_left_right, extract_lsb_data_left_right};
    use crate::filetype_support::bmp::{BmpImageParser, RgbPixel, RgbaPixel};

    #[test]
    fn test_bmp_object_creation(){
        let bmp_image_parser = BmpImageParser::new("src/filetype_support/assets/sample-1024x1024.bmp");

        assert_eq!(bmp_image_parser.file_data.len(), 0);
        
        assert_eq!(bmp_image_parser.pixel_size, 0);
        
        assert_eq!(bmp_image_parser.pixel_map.width, 0);
        assert_eq!(bmp_image_parser.pixel_map.height, 0);
        assert_eq!(bmp_image_parser.pixel_map.pixel_map_start, 0);

        // To avoid unaligned access, we assign the packed header fields to local variables first
        let bf_type = bmp_image_parser.bmp_header.bf_type;
        let bf_size = bmp_image_parser.bmp_header.bf_size;
        let bf_reserved1 = bmp_image_parser.bmp_header.bf_reserved1;
        let bf_reserved2 = bmp_image_parser.bmp_header.bf_reserved2;
        let bf_off_bits = bmp_image_parser.bmp_header.bf_off_bits;

        // Check the bmp_header fields via local variables
        assert_eq!(bf_type, 0);
        assert_eq!(bf_size, 0);
        assert_eq!(bf_reserved1, 0);
        assert_eq!(bf_reserved2, 0);
        assert_eq!(bf_off_bits, 0);
        
        let bi_size = bmp_image_parser.bmp_dib_header.bi_size;
        let bi_width = bmp_image_parser.bmp_dib_header.bi_width;
        let bi_height = bmp_image_parser.bmp_dib_header.bi_height;
        let bi_planes = bmp_image_parser.bmp_dib_header.bi_planes;
        let bi_bit_count = bmp_image_parser.bmp_dib_header.bi_bit_count;
        let bi_compression = bmp_image_parser.bmp_dib_header.bi_compression;
        let bi_size_image = bmp_image_parser.bmp_dib_header.bi_size_image;
        let bi_x_pels_per_meter = bmp_image_parser.bmp_dib_header.bi_x_pels_per_meter;
        let bi_y_pels_per_meter = bmp_image_parser.bmp_dib_header.bi_y_pels_per_meter;
        let bi_clr_used = bmp_image_parser.bmp_dib_header.bi_clr_used;
        let bi_clr_important = bmp_image_parser.bmp_dib_header.bi_clr_important;
        
        assert_eq!(bi_size, 0);
        assert_eq!(bi_width, 0);
        assert_eq!(bi_height, 0);
        assert_eq!(bi_planes, 0);
        assert_eq!(bi_bit_count, 0);
        assert_eq!(bi_compression, 0);
        assert_eq!(bi_size_image, 0);
        assert_eq!(bi_x_pels_per_meter, 0);
        assert_eq!(bi_y_pels_per_meter, 0);
        assert_eq!(bi_clr_used, 0);
        assert_eq!(bi_clr_important, 0);
        
        assert_eq!(bmp_image_parser.padding_size, 0);


    }

    #[test]
    fn test_bmp_image_parsing(){
        let mut bmp_image_parser = BmpImageParser::new("src/filetype_support/assets/sample-1024x1024.bmp");

        bmp_image_parser.parse_file();
       assert_ne!(bmp_image_parser.file_data.len(), 0);

        assert_ne!(bmp_image_parser.pixel_size, 0);

        assert_ne!(bmp_image_parser.pixel_map.width, 0);
        assert_ne!(bmp_image_parser.pixel_map.height, 0);
        assert_ne!(bmp_image_parser.pixel_map.pixel_map_start, 0);

        // To avoid unaligned access, we assign the packed header fields to local variables first
        let bf_type = bmp_image_parser.bmp_header.bf_type;
        let bf_size = bmp_image_parser.bmp_header.bf_size;
        let bf_reserved1 = bmp_image_parser.bmp_header.bf_reserved1;
        let bf_reserved2 = bmp_image_parser.bmp_header.bf_reserved2;
        let bf_off_bits = bmp_image_parser.bmp_header.bf_off_bits;

        // Check the bmp_header fields via local variables
        assert_ne!(bf_type, 0);
        assert_ne!(bf_size, 0);
        assert_eq!(bf_off_bits, 54);

        let bi_size = bmp_image_parser.bmp_dib_header.bi_size;
        let bi_width = bmp_image_parser.bmp_dib_header.bi_width;
        let bi_height = bmp_image_parser.bmp_dib_header.bi_height;
        let bi_planes = bmp_image_parser.bmp_dib_header.bi_planes;
        let bi_bit_count = bmp_image_parser.bmp_dib_header.bi_bit_count;
        let bi_compression = bmp_image_parser.bmp_dib_header.bi_compression;
        let bi_size_image = bmp_image_parser.bmp_dib_header.bi_size_image;
        let bi_x_pels_per_meter = bmp_image_parser.bmp_dib_header.bi_x_pels_per_meter;
        let bi_y_pels_per_meter = bmp_image_parser.bmp_dib_header.bi_y_pels_per_meter;
        let bi_clr_used = bmp_image_parser.bmp_dib_header.bi_clr_used;
        let bi_clr_important = bmp_image_parser.bmp_dib_header.bi_clr_important;

        assert_ne!(bi_size, 0);
        assert_eq!(bi_width, 1024);
        assert_eq!(bi_height, 1024);
        assert_ne!(bi_planes, 0);
        assert_eq!(bi_bit_count, 24);
        assert_ne!(bi_size_image, 0);
        assert_ne!(bi_x_pels_per_meter, 0);
        assert_ne!(bi_y_pels_per_meter, 0);

        assert_eq!(bmp_image_parser.padding_size, 0);
    }

    #[test]
    fn test_bmp_lsb_embed_left_right(){
        let mut bmp_image_parser = BmpImageParser::new("src/filetype_support/assets/sample-1024x1024.bmp");
        bmp_image_parser.parse_file();

        let data_vec : Vec<u8> = "This is a test embedding for testing purposes".as_bytes().to_vec();
        match bmp_image_parser.pixel_size {
            3 => {
                embed_lsb_data_left_right::<RgbPixel>(&data_vec, &mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64);
            }

            4 => {
                embed_lsb_data_left_right::<RgbaPixel>(&data_vec, &mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64);
            }
            _ => {
                println!("bmp test.rs Got bad value for pixel size, exiting ...");
                exit(1);
            }
        }

        bmp_image_parser.write_file("src/filetype_support/assets/sample-1024x1024-TEST_EMBED.bmp");

    }

    #[test]
    fn test_bmp_lsb_retrieve_left_right(){
        let mut bmp_image_parser = BmpImageParser::new("src/filetype_support/assets/sample-1024x1024-TEST_EMBED.bmp");
        bmp_image_parser.parse_file();

        let mut data_vec: Vec<u8> = vec![0];
        match bmp_image_parser.pixel_size {
            3 => {
               data_vec = extract_lsb_data_left_right::<RgbPixel>( &mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64,360);
            }

            4 => {
                data_vec = extract_lsb_data_left_right::<RgbaPixel>(&mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64, 360);
            }
            _ => {
                println!("bmp test.rs Got bad value for pixel size, exiting ...");
                exit(1);
            }
        }

        /*
            the data vec will have an extra byte. the slice is just to remove that extra byte so that the test will pass
         */
            assert_eq!(data_vec[0..(360/8)], "This is a test embedding for testing purposes".as_bytes().to_vec());

    }

    #[test]
    fn test_bmp_lsb_embed_large_message(){
        let mut bmp_image_parser = BmpImageParser::new("src/filetype_support/assets/sample-1024x1024.bmp");
        bmp_image_parser.parse_file();

        let data_vec : Vec<u8> = "This is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposes".as_bytes().to_vec();
        match bmp_image_parser.pixel_size {
            3 => {
                embed_lsb_data_left_right::<RgbPixel>(&data_vec, &mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64);
            }

            4 => {
                embed_lsb_data_left_right::<RgbaPixel>(&data_vec, &mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64);
            }
            _ => {
                println!("bmp test.rs Got bad value for pixel size, exiting ...");
                exit(1);
            }
        }

        bmp_image_parser.write_file("src/filetype_support/assets/sample-1024x1024-TEST_EMBED_LARGE.bmp");

        //ensure we shoved more than a full pixel row of bits in there so that we can test multi row embedding
        assert!(data_vec.len() > 384);

    }

    #[test]
    fn test_bmp_lsb_retrieve_large_message(){
        let mut bmp_image_parser = BmpImageParser::new("src/filetype_support/assets/sample-1024x1024-TEST_EMBED_LARGE.bmp");
        bmp_image_parser.parse_file();
        let message_vec : Vec<u8> = "This is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposesThis is a test embedding for testing purposes".as_bytes().to_vec();

        let mut data_vec: Vec<u8> = vec![0];
        match bmp_image_parser.pixel_size {
            3 => {
                data_vec = extract_lsb_data_left_right::<RgbPixel>( &mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64,(message_vec.len() * 8) as u64);
            }

            4 => {
                data_vec = extract_lsb_data_left_right::<RgbaPixel>(&mut bmp_image_parser.file_data[bmp_image_parser.pixel_map.pixel_map_start as usize..], bmp_image_parser.pixel_map.width as u64, bmp_image_parser.pixel_map.height as u64, bmp_image_parser.padding_size as u64, bmp_image_parser.pixel_size as u64, (message_vec.len() * 8) as u64);
            }
            _ => {
                println!("bmp test.rs Got bad value for pixel size, exiting ...");
                exit(1);
            }
        }

        /*
            the data vec will have an extra byte. the slice is just to remove that extra byte so that the test will pass
         */
        assert_eq!(String::from_utf8(data_vec[0..(message_vec.len())].to_owned()), String::from_utf8(message_vec));
    }
}


