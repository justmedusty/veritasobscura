/*
    Copyright (C) 2025 Dustyn Gibb

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

pub mod arg_handling {
    const ERROR : i32 = 1;
    const SUCCESS : i32 = 0;
    use std::process::exit;
    use crate::file_encoding_support;
    use crate::file_encoding_support::file_encoding_support::{FileEncodingSupport, ImageSupport};


    pub fn parse_arguments<T: file_encoding_support::file_encoding_support::FileEncodingAlgorithms + file_encoding_support::file_encoding_support::FileEncodingSupport>(args: Vec<String>) -> ImageSupport<T> {
        let use_key: bool = args.len() == 5;
        if (args.len() > 5) {
            println!("Too many arguments!");
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("Try --help for help.");
            exit(ERROR);
        }

        if { args.len() < 2 } {
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("Try --help for help.");
            exit(ERROR);
        }
        if (args[1] == "--help") {
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("This is a stegonagraphy tool for embedding and extracting secret messages within images.");
            println!("Options: --help, --version");
            exit(SUCCESS);
        }

        if (args[1] == "--version") {
            println!("Maya version {}", env!("CARGO_PKG_VERSION"));
            exit(SUCCESS);
        }

        if { args.len() < 4 } {
            println!("Usage: maya encoding(Lsb,PixelValueDifferencing,Hamming) encoding-method(LeftRight, TopBottom, SinWave,CosWave, PolyFunc, FractalFunc) operation(embed/extract) <optional>'Message to be hidden'</optional> filename.ext(either the file to extract or the filename to embed into)");
            println!("Try --help for help.");
            exit(ERROR);
        }

        todo!()





    }
}