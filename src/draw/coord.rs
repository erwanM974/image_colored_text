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


pub enum DrawCoord {
    StartingAt(f32),
    EndingAt(f32),
    CenteredAround(f32)
}



impl DrawCoord {
    
    pub fn get_adjusted_object_top_left_corner(
            x_pos : &DrawCoord,
            y_pos : &DrawCoord,
            object_width : f32, 
            object_height : f32) 
        -> (f32, f32) {
        let adjusted_x_pos = match x_pos {
            DrawCoord::CenteredAround( x ) => {
                x-(object_width/2.0)
            },
            DrawCoord::EndingAt( x ) => {
                x-object_width
            },
            DrawCoord::StartingAt(x) => {
                *x
            }
        };
        // ***
        let adjusted_y_pos = match y_pos {
            DrawCoord::CenteredAround( y ) => {
                y-(object_height/2.0)
            },
            DrawCoord::EndingAt( y ) => {
                y-object_height
            },
            DrawCoord::StartingAt(y) => {
                *y
            }
        };
        (adjusted_x_pos,adjusted_y_pos)
    }

}

