extern crate cairo;
extern crate gtk;
extern crate image;

use cairo::Context;
use gtk::DrawingArea;
use super::super::game;
use image::RgbaImage;

pub trait Living: Send+Sync{
    fn get_pos(&self) -> (i64, i64);
    fn set_pos(&mut self, pos: (i64, i64));
    fn get_hp(&self) -> i64;
    fn set_hp(&mut self, hp: i64);
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn update(&mut self, deltatime: u128);
    // fn draw(&self, to_image: &mut [u8], window_size: (usize, usize));
    fn draw(&self, context: &cairo::Context, painted_cells: &mut Vec<Vec<u8>>);
    fn is_dead(&self)->bool{
        self.get_hp() <= 0
    }
}
