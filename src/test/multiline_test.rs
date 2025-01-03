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

use imageproc::drawing::{draw_cross_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;

use crate::draw::multi_line::draw_multiline_colored_text;
use crate::draw::coord::DrawCoord;
use crate::test::commons::*;

use crate::text::line::ColoredTextLine;
use crate::text::paragraph::{ColoredTextParagraph,MultiLineTextAlignment};



const IMG_WIDTH : u32 = 200;
const IMG_HEIGHT : u32 = 400;




#[test]
pub fn multi_line_test() {
    let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();

    let mut image = RgbImage::new( IMG_WIDTH, IMG_HEIGHT);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(IMG_WIDTH,IMG_HEIGHT),
                         Rgb(WHITE));

    {
        let x = 100;
        let y = 200;
        let line1 = ColoredTextLine::new(vec![("xcent".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("ycent".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Center, None, None);
        let xpos = DrawCoord::CenteredAround( x as f32 );
        let ypos = DrawCoord::CenteredAround( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 25;
        let y = 50;
        let line1 = ColoredTextLine::new(vec![("xstart".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("ystart".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Left, None, None);
        let xpos = DrawCoord::StartingAt( x as f32 );
        let ypos = DrawCoord::StartingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 175;
        let y = 350;
        let line1 = ColoredTextLine::new(vec![("xend".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("yend".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Right, None, None);
        let xpos = DrawCoord::EndingAt( x as f32 );
        let ypos = DrawCoord::EndingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    // ***
    let path_buf : PathBuf = ["/home", "stan", "vscodium_projects", "image_colored_text", "multi_line_test.png"].iter().collect();
    image.save(path_buf.as_path()).unwrap();
}



#[test]
pub fn multi_line_test_with_background() {
    let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();

    let mut image = RgbImage::new( IMG_WIDTH, IMG_HEIGHT);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(IMG_WIDTH,IMG_HEIGHT),
                         Rgb(WHITE));

    {
        let x = 100;
        let y = 200;
        let line1 = ColoredTextLine::new(vec![("xcent".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("ycent".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Center, Some(Rgb(OLD_PAPER)), None);
        let xpos = DrawCoord::CenteredAround( x as f32 );
        let ypos = DrawCoord::CenteredAround( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 25;
        let y = 50;
        let line1 = ColoredTextLine::new(vec![("xstart".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("ystart".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Left, Some(Rgb(OLD_PAPER)), None);
        let xpos = DrawCoord::StartingAt( x as f32 );
        let ypos = DrawCoord::StartingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 175;
        let y = 350;
        let line1 = ColoredTextLine::new(vec![("xend".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("yend".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Right, Some(Rgb(OLD_PAPER)), None);
        let xpos = DrawCoord::EndingAt( x as f32 );
        let ypos = DrawCoord::EndingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }
    // ***
    let path_buf : PathBuf = ["/home", "stan", "vscodium_projects", "image_colored_text", "multi_line_test_with_background.png"].iter().collect();
    image.save(path_buf.as_path()).unwrap();
}



#[test]
pub fn multi_line_test_with_background_and_border() {
    let font = ab_glyph::FontRef::try_from_slice(DRAWING_GRAPHIC_FONT).unwrap();

    let mut image = RgbImage::new( IMG_WIDTH, IMG_HEIGHT);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(IMG_WIDTH,IMG_HEIGHT),
                         Rgb(WHITE));

    {
        let x = 100;
        let y = 200;
        let line1 = ColoredTextLine::new(vec![("xcent".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("ycent".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Center, Some(Rgb(OLD_PAPER)), Some(Rgb(BLACK)));
        let xpos = DrawCoord::CenteredAround( x as f32 );
        let ypos = DrawCoord::CenteredAround( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 25;
        let y = 50;
        let line1 = ColoredTextLine::new(vec![("xstart".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("ystart".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Left, Some(Rgb(OLD_PAPER)), Some(Rgb(BLACK)));
        let xpos = DrawCoord::StartingAt( x as f32 );
        let ypos = DrawCoord::StartingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 175;
        let y = 350;
        let line1 = ColoredTextLine::new(vec![("xend".to_string(), Rgb(DARK_RED)),(format!("{}",x), Rgb(DARK_GREEN))]);
        let line2 = ColoredTextLine::new(vec![(";".to_string(), Rgb(DARK_BLUE))]);
        let line3 = ColoredTextLine::new(vec![("yend".to_string(), Rgb(DARK_RED)),(format!("{}",y), Rgb(DARK_GREEN))]);
        let to_print = ColoredTextParagraph::new(vec![line1,line2,line3], MultiLineTextAlignment::Right, Some(Rgb(OLD_PAPER)), Some(Rgb(BLACK)));
        let xpos = DrawCoord::EndingAt( x as f32 );
        let ypos = DrawCoord::EndingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&to_print,&font,SCALE);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }
    // ***
    let path_buf : PathBuf = ["/home", "stan", "vscodium_projects", "image_colored_text", "multi_line_test_with_background_and_border.png"].iter().collect();
    image.save(path_buf.as_path()).unwrap();
}
