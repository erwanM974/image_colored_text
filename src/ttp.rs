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


use image::Rgb;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TextToPrint {
    pub text : String,
    pub color : Rgb<u8>
}

impl TextToPrint {

    pub fn new(text : String,
               color : Rgb<u8>) -> TextToPrint {
        return TextToPrint{text,color};
    }

    pub fn flatten(to_print : &Vec<TextToPrint>) -> String {
        let mut flattened : String = String::new();
        for ttp in to_print {
            flattened.push_str(&ttp.text);
        }
        return flattened;
    }

    pub fn char_count(to_print : &Vec<TextToPrint>) -> usize {
        let mut count : usize = 0;
        for ttp in to_print {
            count = count + ttp.text.chars().count();
        }
        return count;
    }

    pub fn get_text_width(to_print : &Vec<TextToPrint>, font_width : f32) -> f32 {
        return (TextToPrint::char_count(&to_print) as f32)*font_width/2.0;
    }

}