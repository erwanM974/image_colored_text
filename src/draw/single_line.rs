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

use rusttype::{Font, Scale};
use image::RgbImage;
use imageproc::drawing::draw_text_mut;


use crate::ttp::TextToPrint;


pub enum DrawCoord {
    StartingAt(f32),
    EndingAt(f32),
    CenteredAround(f32)
}


pub fn draw_line_of_colored_text(image: &mut RgbImage,
                                 x_pos : &DrawCoord,
                                 y_pos : &DrawCoord,
                                 to_print : &Vec<TextToPrint>,
                                 font: &Font,
                                 scale: &Scale) {
    // ***
    let adjusted_x_pos : i32 = match x_pos {
        DrawCoord::CenteredAround( x ) => {
            let text_width = TextToPrint::get_text_width(to_print, font, scale);
            (x-(text_width/2.0)) as i32
        },
        DrawCoord::EndingAt( x ) => {
            let text_width = TextToPrint::get_text_width(to_print, font, scale);
            (x-text_width) as i32
        },
        DrawCoord::StartingAt(x) => {
            *x as i32
        }
    };
    // ***
    let adjusted_y_pos : i32 = match y_pos {
        DrawCoord::CenteredAround( y ) => {
            let font_height = TextToPrint::get_text_height(font,scale);
            (y-(font_height/2.0)) as i32
        },
        DrawCoord::EndingAt( y ) => {
            let font_height = TextToPrint::get_text_height(font,scale);
            (y-font_height) as i32
        },
        DrawCoord::StartingAt(y) => {
            *y as i32
        }
    };
    // ***
    {
        let mut char_count : usize = 0;
        for txt_to_print in to_print {
            let mut my_text : String = (0..char_count).map(|_| " ").collect::<String>();
            my_text.push_str( &txt_to_print.text );
            draw_text_mut(image,
                          txt_to_print.color,
                          adjusted_x_pos,
                          adjusted_y_pos,
                          *scale,
                          font,
                          &my_text
            );
            char_count += txt_to_print.text.chars().count();
        }
    }
}



