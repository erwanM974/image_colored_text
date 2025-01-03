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

pub const DRAWING_GRAPHIC_FONT: &'static [u8] = include_bytes!("DejaVuSansMono.ttf");

pub const DARK_RED : [u8;3] = [86u8, 15u8, 15u8];
pub const DARK_GREEN : [u8;3] = [15u8, 86u8, 15u8];
pub const DARK_BLUE : [u8;3] = [15u8, 15u8, 86u8];
pub const BLACK : [u8;3] = [0u8, 0u8, 0u8];
pub const WHITE : [u8;3] = [255u8,  255u8,  255u8];

pub const OLD_PAPER : [u8;3] = [250u8, 221u8, 147u8];

pub const SCALE : f32 = 20.0;