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

use ab_glyph::{Font, PxScale};
use image::RgbImage;

use imageproc::drawing::{draw_filled_rect_mut,draw_hollow_rect_mut};
use imageproc::rect::Rect;
use crate::draw::coord::DrawCoord;
use crate::draw::single_line::draw_line_of_colored_text;
use crate::text::paragraph::{ColoredTextParagraph,MultiLineTextAlignment};



pub fn draw_multiline_colored_text(image: &mut RgbImage,
                                   x_pos : &DrawCoord,
                                   y_pos : &DrawCoord,
                                   to_print : &ColoredTextParagraph,
                                   font: &impl Font,
                                   scale: impl Into<PxScale> + Copy) {
    // ***
    let (para_width, para_height, line_height) = to_print.paragraph_size(scale, font);
    // ***
    let (adjusted_left_x_pos, adjusted_top_y_pos) = DrawCoord::get_adjusted_object_top_left_corner(x_pos, y_pos, para_width, para_height);
    // ***
    if let Some(bg_col) = &to_print.background_color {
        let bg_rect = Rect::at(adjusted_left_x_pos as i32,adjusted_top_y_pos as i32).of_size(para_width as u32, para_height as u32);
        draw_filled_rect_mut(image, bg_rect,*bg_col)
    }
    // ***
    if let Some(bd_col) = &to_print.border_color {
        let bd_rect = Rect::at(adjusted_left_x_pos as i32,adjusted_top_y_pos as i32).of_size(para_width as u32, para_height as u32);
        draw_hollow_rect_mut(image, bd_rect,*bd_col)
    }
    // ***
    let (mut current_y_pos,startline_x_pos, inner_para_width) = match to_print.border_color {
        None => (adjusted_top_y_pos,adjusted_left_x_pos,para_width),
        Some(_) => (adjusted_top_y_pos + 2.0, adjusted_left_x_pos + 2.0,para_width - 4.0)
    };
    // ***
    for line in &to_print.lines {
        // ***
        let line_y_pos = DrawCoord::StartingAt(current_y_pos);
        current_y_pos += line_height;
        // ***
        let line_x_pos : DrawCoord = match &to_print.alignment {
            MultiLineTextAlignment::Left => {
                DrawCoord::StartingAt(startline_x_pos)
            },
            MultiLineTextAlignment::Right => {
                DrawCoord::EndingAt(startline_x_pos + inner_para_width)
            },
            MultiLineTextAlignment::Center => {
                DrawCoord::CenteredAround(startline_x_pos + (inner_para_width/2.0))
            }
        };
        draw_line_of_colored_text(image,&line_x_pos,&line_y_pos,line,font,scale);
    }

}