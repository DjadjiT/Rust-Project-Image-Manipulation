use projectRust::image_struct::Image;
use std::path::Path;

fn main(){
    let pathPPM = Path::new("test.ppm");
    let mut img = Image::new_with_file(pathPPM).unwrap();
    let mut img2 = Image::new_with_file(pathPPM).unwrap();

    img.to_string();
    println!("invert the image");
    img.invert();
    img.to_string();
    println!("Greyscale the image");
    img2.greyscale();
    img2.to_string();
}