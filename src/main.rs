mod modeling;
mod scene;

use std::time::Instant;

use scene::scene;
use sfml::graphics::{
    RenderWindow,
    RenderTarget,
    Color,
    Shader,
    RenderStates,
    Transformable,
    RectangleShape, Text, Font
};
use sfml::window::{Style, Event};

const WIN_SIZE: (u32, u32) = (800, 600);
const WIN_SIZE_F32: (f32, f32) = (WIN_SIZE.0 as f32, WIN_SIZE.1 as f32);
const VERTEX_SHADER_PATH: &str = "shaders/vertex.glsl";
const FRAGMENT_SHADER_PATH: &str = "shaders/fragment.glsl";
const FONT_PATH: &str = "assets/OCRAEXT.TTF";

fn main() {
    // Acquire resources
    let font = Font::from_file(FONT_PATH).unwrap();

    // Initialize shaders
    if !Shader::is_available() {
        panic!();
    }
    
    let mut shaders = Shader::from_file_vert_frag(VERTEX_SHADER_PATH, FRAGMENT_SHADER_PATH).unwrap();

    // Create the window
    let mut window = RenderWindow::new(
        WIN_SIZE,
        "CSE 472 Final Project",
        Style::CLOSE,
        &Default::default()
    );
    window.set_framerate_limit(60);
    
    // Create the drawing surface
    let mut screen = RectangleShape::new();
    screen.set_position((0.0, 0.0));
    screen.set_size(WIN_SIZE_F32);

    let mut text = Text::new("J.A. CSE472 2023", &font, 16);
    text.set_fill_color(Color::BLACK);
    text.set_position((20., WIN_SIZE_F32.1 - 60.));
    
    let start_time = Instant::now();
    
    // Run the render loop
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close()
            }
        }
        
        window.clear(Color::CYAN);
        
        let mut render_states = RenderStates::default();
        let elapsed_time = Instant::now() - start_time;
        scene(&mut shaders, elapsed_time);
        shaders.set_uniform_ivec2("iResolution", (WIN_SIZE.0 as i32, WIN_SIZE.1 as i32).into());
        // shaders.set_uniform_float("iTime", elapsed_time.as_secs_f32());
        render_states.set_shader(Some(&shaders));
        
        window.draw_with_renderstates(&screen, &render_states);

        window.draw(&text);

        window.display();
    }
}
