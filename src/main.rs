extern crate sdl2; 

use std::env;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::mixer::{InitFlag as AudioInitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

mod logging;
 
static SCREEN_WIDTH: u32 = 512;
static SCREEN_HEIGHT: u32 = 448;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    let sdl_context = sdl2::init().unwrap();
    let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video().unwrap();
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let sdlaudio_context = sdl_context.audio()?;

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;

    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    let _mixer_context = sdl2::mixer::init(AudioInitFlag::MP3 | AudioInitFlag::FLAC | AudioInitFlag::MOD | AudioInitFlag::OGG)?;
    sdl2::mixer::allocate_channels(4);


    {
        let n = sdl2::mixer::get_chunk_decoders_number();
        println!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        }
    }

    {
        let n = sdl2::mixer::get_music_decoders_number();
        println!("available music decoders: {}", n);
        for i in 0..n {
            println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        }
    }

    let mut timer = sdl_context.timer()?;
    
    let window = video_subsystem.window("OMK", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
 

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut font = ttf_context.load_font("regular.ttf", 64)?;
    //let mut font = ttf_context.load_font("demo font.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);


    let ges = texture_creator.load_texture("ges.png")?;

    println!("query spec => {:?}", sdl2::mixer::query_spec());

    let music = sdl2::mixer::Music::from_file("hth.mp3")?;

    fn hook_finished() {
        println!("play ends! from rust cb");
    }

    sdl2::mixer::Music::hook_finished(hook_finished);

    println!("music => {:?}", music);
    println!("music type => {:?}", music.get_type());
    println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
    println!("play => {:?}", music.play(1));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let dest: Rect = rect!(468, 408, 32, 32);

    logging::printmsg("Hello i love u");
    
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        let x = i.to_string();

        //println!("{}", &x);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(&x)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

         // If the example text is too big for the screen, downscale it (and center irregardless)
         let padding = 64;
         let target = get_centered_rect(
             width,
             height,
             SCREEN_WIDTH - padding,
             SCREEN_HEIGHT - padding,
         );
     

        // The rest of the game loop goes here...
        canvas.copy(&ges, None, dest)?;
        canvas.copy(&texture, None, Some(target))?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}