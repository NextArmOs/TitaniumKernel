pub struct CopperUI;

impl CopperUI {
    pub unsafe fn draw_rect(fb_ptr: *mut u32, pitch: usize, start_x: usize, start_y: usize, width: usize, height: usize, color: u32) {
        for y in 0..height {
            for x in 0..width {
                fb_ptr.add((start_y + y) * pitch + (start_x + x)).write_volatile(color);
            }
        }
    }
    pub unsafe fn draw_block_char(fb_ptr: *mut u32, pitch: usize, start_x: usize, start_y: usize, color: u32) {
        for y in 0..6 {
            for x in 0..6 {
                fb_ptr.add((start_y + y) * pitch + (start_x + x)).write_volatile(color);
            }
        }
    }
}
