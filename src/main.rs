extern crate rust_seitai;
extern crate gtk;
extern crate gio;
extern crate gdk_pixbuf;
extern crate cairo;
extern crate rand;
extern crate glib;
extern crate image;

use std::sync::RwLock;

use gio::prelude::*;
use gtk::prelude::*;
use gdk::prelude::*;
use cairo::prelude::*;

use gtk::Builder;
use gdk_pixbuf::Pixbuf;
use gtk::Value;

use cairo::{ImageSurface, Format};

use std::env::args;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;
use std::sync::*;

use rand::Rng;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbaImage};

use rust_seitai::game;
use rust_seitai::living;
use rust_seitai::living::living::*;
use rust_seitai::living::grass::Grass;


fn draw_getwidgets<'a>(window: &'a gtk::Paned)->
        (gtk::DrawingArea,  gtk::Notebook, gtk::ListBox, gtk::Label){
    let widgets = window.get_children();
    let mut drawing: Option<gtk::DrawingArea> = None;
    let mut notebook: Option<gtk::Notebook> = None;

    for w in widgets{
        w.downcast::<gtk::DrawingArea>()
            .map(|t|drawing=Some(t))
            .map_err(|t|t.downcast::<gtk::Notebook>()
            .map(|t|notebook=Some(t))
            );
    }
    let notebook = notebook.unwrap();
    let drawing = drawing.unwrap();

    let notebook_widgets = notebook.get_children();
    let mut listbox: Option<gtk::ListBox> = None;
    for w in notebook_widgets{
        w.downcast::<gtk::ListBox>()
            .map(|t|listbox=Some(t))
            .unwrap();
    }
    let listbox = listbox.unwrap();

    let listbox_widgets = listbox.get_children();
    let mut first: Option<gtk::ListBoxRow> = None;
    for w in listbox_widgets{
        w.downcast::<gtk::ListBoxRow>()
            .map(|t|first=Some(t))
            .unwrap();
    }
    let first = first.unwrap();

    let first_widgets = first.get_children();
    let mut statistics: Option<gtk::Label> = None;
    for w in first_widgets{
        w.downcast::<gtk::Label>()
            .map(|t|statistics=Some(t))
            .unwrap();
    }
    let statistics = statistics.unwrap();

    (drawing, notebook, listbox, statistics)
}

fn draw(window: &gtk::Paned, context: &cairo::Context)->Inhibit{
    
    let (drawing, notebook, listbox, statistics) = draw_getwidgets(window);
    let off_screen = ImageSurface::create(Format::ARgb32, drawing.get_allocated_width(), drawing.get_allocated_height()).unwrap();
    let off_context = cairo::Context::new(&off_screen);


    let mut delta_time = 0;
    {
        let mut game = game::GAME.write().unwrap();

        //deltatimeを計算
        delta_time = game.last_time.elapsed().as_millis();
        game.time += delta_time;
        game.last_time = Instant::now();
        // println!("{}", game.time);
    }
    {
        let mut timer = Instant::now();

        let game = game::GAME.read().unwrap();
        let livings = game.world.livings.read().unwrap();
        for li in &*livings{
            let mut li = li.lock().unwrap();
            li.update(delta_time);
        }
        let mut painted: Vec<Vec<u8>> = Vec::new();
        let width = off_screen.get_width();
        let height = off_screen.get_height();
        for _ in 0..width{
            let mut v: Vec<u8> = Vec::with_capacity(height as usize);
            v.resize(height as usize, 0);
            painted.push(v);
        }
        let time_update = timer.elapsed().as_millis();
        timer = Instant::now();
        // println!("----------------------------------------");
        for li in &*livings{
            let mut li = li.lock().unwrap();
            li.draw(&off_context, &mut painted);
        }

        
        // let n_threads = 4;
        // let unit = livings.len()/n_threads;
        // let width: usize = off_screen.get_width() as usize;
        // let height: usize = off_screen.get_height() as usize;
        // let mut threads : Vec<JoinHandle<Vec<u8>>> = Vec::new();
        // for i in 0..n_threads{
        //     let th = thread::spawn(move || {
        //         let game = game::GAME.read().unwrap();
        //         let livings = game.world.livings.read().unwrap();
        //         let sep = &livings[i * unit..(i+1)*unit];
        //         // let mut image: RgbaImage = ImageBuffer::new(width as u32, height as u32);
        //         let mut vec: Vec<u8> = Vec::with_capacity(width * height * 4);
        //         unsafe{
        //             vec.set_len(vec.capacity());
        //         }
        //         for li in sep{
        //             li.lock().unwrap().draw(&mut vec, (width, height));
        //         }
        //         vec
        //     });
        //     threads.push(th);
        // }
        
        // for t in threads{
        //     let btimer = Instant::now();
        //     let image = t.join().unwrap();
        //     println!("{}", btimer.elapsed().as_millis());
        //     // let data = image.pixels();
        //     // let mut vec: Vec<u8> = Vec::with_capacity(width as usize * height as usize);
            
        //     // for rgba in data{
        //     //     vec.push(rgba[0]);vec.push(rgba[1]);vec.push(rgba[2]);vec.push(rgba[3]);
        //     // }
             
             
            
            
        //     let surface = cairo::ImageSurface::create_for_data(image, Format::ARgb32, width as i32, height as i32, off_screen.get_stride()).unwrap();
        //     off_context.set_source_surface(&surface, 0f64, 0f64);
        //     off_context.paint();
            
        // }
        let time_draw = timer.elapsed().as_millis();
        statistics.set_label(&format!("総数: {}\nupdate: {}\ndraw:{}", livings.len(), time_update, time_draw));

    }
    {
        let game = game::GAME.read().unwrap();

        let livings = game.world.livings.read().unwrap();
        for li in &*livings{
            let l = li.lock().unwrap();
            if l.is_dead(){
                let game = game::GAME.read().unwrap();
                game.world.remove_living(li);
            }
        }
    }
    {
        let mut game = game::GAME.write().unwrap();
        
        game.world.flush_modify_livings();
    }
    
    off_screen.flush();
    context.set_source_surface(&off_screen, 0 as f64, 0 as f64);
    context.paint();

    Inhibit(false)
}



fn build_ui(application: &gtk::Application) {
    //glade読み込み
    let glade_src = include_str!("main.glade");
    let builder = Builder::new_from_string(glade_src);

    //windowの設定
    let window: gtk::ApplicationWindow = builder.get_object("window1").expect("glade file is incomplete");
    window.set_application(application);
    window.set_title("First GTK+ Program");

    //drawingareaの設定
    let drawingarea: gtk::DrawingArea = builder.get_object("drawingarea1").expect("drawingarea1");

    //drawingareaの設定
    let paned: gtk::Paned = builder.get_object("paned").unwrap();

    
    //30FPSで描画するように設定
    paned.connect_draw(draw);
    gtk::timeout_add(1000/30, move||{&paned.queue_draw();gtk::Continue(true)});

    //最初の生物をばらまく
    let mut game = game::GAME.write().unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..10{
        let x: i64 = rng.gen_range(1,200);
        let y: i64 = rng.gen_range(1,200);
        let li: Arc<Mutex<Living>> = Arc::new(Mutex::new(Grass::new(10000, (x, y))));
        game.world.add_living(&li);
    }
    game.world.flush_modify_livings();
    
    
    window.show_all();
}

fn main() {
    let application = gtk::Application::new
        ("jp.outlook.taby", Default::default()).expect("Initialization Failed.");
    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
