use image::Rgba;
use std::collections::HashMap;
//use std::env;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};



fn main() {
    let filename: &str = "soho.jpg";
    // terminal approach
    //let args: Vec<String> = env::args().collect();
    // load canvas with image
    canvas(filename.to_string());
    let image_colors = load_image_colors(filename);
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

// GTK4
// .dektop file
fn canvas(filename: String) -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.ICFA")
        .build();

    app.connect_activate(move |app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .default_height(400)
            .title("Image Color Frequency Analyzer")
            .build();

        // Create a new Image widget and set the image file.
        let image = gtk::Image::from_file(&filename);

        // Add the image to the window.
        window.set_child(Some(&image));

        window.present();
    });

    app.run()
}

// TODO: Canvas
// TODO: Show color scheme
// TODO: Divided by some white space
// TODO: Graphical interface?
