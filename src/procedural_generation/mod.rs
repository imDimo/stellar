use bevy::prelude::*;

pub mod gen_planet;
pub mod gen_star;
pub mod gen_system;

pub fn circle_texture(width: u32, height: u32, images: &mut ResMut<Assets<Image>>) -> Handle<Image> {

    let pix_width = match width {
        x if x < 1 => 1,
        x => x
    };
    let pix_height = match height {
        x if x < 1 => 1,
        x => x
    };


    //create an image buffer
    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = 
        image::ImageBuffer::new(pix_width, pix_height);

    //calculate center and radius of circle
    let radius = pix_width as f32 / 2.0;
    let c_x = pix_width as f32 / 2.0;
    let c_y = pix_height as f32 / 2.0;

    //iterate thru all pixels
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //get squared distance from the center
        let distance_squared = (x as f32 - c_x).powi(2) + (y as f32 - c_y).powi(2);

        //if it falls in this small range, then color it whatever color
        if distance_squared >= (radius - 1.0).powi(2) && distance_squared <= radius.powi(2) {
            *pixel = image::Rgba([255, 165, 0, 255]);
        }
    }

    //convert the image::Image into bevy_image::image::Image via image::DynamicImage
    //lol
    let img = Image::from_dynamic(
        image::DynamicImage::ImageRgba8(imgbuf),
        false,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD
    );
    //lastly add it to the thing 
    let img_handle = images.add(img);

    //to get the thing
    return img_handle;
}

pub fn circle_texture_color(width: u32, height: u32, images: &mut ResMut<Assets<Image>>, r: u8, g: u8, b: u8, a: u8)
 -> Handle<Image> {

    let pix_width = match width {
        x if x < 1 => 1,
        x => x
    };
    let pix_height = match height {
        x if x < 1 => 1,
        x => x
    };


    //create an image buffer
    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = 
        image::ImageBuffer::new(pix_width, pix_height);

    //calculate center and radius of circle
    let radius = pix_width as f32 / 2.0;
    let c_x = pix_width as f32 / 2.0;
    let c_y = pix_height as f32 / 2.0;

    //iterate thru all pixels
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //get squared distance from the center
        let distance_squared = (x as f32 - c_x).powi(2) + (y as f32 - c_y).powi(2);

        //if it falls in this small range, then color it whatever color
        if distance_squared >= (radius - 1.0).powi(2) && distance_squared <= radius.powi(2) {
            *pixel = image::Rgba([r, g, b, a]);
        }
    }

    //convert the image::Image into bevy_image::image::Image via image::DynamicImage
    //lol
    let img = Image::from_dynamic(
        image::DynamicImage::ImageRgba8(imgbuf),
        false,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD
    );
    //lastly add it to the thing 
    let img_handle = images.add(img);

    //to get the thing
    return img_handle;
}