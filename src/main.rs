use image::Rgba;
use std::collections::HashMap;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let image_colors = load_image_colors(&args[1]);
    find_dominant_colors(image_colors);
}

// argument of function & return type
fn load_image_colors(str_path : &str) -> Vec<Rgba<u8>> {
    println!("Opening {}...", str_path);
    let img = image::open(str_path).unwrap();
    println!("Image {} loaded succesfully...", str_path);

    //load pixels
    let mut pixel_colors: Vec<Rgba<u8>> = Vec::new();
    for (_x, _y, pixel) in img.to_rgba8().enumerate_pixels() {
        pixel_colors.push(*pixel);
    }

    pixel_colors
}

fn find_dominant_colors(pixel_colors: Vec<Rgba<u8>>){
    println!("Finding dominant colors...");
    // dict
    // key <color> : value <frequency>
    // if key in dict => value + 1
    // else 
    // add key
    // dict unsorted => convert to vec
    let mut colors_frequency: HashMap<Rgba<u8>, i16> = HashMap::new();

    for color in pixel_colors{
        if colors_frequency.contains_key(&color){
            *colors_frequency.get_mut(&color).unwrap() += 1;
        } else {
            colors_frequency.insert(color, 0);
        }
    }

    // convert
    let mut colors_vec: Vec<(&Rgba<u8>, &i16)> = colors_frequency.iter().collect();
    
    // sort
    colors_vec.sort_by(|a, b| b.1.cmp(a.1));

    // first 100
    for (key, value) in colors_vec.iter().take(100) {
        println!("{:?}: {}", key, value);
    }
}

// TODO: Canvas
// TODO: Show color scheme
// TODO: Divided by some white space
// TODO: Graphical interface?
