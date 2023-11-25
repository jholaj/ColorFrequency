use image::Rgba;
use std::collections::HashMap;
//use std::env;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Image};



fn main() {
    let filename: &str = "file";
    // terminal approach
    //let args: Vec<String> = env::args().collect();
    // load canvas with image
    // attempt to add with overflow with big picture?
    canvas(filename.to_string());
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

fn find_dominant_colors(pixel_colors: Vec<Rgba<u8>>) -> Vec<Rgba<u8>>{
    println!("Finding dominant colors...");
    let mut colors_frequency: HashMap<Rgba<u8>, i16> = HashMap::new();
    
    // dict
    // key <color> : value <frequency>
    // if key in dict => value + 1
    // else 
    // add key
    // dict unsorted => convert to vec
    /* 

    // old way
    for color in pixel_colors{
        if colors_frequency.contains_key(&color){
            *colors_frequency.get_mut(&color).unwrap() += 1;
        } else {
            colors_frequency.insert(color, 0);
        }
    }
    */
    // refactor
    for color in &pixel_colors {
        *colors_frequency.entry(*color).or_insert(0) += 1;
    }

    // convert
    let mut colors_vec: Vec<(&Rgba<u8>, &i16)> = colors_frequency.iter().collect();
    
    // sort
    colors_vec.sort_by(|a, b| b.1.cmp(a.1));

    // first 100
    colors_vec.iter().take(100).map(|(color, _)| *color.clone()).collect()

}

fn load_picture_info(filename: String) -> Vec<Rgba<u8>> {
    let image_colors = load_image_colors(&filename);
    find_dominant_colors(image_colors)
}

// GTK4
fn canvas(filename: String) -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.ICFA")
        .build();
    
    app.connect_activate(move |app| {

        // main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .default_height(400)
            .title("Image Color Frequency Analyzer")
            .build();

        // image widgets
        let main_box = Box::new(gtk::Orientation::Vertical, 0);
        let image = Image::from_file(&filename);
        image.set_size_request(1000, 500);
        main_box.append(&image);

        let colors: Vec<Rgba<u8>> = load_picture_info(filename.to_string());

        let color_list = Box::new(gtk::Orientation::Horizontal, 0);
        
        // drawing rectangles
        for color in colors {
            let rectangle = gtk::DrawingArea::new();
            rectangle.set_content_width(10);
            rectangle.set_content_height(100);
            rectangle.set_draw_func(move |_, cr, _width, _height| {
                cr.set_source_rgb(color[0] as f64 / 255.0, color[1] as f64 / 255.0, color[2] as f64 / 255.0);
                cr.paint();
            });
            color_list.append(&rectangle);
        }

        main_box.append(&color_list);

        window.set_child(Some(&main_box));

        window.present();
    });

    app.run()
}
// TODO: Graphical interface?
// TODO: Only colors, that are diff // some threshold?
