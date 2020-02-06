use projectRust::image_struct::Image;
use std::path::Path;

fn main(){
    let pathPPM = Path::new("mandelbrot.ppm");
    let pathTXT = Path::new("test.txt");
    let img = Image::new_with_file(pathPPM);
    let imgtxt = Image::new_with_file(pathTXT);

    img.toString();
}