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

use ab_glyph::{point, Font, GlyphId, PxScale, ScaleFont};
use image::Rgb;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ColoredTextLine {
    pub colored_segments : Vec<(String,Rgb<u8>)>
}


impl ColoredTextLine {

    pub fn new(colored_segments : Vec<(String,Rgb<u8>)>) -> ColoredTextLine {
        ColoredTextLine{colored_segments}
    }

    pub fn line_size(
        &self,
        scale: impl Into<PxScale> + Copy,
        font: &impl Font
    ) -> (f32, f32) {
        ColoredTextLine::text_size(scale,font,&self.flatten())
    }

    fn flatten(&self) -> String {
        let mut flattened : String = String::new();
        for (text,_) in &self.colored_segments {
            flattened.push_str(text);
        }
        flattened
    }

    fn text_size(
        scale: impl Into<PxScale> + Copy,
        font: &impl Font,
        text: &str
    ) -> (f32, f32) {
        if text.is_empty() {
            return (0.0, 0.0);
        }
        let scaled_font = font.as_scaled(scale);
        let mut width = 0.0;
        let mut prev: Option<GlyphId> = None;
        // ***
        for c in text.chars() {
            let glyph_id = font.glyph_id(c);
            let glyph = glyph_id.with_scale_and_position(scale, point(width, scaled_font.ascent()));
            width += scaled_font.h_advance(glyph_id);
            if let Some(_) = scaled_font.outline_glyph(glyph) {
                if let Some(prev) = prev {
                    width += scaled_font.kern(glyph_id, prev);
                }
                prev = Some(glyph_id);
            }
        }
        // ***
        (width, scaled_font.height())
    }
}

