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