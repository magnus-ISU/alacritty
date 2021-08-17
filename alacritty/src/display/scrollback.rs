use alacritty_terminal::term::color::Rgb;
use alacritty_terminal::term::SizeInfo;
use alacritty_terminal::grid::Dimensions;

use crate::renderer::rects::RenderRect;
//use crate::event::ScrollbackState;

// TODO configurable
const BACKGROUND_COLOR: Rgb = Rgb { r: 0, g: 0, b: 0 };
const BACKGROUND_ALPHA: f32 = 0.2;
const FOREGROUND_COLOR: Rgb = Rgb { r: 255, g: 255, b: 255 };
const FOREGROUND_ALPHA: f32 = 0.6;
const HIGHLIGHT_COLOR: Rgb = Rgb { r: 64, g: 32, b: 192 };
const HIGHLIGHT_ALPHA: f32 = 1.0;

pub struct ScrollbackRects {
    index: usize,
    total_lines: usize,
    display_offset: usize,
    size_info: SizeInfo,

    viewport_left_x: f32,
    viewport_top_y: f32,
    viewport_width: f32,
    viewport_height: f32,
    //pub scrollback_state: &'a ScrollbackState,
}

impl ScrollbackRects {
    pub fn new(total_lines: usize, display_offset: usize, size_info: SizeInfo) -> ScrollbackRects {
        eprintln!("total_lines {}, screen_lines {}, display_offset {}", total_lines, size_info.screen_lines(), display_offset);
        let viewport_height = (size_info.screen_lines() * 2) as f32;
        let mut viewport_top_y = 2. * (total_lines - display_offset - size_info.screen_lines()) as f32;
        let top_of_middle_of_screen = (size_info.height() - viewport_height) / 2.0;
        if (total_lines * 2) as f32 > size_info.height() && viewport_top_y > top_of_middle_of_screen {
            //try rendering from the bottom up instead. If we are past the center, stay in the center
            viewport_top_y = size_info.height() - viewport_height - (display_offset * 2) as f32;
            if (viewport_top_y < top_of_middle_of_screen) {
                viewport_top_y = top_of_middle_of_screen;
            }
        }
        ScrollbackRects {
            index: 0,
            total_lines,
            display_offset,
            size_info,
            viewport_left_x: size_info.width() - ((size_info.columns() * 2) as f32),
            viewport_top_y,
            viewport_width: (size_info.columns() * 2) as f32,
            viewport_height,
        }
    }
}

// TODO figure out how to show an actual preview of the content in the terminal
impl Iterator for ScrollbackRects {
    type Item = RenderRect;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO currently pixels are hardcoded to be 2x2 pixels. Maybe make this configurable
        self.index += 1;
        if self.index == 1 {
            return Some(RenderRect::new(
                    self.viewport_left_x,
                    0.,
                    self.viewport_width,
                    self.size_info.height(),
                    BACKGROUND_COLOR,
                    BACKGROUND_ALPHA,
            ));
        } else if self.index == 2 {
            return Some(RenderRect::new(
                    self.viewport_left_x,
                    self.viewport_top_y,
                    self.viewport_width,
                    self.viewport_height,
                    FOREGROUND_COLOR,
                    FOREGROUND_ALPHA,
            ));
        }
        None
    }
}
