
extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use sdl2::keyboard::Keycode;

#[derive(Debug)]
struct Player {
   x: i32,
   y: i32,
   radius: i8,
}

fn main() {

    println!("");

    let player = Player{
        x: 100,
        y: 100,
        radius: 20,
    };

    game_loop(player);
}

fn game_loop(mut player: Player){

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
        "Petridish",
        800,
        600,
    ).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255,210,0));
        canvas.fill_rect(Rect::new(player.x, player.y, 10, 10)).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'gameloop
                },
                
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player.y -= 5;
                },

                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player.y += 5;
                },

                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player.x -= 5;
                },

                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    player.x += 5;
                },

                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0,1_000_000_000u32/60,));

    }

}

