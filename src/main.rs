use projectRust::image_struct::Image;
use std::path::Path;

fn main(){
    let pathPPM = Path::new("test.ppm");
    let pathTXT = Path::new("test.txt");
    let img = Image::new_with_file(pathPPM);

    img.toString();
}