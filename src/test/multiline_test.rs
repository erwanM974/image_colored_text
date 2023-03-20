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

use crate::draw::multi_line::{draw_multiline_colored_text, MultiLineTextAlignment};
use crate::draw::single_line::DrawCoord;
use crate::ttp::TextToPrint;



const DARK_RED : [u8;3] = [86u8, 15u8, 15u8];
const DARK_GREEN : [u8;3] = [15u8, 86u8, 15u8];
const DARK_BLUE : [u8;3] = [15u8, 15u8, 86u8];
const BLACK : [u8;3] = [0u8, 0u8, 0u8];
const WHITE : [u8;3] = [255u8,  255u8,  255u8];

const FONT_WIDTH : f32 = 20.0;
const FONT_HEIGHT : f32 = 20.0;

const IMG_WIDTH : u32 = 200;
const IMG_HEIGHT : u32 = 400;




#[test]
pub fn multi_line_test() {
    let mut image = RgbImage::new( IMG_WIDTH, IMG_HEIGHT);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(IMG_WIDTH,IMG_HEIGHT),
                         Rgb(WHITE));

    {
        let x = 100;
        let y = 200;
        let to_print = vec![
            vec![TextToPrint::new("xcent".to_string(), Rgb(DARK_RED)),TextToPrint::new(format!("{}",x), Rgb(DARK_GREEN))],
            vec![TextToPrint::new(";".to_string(), Rgb(DARK_BLUE))],
            vec![TextToPrint::new("ycent".to_string(), Rgb(DARK_RED)),TextToPrint::new(format!("{}",y), Rgb(DARK_GREEN))]
        ];
        let xpos = DrawCoord::CenteredAround( x as f32 );
        let ypos = DrawCoord::CenteredAround( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&MultiLineTextAlignment::Center, &to_print,FONT_WIDTH,FONT_HEIGHT);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 25;
        let y = 50;
        let to_print = vec![
            vec![TextToPrint::new("xstart".to_string(), Rgb(DARK_RED)),TextToPrint::new(format!("{}",x), Rgb(DARK_GREEN))],
            vec![TextToPrint::new(";".to_string(), Rgb(DARK_BLUE))],
            vec![TextToPrint::new("ystart".to_string(), Rgb(DARK_RED)),TextToPrint::new(format!("{}",y), Rgb(DARK_GREEN))]
        ];
        let xpos = DrawCoord::StartingAt( x as f32 );
        let ypos = DrawCoord::StartingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&MultiLineTextAlignment::Left,&to_print,FONT_WIDTH,FONT_HEIGHT);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    {
        let x = 175;
        let y = 350;
        let to_print = vec![
            vec![TextToPrint::new("xend".to_string(), Rgb(DARK_RED)),TextToPrint::new(format!("{}",x), Rgb(DARK_GREEN))],
            vec![TextToPrint::new(";".to_string(), Rgb(DARK_BLUE))],
            vec![TextToPrint::new("yend".to_string(), Rgb(DARK_RED)),TextToPrint::new(format!("{}",y), Rgb(DARK_GREEN))]
        ];
        let xpos = DrawCoord::EndingAt( x as f32 );
        let ypos = DrawCoord::EndingAt( y as f32 );
        draw_multiline_colored_text(&mut image,&xpos,&ypos,&MultiLineTextAlignment::Right, &to_print,FONT_WIDTH,FONT_HEIGHT);
        draw_cross_mut(&mut image,Rgb(BLACK),x,y);
    }

    // ***
    let path_buf : PathBuf = ["c:\\", "Users", "EM244186", "IdeaProjects", "colored_text", "multi_line_test.png"].iter().collect();
    image.save(path_buf.as_path()).unwrap();
}


