extern crate gtk;
extern crate cairo;
extern crate glib;
extern crate gdk_pixbuf;
extern crate lazy_static;
extern crate rand;
extern crate image;

use cairo::Context;
use gdk::prelude::*;
use gtk::prelude::*;
use gdk_pixbuf::*;
use gdk_pixbuf::prelude::*;

use rand::Rng;

use image::{ImageBuffer, RgbaImage, DynamicImage, imageops, GenericImageView, GenericImage};

use std::borrow::Borrow;
use std::sync::*;
use std::rc::Rc;
use std::cell::RefCell;

use super::living;
use super::living::*;
use super::super::game;

// thread_local!(static PICT: RefCell<(Vec<u8>, (usize, usize))> = {
//     let image = image::open("grass.png").unwrap();
//     let data = image.pixels();
//     let mut vec: Vec<u8> = Vec::with_capacity(image.width() as usize * image.height() as usize);
//     for (x, y, rgba) in data{
//         vec.push(rgba[0]);vec.push(rgba[1]);vec.push(rgba[2]);vec.push(rgba[3]);
//     }
//     RefCell::new((vec, (image.width() as usize, image.height() as usize)))
// });
thread_local!(static PICT: RefCell<Pixbuf>={
    RefCell::new(Pixbuf::new_from_file("grass.png").unwrap())
});

pub struct Grass{
    pub hp: i64,
    pub pos: (i64, i64),
    pub id: usize
}
impl Grass{
    pub fn new(hp: i64, pos: (i64, i64))->Grass{
        Grass{hp: hp, pos: pos, id: 0}
    }
}
impl living::Living for Grass{
    fn get_hp(&self)->i64{
        self.hp
    }
    fn get_pos(&self)->(i64,i64){
        self.pos
    }
    fn update(&mut self, deltatime: u128){
        self.hp -= deltatime as i64;
        if self.pos.0 < 0 { self.pos.0 = 0;}
        else if self.pos.0 > 640{self.pos.0 = 640;}
        if self.pos.1 < 0 { self.pos.1 = 0;}
        else if self.pos.1 > 480{self.pos.1 = 480;}
        
        let game = game::GAME.read().unwrap();

        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 100) == 0{
            let p = (self.pos.0 + rng.gen_range(-30, 30), self.pos.1 + rng.gen_range(-30, 30));
            let li: Arc<Mutex<Living>> = Arc::new(Mutex::new(Grass{hp: 10000, pos: p, id: 0}));
            game.world.add_living(&li);
        }
    }
    fn draw(&self, context: &cairo::Context, painted_cells: &mut Vec<Vec<u8>>){
        if painted_cells.len() == 0 {return;}
        if painted_cells[0].len() == 0{return;}
        PICT.with(|pict|{
            let pict = pict.borrow();
            let width = pict.get_width();
            let height = pict.get_height();

            let mut tl = self.pos;
            let mut br = (self.pos.0 + width as i64, self.pos.1 + height as i64);
            if br.0 >= painted_cells.len() as i64 {br.0 = painted_cells.len() as i64 - 1;}
            if br.0 < 0 {br.0 = 0;}
            if br.1 >= painted_cells[0].len() as i64 {br.1 = painted_cells[0].len() as i64 - 1;}
            if br.1 < 0 {br.1 = 0;}
            if br.1 - tl.1 <= 0 || br.0 - tl.0 <= 0{return;}
            if painted_cells[br.0 as usize][br.1 as usize] == 1 && painted_cells[tl.0 as usize][tl.1 as usize] == 1 && painted_cells[((tl.0+br.0)/2) as usize][((tl.1+br.1)/2) as usize] == 1 {
                // println!("{:?}, {:?}", br, tl);
                return;
            }
            // println!("AAAAAAA{:?}, {:?}", br, tl);
            
            for x in tl.0 as usize..(br.0+1) as usize{
                for y in tl.1 as usize..(br.1+1) as usize{
                    painted_cells[x][y] = 1
                }
            }
            context.set_source_pixbuf(&pict, self.pos.0 as f64, self.pos.1 as f64);
            context.paint();
        });
    }
    // fn draw(&self, to_image: &mut [u8], window_size: (usize, usize)){
    //     PICT.with(|pict|{
    //         let pict = pict.borrow();
    //         let pict_bin = &pict.0;
    //         let siz = pict.1;

    //         if window_size.0 < siz.0 as usize  || window_size.1 < siz.1 as usize  {return;}
    //         if self.pos.0 as usize > window_size.0 - siz.0 as usize {return;}
    //         if self.pos.1 as usize > window_size.1 - siz.1 as usize {return;}

    //         for x in self.pos.0..self.pos.0+siz.0 as i64{
    //             for y in self.pos.1..self.pos.1+siz.1 as i64{
    //                 let window_range_start = (x as usize+y as usize*window_size.0)*4;
    //                 let myimg_range_start = ((x - self.pos.0) as usize+(y - self.pos.1) as usize*siz.0)*4;
    //                 for i in 0..4{
    //                     to_image[window_range_start+i] = pict_bin[myimg_range_start+i];
    //                 }
    //             }
    //         }
    //     });
    // }
    fn get_id(&self)->usize{
        self.id
    }
    fn set_id(&mut self, id:usize){
        self.id = id;
    }
    fn set_pos(&mut self,pos: (i64, i64)){
        self.pos = pos;
    }
    fn set_hp(&mut self, hp: i64){
        self.hp = hp;
    }
}