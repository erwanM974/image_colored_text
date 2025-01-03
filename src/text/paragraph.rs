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
use image::Rgb;
use crate::text::line::ColoredTextLine;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MultiLineTextAlignment {
    Left,
    Center,
    Right
}



#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ColoredTextParagraph {
    pub lines : Vec<ColoredTextLine>,
    pub alignment : MultiLineTextAlignment,
    pub background_color : Option<Rgb<u8>>,
    pub border_color : Option<Rgb<u8>>
}


impl ColoredTextParagraph {

    pub fn new(
        lines : Vec<ColoredTextLine>,
        alignment : MultiLineTextAlignment,
        background_color : Option<Rgb<u8>>,
        border_color : Option<Rgb<u8>>
    ) -> ColoredTextParagraph {
        ColoredTextParagraph{lines,alignment,background_color,border_color}
    }

pub fn paragraph_size(&self,
        scale: impl Into<PxScale> + Copy,
        font: &impl Font
    ) -> (f32,f32,f32) {
    let (max_text_width,font_height) = self.lines.iter()
        .fold((0.0_f32,0.0_f32),
              |(w,h),line: &ColoredTextLine|
                  {
                    let (w2,h2) = line.line_size(scale, font);
                    (w.max(w2),h.max(h2))
                  }
    );
    let text_height = (self.lines.len() as f32) * font_height;
    if self.border_color.is_some() {
        (max_text_width + 4.0, text_height + 4.0, font_height)
    } else {
        (max_text_width, text_height, font_height)
    }
}

}