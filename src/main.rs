mod modeling;
mod scene;

use scene::scene;
use sfml::graphics::{
    RenderWindow,
    RenderTarget,
    Color,
    Shader,
    RenderStates,
    Transformable,
    RectangleShape
};
use sfml::window::{Style, Event};

const WIN_SIZE: (u32, u32) = (800, 600);
const WIN_SIZE_F32: (f32, f32) = (WIN_SIZE.0 as f32, WIN_SIZE.1 as f32);
const VERTEX_SHADER_PATH: &str = "shaders/vertex.glsl";
const FRAGMENT_SHADER_PATH: &str = "shaders/fragment.glsl";

fn main() {
    // Initialize shaders
    if !Shader::is_available() {
        panic!();
    }

    let mut shaders = Shader::from_file_vert_frag(VERTEX_SHADER_PATH, FRAGMENT_SHADER_PATH).unwrap();

    // Create the window
    let mut window = RenderWindow::new(
        WIN_SIZE,
        "CSE 472 | Final Project | Joshua Austin",
        Style::CLOSE,
        &Default::default()
    );
    window.set_framerate_limit(5);

    // Create the drawing surface
    let mut screen = RectangleShape::new();
    screen.set_position((0.0, 0.0));
    screen.set_size(WIN_SIZE_F32);
    
    // Run the render loop
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let Event::Closed = event {
                window.close()
            }
        }
        
        window.clear(Color::CYAN);
        
        let mut render_states = RenderStates::default();
        scene(&mut shaders);
        shaders.set_uniform_ivec2("iResolution", (WIN_SIZE.0 as i32, WIN_SIZE.1 as i32).into());
        render_states.set_shader(Some(&shaders));
        
        window.draw_with_renderstates(&screen, &render_states);

        window.display();
    }
}
