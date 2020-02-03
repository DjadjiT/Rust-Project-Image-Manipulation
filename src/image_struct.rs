use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::str::FromStr;


    
#[derive(Clone, Debug, Copy)]
pub struct Pixel{
    r : u8,
    g : u8,
    b : u8,
}

impl Pixel{
    fn init() -> Pixel {
        return Pixel{
            r:0,
            g:0,
            b:0,
        }
    }

    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        return Pixel{
            r:red,
            g:green,
            b:blue,
        }
    }

    fn display(&self)-> String{
        return format!("{}-{}-{}", &self.r, &self.g,&self.b);
    }

    fn invert(&mut self){
        self.r = 255-&self.r;
        self.g = 255-&self.g;
        self.b = 255-&self.b;
    }

    fn greyScale(&mut self){
        let average : u8 = (&self.r+&self.g+&self.b)/3;
        self.r = average;
        self.g = average;
        self.b = average;
    }
}

impl PartialEq for Pixel{
    fn eq(&self, other : &Pixel) -> bool{
        if self.r == other.r &&
        self.b == other.b &&
        self.g == other.g {
            return true;
        }else {
            return false;
        }
    }
}

pub struct Image {
    height : usize,
    width : usize,
    pixels : Vec<Pixel>,
}

impl Image {
    pub fn new(h : &usize, w : &usize)-> Image {
        return Image {
                height : *h,
                width : *w,
                pixels : vec![Pixel::init(); w*h]
        }
    }

    pub fn new_with_file(filename: &Path) -> Image {
        if filename.is_file() && filename.extension().unwrap()=="ppm" {
            let mut init : bool = false;
            let mut file = match File::open(&filename) {
                Err(e) => panic!("couldn't open file : {}", e),
                Ok(file) => file,
            };
            let mut img : Image = Image {
                height : 0,
                width : 0,
                pixels : vec![Pixel::init(); 0]
            };

            let mut buf_reader = BufReader::new(file);
            for line in buf_reader.lines() {
                let l = line.unwrap();
                if getCharsAtIndex(&l, 0)!='#'{
                    let strList = l.split_whitespace();
                    let vec = strList.collect::<Vec<&str>>();
                    match vec.len() {
                        1 => {
                            if getCharsAtIndex(&String::from(vec[0]), 0)=='P' {
                                println!("Format : {}", vec[0]);
                            }else {
                                println!("maximum value for each color : {} ", vec[0]);
                                if u8::from_str(vec[0]).unwrap()>255 {
                                    panic!("The maximum value for the color is too big!");
                                }
                            }
                        },
                        2 => {
                            let h : usize = usize::from_str(vec[1]).unwrap();
                            let w : usize = usize::from_str(vec[0]).unwrap();

                            img = Image {
                                height : h,
                                width : w,
                                pixels : vec![Pixel::init(); w*h]
                            };
                            init = true;
                            println!("Init ")
                        },
                        _ => {
                            if init == true {
                            let r : u8 = u8::from_str(vec[0]).unwrap();
                            let g : u8 = u8::from_str(vec[1]).unwrap();
                            let b : u8 = u8::from_str(vec[2]).unwrap();
                            
                            let mut pix : Pixel = Pixel::new(r,g,b);
                            println!("pix {} ", pix.display());
                            img.pixels.push(pix);
                            }else{
                                panic!("The image wasn't initialize");
                            }
                        }

                    }
                }
            }
            return img;
        }
        else {
            return Image {
                height : 0,
                width : 0,
                pixels : vec![Pixel::init(); 0]
        }
        }
    }

    pub fn toString(&self){
        for i in 0..self.height{
            for j in 0..self.width{
                print!("{:?} - ", self.getPixel(&i, &j).display());
            }
            println!("");
        }
    }

    pub fn getPixel(&self, x : &usize, y : &usize) -> Pixel{
        let index : usize = self.width*x+y;
        return self.pixels[index];
    }

    pub fn greyScale(&mut self){
        for x in 0..self.pixels.len(){
            self.pixels[x].greyScale();
        }
    }

    pub fn invert(&mut self){
        for x in 0..self.pixels.len(){
            self.pixels[x].invert();
        }
    }

}

fn getCharsAtIndex(my_string : &String, index :usize) -> char{
    match my_string.chars().nth(index) {
        Some(c) => return c,
        None => panic!("No character at index : {}", index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn PixelCreation() {
        let mut pixelA : Pixel = Pixel::new(0, 0, 0);
        let pixelB : Pixel = Pixel::new(255, 255, 255);

        pixelA.invert();

        assert!(pixelA.eq(&pixelB));
    }
}
