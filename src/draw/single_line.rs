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
use imageproc::drawing::draw_text_mut;

use crate::draw::coord::DrawCoord;
use crate::text::line::ColoredTextLine;





pub fn draw_line_of_colored_text(image: &mut RgbImage,
                                 x_pos : &DrawCoord,
                                 y_pos : &DrawCoord,
                                 to_print : &ColoredTextLine,
                                 font: &impl Font,
                                 scale: impl Into<PxScale> + Copy) {
    // ***
    let (text_width,font_height) = to_print.line_size(scale, font);
    let (adjusted_x_pos,adjusted_y_pos) = DrawCoord::get_adjusted_object_top_left_corner(x_pos, y_pos, text_width, font_height);
    // ***
    {
        let mut char_count : usize = 0;
        for (text,color) in &to_print.colored_segments {
            let mut my_text : String = (0..char_count).map(|_| " ").collect::<String>();
            my_text.push_str( text );
            draw_text_mut(image,
                          *color,
                          adjusted_x_pos as i32,
                          adjusted_y_pos as i32,
                          scale,
                          font,
                          &my_text
            );
            char_count += text.chars().count();
        }
    }
}



