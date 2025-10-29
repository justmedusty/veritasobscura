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

use std::fs::File;
use std::io::Error;
use std::rc::Rc;
use crate::file_encoding_support::pixel::Pixel;

pub struct ImageSupport<T: FileEncodingSupport + FileEncodingAlgorithms> {
    image_file: File,
    encoding: FileEncoding,
    encoding_method: FileEncodingMethod,
    file_encoding_function_derivation: FileEncodingFunctionDerivation,
    operation: Operation,
    data : Vec<u8>,
    encoding_support: T,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Embed,
    Extract
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileEncoding {
    Lsb,
    PixelValueDifferencing,
    HammingMatrix,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileEncodingMethod {
    LeftToRight,
    TopToBottom,
    SinWave,
    CosWave,
    PolynomialFunction,
    FractalFunction,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FileEncodingFunctionDerivation {
    KeyBased,
}

pub trait FileEncodingSupport {
    fn new(filename : &str) -> Self;

    fn parse_file(&mut self, file_location: &str);

    fn embed_data(&mut self, data: &mut Vec<u8>, encoding: FileEncoding, encoding_method: FileEncodingMethod, file_encoding_function_derivation: FileEncodingFunctionDerivation);

    fn retrieve_data(&mut self,data: Vec<u8>,encoding: FileEncoding, encoding_method: FileEncodingMethod, file_encoding_function_derivation: FileEncodingFunctionDerivation);

    fn write_file(&mut self, file_location: &str);
    
}

pub enum WaveType {
    Sine,
    Cosine,
}

/*
    The idea at the time of writing this is the other parameters like the FileEncoding as defined above, will be stored in the specific object and can be referenced internally
 */
pub trait FileEncodingAlgorithms{
    fn left_to_right(&self);
    fn top_to_bottom(&self);
    fn wave(&self, wave_type: WaveType, amplitude: f32, phase: f32, frequency: f32);
}
