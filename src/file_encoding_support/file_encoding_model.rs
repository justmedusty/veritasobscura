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
use std::io::Error;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum FileEncoding{
    Lsb,
    PixelValueDifferencing,
    HammingMatrix
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum FileEncodingMethod{
    LeftToRight,
    TopToBottom,
    KeyBasedSinWave,
    KeyBasedCosWave,
    KeyBasedPolynomialFunction,
    KeyBasedFractalFunction,
}


pub trait FileEncodingSupport{
    /*
        Will return pixel map or will return error
     */
    fn parse_file(&mut self, &str : file_location) -> Result<Error,Vec<u8>>;
    fn embed_data(&mut self, data : &mut Vec<u8>);
}