/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/



use std::path::PathBuf;
use image::Rgb;
use image::RgbImage;

use imageproc::drawing::{draw_cross_mut, draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

use crate::draw::single_line::draw_line_of_colored_text;
use crate::test::commons::*;
use crate::text::line::ColoredTextLine;
use crate::draw::coord::DrawCoord;




const IMG_WIDTH : u32 = 225;
const IMG_HEIGHT : u32 = 200;


#[test]
pub fn single_line_test() {
    let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();

    let mut image = RgbImage::new( IMG_WIDTH, IMG_HEIGHT);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(IMG_WIDTH,IMG_HEIGHT),
                         Rgb(WHITE));

    {
        let x = 100;
        let y = 100;
        let to_print = ColoredTextLine::new(
            vec![
                ("xcent".to_string(), Rgb(DARK_RED)),
                (format!("{}",x), Rgb(DARK_GREEN)),
                (";".to_string(), Rgb(DARK_BLUE)),
                ("ycent".to_string(), Rgb(DARK_RED)),
                (format!("{}",y), Rgb(DARK_GREEN))
            ]

        );
        let xpos = DrawCoord::CenteredAround( x as f32 );
        let ypos = DrawCoord::CenteredAround( y as f32 );
        draw_line_of_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
        // ***
        let (txt_width,_) = to_print.line_size(SCALE,&font);
        draw_line_segment_mut(&mut image,
                              ((x as f32) - txt_width/2.0,y as f32),
                              ((x as f32) + txt_width/2.0,y as f32),
                              Rgb(BLACK));
    }

    {
        let x = 25;
        let y = 50;

        let to_print = ColoredTextLine::new(
            vec![
                ("xstart".to_string(), Rgb(DARK_RED)),
                (format!("{}",x), Rgb(DARK_GREEN)),
                (";".to_string(), Rgb(DARK_BLUE)),
                ("ystart".to_string(), Rgb(DARK_RED)),
                (format!("{}",y), Rgb(DARK_GREEN))
            ]

        );
        let xpos = DrawCoord::StartingAt( x as f32 );
        let ypos = DrawCoord::StartingAt( y as f32 );
        draw_line_of_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
        // ***
        let (txt_width,_) = to_print.line_size(SCALE,&font);
        draw_line_segment_mut(&mut image,
                              (x as f32,y as f32),
                              ((x as f32) + txt_width,y as f32),
                              Rgb(BLACK));
    }

    {
        let x = 175;
        let y = 150;
        let to_print = ColoredTextLine::new(
            vec![
                ("xend".to_string(), Rgb(DARK_RED)),
                (format!("{}",x), Rgb(DARK_GREEN)),
                (";".to_string(), Rgb(DARK_BLUE)),
                ("yend".to_string(), Rgb(DARK_RED)),
                (format!("{}",y), Rgb(DARK_GREEN))
            ]

        );
        let xpos = DrawCoord::EndingAt( x as f32 );
        let ypos = DrawCoord::EndingAt( y as f32 );
        draw_line_of_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
        // ***
        let (txt_width,_) = to_print.line_size(SCALE,&font);
        draw_line_segment_mut(&mut image,
                              ((x as f32) - txt_width,y as f32),
                              (x as f32,y as f32),
                              Rgb(BLACK));
    }

    // ***
    let path_buf : PathBuf = ["/home", "stan", "vscodium_projects", "image_colored_text", "single_line_test.png"].iter().collect();
    image.save(path_buf.as_path()).unwrap();
}




