use entities::World;
use entities::components::position::Position;
use map::tile::Tiles;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::ops::Div;
use std::time::{Duration, Instant};
// use crate::map::tile::Tiles;
use crate::maths::transform::Transform;
use crate::maths::vector;
use crate::maths::vector::Vector;

mod assets;
mod entities;
mod map;
mod maths;

const FPS: u32 = 60;

const WINDOW_SIZE: (u32, u32) = (640, 360);
fn render(canvas: &mut WindowCanvas) {
    canvas.clear();

    canvas.present();
}

fn tick() {
    // tick here
}
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window_scale = 1.0;
    let window = video_subsystem
        .window(
            "game",
            (WINDOW_SIZE.0 as f32 * window_scale) as u32,
            (WINDOW_SIZE.1 as f32 * window_scale) as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    // let mut texture_manager = assets::TextureManager::new(&texture_creator);
    let mut texture_atlas_manager = assets::TextureAtlasManager::new(&texture_creator);
    let test_atlas = texture_atlas_manager
        .load("tiles")
        .expect("texture should exist");
    let test_region = test_atlas.get_region("floor").unwrap();
    let test_sprite = test_region.unwrap_single();
    let mut ticks = 0;
    let mut event_pump = sdl_context.event_pump()?;
    let mut last_time = Instant::now();
    let time_per_tick = Duration::from_secs(1) / FPS;
    let mut delta = 0.0;
    let mut timer = 0;

    let tiles = Tiles::init(&texture_atlas_manager.load("tiles").unwrap());
    let map = map::Map::new("assets/rooms/room.rm", &tiles)?;

    let mut world = World::init();
    let entity = world.new_entity();
    world.set_component(entity, Position(1.0, 4.0))?;
    'running: loop {
        let now = Instant::now();
        delta += (now - last_time).as_nanos() as f32 / time_per_tick.as_nanos() as f32;
        timer += (now - last_time).as_nanos();
        last_time = now;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::KpPlus),
                    ..
                } => {
                    window_scale += 0.5;
                    canvas
                        .window_mut()
                        .set_size(
                            (WINDOW_SIZE.0 as f32 * window_scale) as u32,
                            (WINDOW_SIZE.1 as f32 * window_scale) as u32,
                        )
                        .map_err(|e| e.to_string())?;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::KpMinus),
                    ..
                } => {
                    window_scale -= 0.5;
                    canvas
                        .window_mut()
                        .set_size(
                            (WINDOW_SIZE.0 as f32 * window_scale) as u32,
                            (WINDOW_SIZE.1 as f32 * window_scale) as u32,
                        )
                        .map_err(|e| e.to_string())?;
                }
                /* Event::MouseButtonDown { x, y, .. } => {
                    let pos = Vector::new(x as f32, y as f32, 0.0);

                    let viewport = canvas.viewport();
                    let x_scale = viewport.width() as f32 / camera.camera_size.0 as f32;
                    let y_scale = viewport.height() as f32 / camera.camera_size.1 as f32;
                    let scale = if x_scale < y_scale { x_scale } else { y_scale };
                    let pos = pos - ((viewport.width() as f32 / 2.0 - camera.camera_size.0 as f32 / 2.0 * scale), (viewport.height() as f32 / 2.0 - camera.camera_size.1 as f32 / 2.0 * scale)).into();
                    let pos = pos / camera.transform.scale / scale;
                    let pos = pos + camera.transform.pos;
                    camera.center(pos);
                    println!("Mouse click at: {:?}", pos);
                }
                */
                _ => {}
            }
        }
        // Tick
        if delta >= 1.0 {
            tick();
            ticks += 1;
            delta -= 1.0;
            // let camera_offset = camera.transform;
            // Render
            canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
            canvas.clear();
            map.render(&mut canvas).ok();
            canvas.present();
            // render(&mut canvas);
        }
        if timer >= 1_000_000_000 {
            println!("Ticks and Frames: {}", ticks);
            ticks = 0;
            timer = 0;
        }
    }

    Ok(())
}
