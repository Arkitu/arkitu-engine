use grid2d::{Color, Grid2D, HEIGHT, WIDTH};
use minifb::{Key, Window, WindowOptions};

mod shape2d;
use shape2d::{EmptyTriangle, Line, Shape2D, Shape2DTrait};
mod grid2d;

struct App {
    window: Window,
    shapes: Vec<Shape2D>
}
impl App {
    fn new() -> Self {
        let mut window = Window::new(
            "Arkitu Engine - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        ).expect("Cannot create new window");

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        
        Self {
            window,
            shapes: Vec::new()
        }
    }
    fn run(&mut self) {
        let mut grid = Grid2D::default();
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.render(&mut grid);
            self.window.update_with_buffer(&grid.0, WIDTH, HEIGHT).unwrap();
        }
    }
    fn render(&mut self, buffer:&mut Grid2D) {
        for shape in self.shapes.iter() {
            shape.render(buffer);
        }
    }
}

fn main() {
    let mut app = App::new();
    app.shapes.push(Shape2D::EmptyTriangle(EmptyTriangle {
        color: Color::new(127, 0, 127),
        vertices: (
            (200, 300),
            (100, 20),
            (500, 3)
        )
    }));
    app.run();
}