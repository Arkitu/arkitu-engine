use crate::grid2d::{Color, Grid2D, Pos2D};

pub trait Shape2DTrait {
    fn render(&self, grid: &mut Grid2D);
}
pub enum Shape2D {
    Line(Line),
    EmptyTriangle(EmptyTriangle)
}
impl Shape2DTrait for Shape2D {
    fn render(&self, grid: &mut Grid2D) {
        match self {
            Self::Line(l) => l.render(grid),
            Self::EmptyTriangle(t) => t.render(grid)
        }
    }
}

pub struct Line {
    pub color: Color,
    pub vertices: (Pos2D, Pos2D)
}
impl Shape2DTrait for Line {
    fn render(&self, grid: &mut Grid2D) {
        let (a, b) = self.vertices;
        let x_right = b.0 > a.0;
        let y_down = b.1 > a.1;
        let l = b.0.abs_diff(a.0);
        let h = b.1.abs_diff(a.1);
        let mut x = 0;
        let mut y = 0;
        while x < l && y < h {
            if l > h {
                if (x*h)/l >= y {
                    y+=1;
                    x+=1;
                } else {
                    x+=1;
                }
            } else {
                if (y*l)/h >= x {
                    y+=1;
                    x+=1;
                } else {
                    y+=1;
                }
            }
            dbg!((x, y));
            grid.set((
                if x_right { a.0+x } else { a.0-x },
                if y_down { a.1+y } else { a.1-y }
            ), self.color)
        }
    }
}

pub struct EmptyTriangle {
    pub color: Color,
    pub vertices: (Pos2D, Pos2D, Pos2D)
}
impl Shape2DTrait for EmptyTriangle {
    fn render(&self, grid: &mut Grid2D) {
        Line {
            color: self.color,
            vertices: (self.vertices.0, self.vertices.1)
        }.render(grid);
        Line {
            color: self.color,
            vertices: (self.vertices.1, self.vertices.2)
        }.render(grid);
        Line {
            color: self.color,
            vertices: (self.vertices.2, self.vertices.0)
        }.render(grid);
    }
}