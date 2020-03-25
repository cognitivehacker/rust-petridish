
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

#[derive(Debug)]
struct Player {
   x: i32,
   y: i32,
   radius: i8,
}

fn main() {

    println!("");

    game_loop();

}

fn game_loop(){

    let mut player = Player{
        x: 100,
        y: 100,
        radius: 20,
    };


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
        "Petridish",
        800,
        600,
    ).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    loop {

        canvas.set_draw_color(Color::RGB(0,0,0));
       // canvas.clear(); // Clear Screen

        canvas.set_draw_color(Color::RGB(255,210,0));
        canvas.fill_rect(Rect::new(player.x, player.y, 10, 10)).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0,1_000_000_000u32/60,));

        player.x += 1;
        player.y += 1;
    }

}