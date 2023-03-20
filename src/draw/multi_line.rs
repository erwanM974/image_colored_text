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


use image::RgbImage;


use crate::draw::single_line::{draw_line_of_colored_text, DrawCoord};
use crate::ttp::TextToPrint;



pub enum MultiLineTextAlignment {
    Left,
    Center,
    Right
}


pub fn draw_multiline_colored_text(image: &mut RgbImage,
                                   x_pos : &DrawCoord,
                                   y_pos : &DrawCoord,
                                   alignment : &MultiLineTextAlignment,
                                   to_print : &Vec<Vec<TextToPrint>>,
                                   font_width : f32,
                                   font_height : f32) {
    // ***
    let adjusted_first_line_y_pos : f32;
    match y_pos {
        DrawCoord::CenteredAround( y ) => {
            let text_height = (to_print.len() as f32) * font_height;
            adjusted_first_line_y_pos = y-(text_height/2.0);
        },
        DrawCoord::EndingAt( y ) => {
            let text_height = (to_print.len() as f32) * font_height;
            adjusted_first_line_y_pos = y-text_height;
        },
        DrawCoord::StartingAt(y) => {
            adjusted_first_line_y_pos = *y;
        }
    }
    // ***
    let max_text_width : f32 = to_print.iter().fold(0.0,
                                                    |prev,ttp|
                                                        prev.max(TextToPrint::get_text_width(ttp,font_width)));
    let adjusted_left_x_pos : f32;
    match x_pos {
        DrawCoord::CenteredAround( x ) => {
            adjusted_left_x_pos = x-(max_text_width/2.0);
        },
        DrawCoord::EndingAt( x ) => {
            adjusted_left_x_pos = x-max_text_width;
        },
        DrawCoord::StartingAt(x) => {
            adjusted_left_x_pos = *x;
        }
    }
    // ***
    let mut current_y_pos = adjusted_first_line_y_pos;
    for ttp in to_print {
        // ***
        let ttp_y_pos = DrawCoord::StartingAt(current_y_pos);
        current_y_pos += font_height;
        // ***
        let ttp_x_pos : DrawCoord;
        match alignment {
            MultiLineTextAlignment::Left => {
                ttp_x_pos = DrawCoord::StartingAt(adjusted_left_x_pos);
            },
            MultiLineTextAlignment::Right => {
                ttp_x_pos = DrawCoord::EndingAt(adjusted_left_x_pos + max_text_width);
            },
            MultiLineTextAlignment::Center => {
                ttp_x_pos = DrawCoord::CenteredAround(adjusted_left_x_pos + (max_text_width/2.0));
            }
        }
        draw_line_of_colored_text(image,&ttp_x_pos,&ttp_y_pos,ttp,font_width,font_height);
    }

}