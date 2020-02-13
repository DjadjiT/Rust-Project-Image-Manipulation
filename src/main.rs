use projectRust::image_struct::Image;
use std::path::Path;

fn main(){
    let pathPPM = Path::new("test.ppm");
    let pathTXT = Path::new("test.txt");
    let mut img = Image::new_with_file(pathPPM);

    img.to_string();
    println!("invert the image");
    img.invert();
    img.to_string();
    println!("Greyscale the image");
    img.grey_scale();
    img.to_string();
}