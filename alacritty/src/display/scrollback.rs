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
const PIXELS_PER_CHAR: f32 = 3.0;
const BAR_SIZE: f32 = 4.0;

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
        let viewport_height = (size_info.screen_lines() as f32) * PIXELS_PER_CHAR;
        let top_of_middle_of_screen = (size_info.height() - viewport_height) / 2.0;

        let mut viewport_top_y = PIXELS_PER_CHAR * (total_lines - display_offset - size_info.screen_lines()) as f32;
        if (total_lines as f32) * PIXELS_PER_CHAR > size_info.height() && viewport_top_y > top_of_middle_of_screen {
            //try rendering from the bottom up instead. If we are past the center, stay in the center
            viewport_top_y = size_info.height() - viewport_height - (display_offset as f32) * PIXELS_PER_CHAR;
            if viewport_top_y < top_of_middle_of_screen {
                viewport_top_y = top_of_middle_of_screen;
            }
        }

        ScrollbackRects {
            index: 0,
            total_lines,
            display_offset,
            size_info,
            viewport_left_x: size_info.width() - (size_info.columns() as f32) * PIXELS_PER_CHAR,
            viewport_top_y,
            viewport_width: (size_info.columns() as f32) * PIXELS_PER_CHAR,
            viewport_height,
        }
    }
}

// TODO figure out how to show an actual preview of the content in the terminal
// TODO maybe seperate the scrollbar from the preview so that you can have one or the other. Also
// maybe allow configuring them to appear on the left, middle, or right side
// TODO maybe figure out how to scroll with the scrollbar rather than always a constant inertia
impl Iterator for ScrollbackRects {
    type Item = RenderRect;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        // TODO yandare dev
        if self.index == 1 {
            // the sidebar
            return Some(RenderRect::new(
                    self.viewport_left_x,
                    0.,
                    self.viewport_width,
                    self.size_info.height(),
                    BACKGROUND_COLOR,
                    BACKGROUND_ALPHA,
            ));
        } else if self.index == 2 {
            // the visible portion of the screen
            return Some(RenderRect::new(
                    self.viewport_left_x,
                    self.viewport_top_y,
                    self.viewport_width,
                    self.viewport_height,
                    FOREGROUND_COLOR,
                    FOREGROUND_ALPHA,
            ));
        } else if self.index == 3 {
            // the full scrollbar
            let visible_portion = self.size_info.screen_lines() as f32 / self.total_lines as f32;
            let scrollbar_height = visible_portion * self.size_info.height();
            let scrolling_height = self.size_info.height() - scrollbar_height;
            let num_offscreen_lines = self.total_lines - self.size_info.screen_lines();
            let portion_through = (num_offscreen_lines - self.display_offset) as f32 / num_offscreen_lines as f32;
            eprintln!("visible: {}, scrollbar: {}, scrolling: {}, portion: {}", visible_portion, scrollbar_height, scrolling_height, portion_through);
            return Some(RenderRect::new(
                    0.0,
                    portion_through * scrolling_height,
                    BAR_SIZE,
                    scrollbar_height,
                    HIGHLIGHT_COLOR,
                    HIGHLIGHT_ALPHA,
            ));
        }
        None
    }
}
