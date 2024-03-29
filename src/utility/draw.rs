use eframe::{
    egui,
    epaint::{CircleShape, RectShape},
};
use egui::*;
#[derive(Clone)]
pub enum Shape {
    Circle(epaint::CircleShape),
    Rect(epaint::RectShape),
    Arrow(ArrowShape),
    Text(WriteShape),
    FreeHand(LineSegment),
}

//Arrow is not implemented as struct but it has only a function
#[derive(Clone)]
pub struct ArrowShape {
    origin: Pos2,
    vec: Vec2,
    stroke: Stroke,
}

impl ArrowShape {
    pub fn new(origin: Pos2, vec: Vec2, stroke: Stroke) -> Self {
        return ArrowShape {
            origin,
            vec,
            stroke,
        };
    }

    pub fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();
        painter.arrow(self.origin, self.vec, self.stroke);
    }
}
#[derive(Clone)]
pub struct WriteShape {
    origin: Pos2,
    anchor: Align2,
    text: String,
    font_id: FontId,
    text_color: Color32,
}

impl WriteShape {
    pub fn new(
        origin: Pos2,
        anchor: Align2,
        text: String,
        font_id: FontId,
        text_color: Color32,
    ) -> Self {
        return WriteShape {
            origin,
            anchor,
            text,
            font_id,
            text_color,
        };
    }

    pub fn write(&self, ui: &mut Ui) {
        let painter = ui.painter();
        painter.text(
            self.origin,
            self.anchor,
            self.text.to_string(),
            self.font_id.clone(),
            self.text_color,
        );
    }
}

#[derive(Clone)]
pub struct LineSegment {
    points: [Pos2; 2],
    stroke: Stroke,
}

impl LineSegment {
    pub fn new(points: [Pos2; 2], stroke: Stroke) -> Self {
        return LineSegment { points, stroke };
    }
    pub fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();
        painter.line_segment(self.points, self.stroke);
    }
}

#[derive(Clone, Copy)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        return Rgb { red, green, blue };
    }

    pub fn convert_in_color_32(self) -> Color32 {
        return Color32::from_rgb(self.red, self.green, self.blue);
    }
}
#[derive(Clone, Copy)]
pub struct ProperDraw {
    pub draw: Option<i32>,
    pub color: Rgb,
    pub filled: bool,
    pub color_fill: Rgb,
    pub width: f32,
}

impl ProperDraw {
    pub fn new(draw: Option<i32>, color: Rgb, filled: bool, color_fill: Rgb, width: f32) -> Self {
        return ProperDraw {
            draw,
            color,
            filled,
            color_fill,
            width,
        };
    }
}

pub fn create_figure(
    vec_shape: &mut Vec<Shape>,
    ctx: &egui::Context,
    property: ProperDraw,
    text: String,
    draw_dim_variable: &mut i32,
    font: FontFamily,
    x: f32,
    y: f32,
    image_width: f32,
    image_height: f32,
) {
    if ctx.input(|i| i.pointer.primary_clicked()) {
        *draw_dim_variable = 0
    }
    if ctx.input(|i| i.pointer.primary_down()) {
        let default_pos = Pos2::new(-1.0, -1.0);
        let mut pos_start = ctx.input(|i| i.pointer.press_origin().unwrap());
        let pos_mouse:Pos2 = ctx.input(|i| i.pointer.hover_pos().unwrap_or(default_pos));
        let mut fill = Color32::TRANSPARENT;
        if property.filled {
            fill = property.color_fill.convert_in_color_32();
        }
        let x1 = x / 2.0;
        let x2 = (x / 2.0) + image_width;
        let y1 = y;
        let y2 = (y/2.0) + image_height;
        if pos_start.y > y1
            && pos_start.y < y2
            && pos_start.x > x1
            && pos_start.x < x2
            && pos_mouse.y > y1
            && pos_mouse.y < y2
            && pos_mouse.x > x1
            && pos_mouse.x < x2
            && pos_mouse.x!=-1.
            && pos_mouse.y!=-1.
        {
            match property.draw.unwrap() {
                0 => {
                    if check_valid_dimension_figure(pos_start, pos_mouse, x1, y1, x2, y2, property.width, 0) {
                        let circle = epaint::CircleShape {
                            center: pos_start,
                            radius: distance_between_two_points(pos_start, pos_mouse),
                            fill: fill,
                            stroke: Stroke {
                                width: property.width,
                                color: property.color.convert_in_color_32(),
                            },
                        };
                        if *draw_dim_variable == 0 {
                            vec_shape.push(Shape::Circle(circle));
                            *draw_dim_variable = 1;
                           
                        } else {
                            let i = vec_shape.len();
                            vec_shape[i - 1] = Shape::Circle(circle);
                        }

                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            *draw_dim_variable = 0;
                        }
                    }
                }
                1 => {
                    if check_valid_dimension_figure(pos_start, pos_mouse, x1, y1, x2, y2, property.width, 1) {
                        let rectangle = epaint::RectShape::new(
                            Rect {
                                min: pos_start,
                                max: epaint::pos2(pos_mouse.x, pos_mouse.y),
                            },
                            Rounding::ZERO,
                            fill,
                            Stroke {
                                width: property.width,
                                color: property.color.convert_in_color_32(),
                            },
                        );

                        if *draw_dim_variable == 0 {
                            vec_shape.push(Shape::Rect(rectangle));
                            *draw_dim_variable = 1;
                        } else {
                            let i = vec_shape.len();
                            vec_shape[i - 1] = Shape::Rect(rectangle);
                        }

                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            *draw_dim_variable = 0;
                        }
                    }
                }
                2 => {
                    let arrow = ArrowShape::new(
                        pos_start,
                        Vec2 {
                            x: pos_mouse.x - pos_start.x,
                            y: pos_mouse.y - pos_start.y,
                        },
                        Stroke {
                            width: property.width,
                            color: property.color.convert_in_color_32(),
                        },
                    );

                    if *draw_dim_variable == 0 {
                        vec_shape.push(Shape::Arrow(arrow));
                        *draw_dim_variable = 1;
                    } else {
                        let i = vec_shape.len();
                        vec_shape[i - 1] = Shape::Arrow(arrow);
                    }

                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        *draw_dim_variable = 0;
                    }
                }
                3 => {
                    let text = WriteShape::new(
                        pos_start,
                        Align2::LEFT_TOP,
                        text,
                        FontId {
                            size: property.width,
                            family: font,
                        },
                        property.color.convert_in_color_32(),
                    );
                    vec_shape.push(Shape::Text(text))
                }
                4 => {
                    if *draw_dim_variable == 1 {
                        let i = vec_shape.len();
                        let shape = vec_shape[i - 1].clone();
                        match shape {
                            Shape::FreeHand(shape) => pos_start = shape.points[1],
                            _ => print!("Error"),
                        }
                    } else if *draw_dim_variable == 0 {
                        *draw_dim_variable = 1;
                    }

                    if ctx.input(|i| i.key_pressed(Key::Enter)) {
                        *draw_dim_variable = -1;
                    }

                    if *draw_dim_variable == 1 || *draw_dim_variable == 0 {
                        let line = LineSegment::new(
                            [pos_start, pos_mouse],
                            Stroke {
                                width: property.width,
                                color: property.color.convert_in_color_32(),
                            },
                        );
                        vec_shape.push(Shape::FreeHand(line));
                      
                    }
                }
                _ => print!("Error"),
            }
        }
    }
}

pub fn draw(ui: &mut Ui, vec_shape: &mut Vec<Shape>) {
    for shape in vec_shape.iter() {
        match shape {
            Shape::Circle(shape) => draw_circle(ui, shape),
            Shape::Rect(shape) => draw_rect(ui, shape),
            Shape::Arrow(shape) => shape.draw(ui),
            Shape::Text(shape) => shape.write(ui),
            Shape::FreeHand(shape) => shape.draw(ui),
        }
    }
}

fn draw_circle(ui: &mut Ui, shape: &CircleShape) {
    let painter = ui.painter();
    painter.circle(shape.center, shape.radius, shape.fill, shape.stroke);
}

pub fn draw_rect(ui: &mut Ui, shape: &RectShape) {
    let painter = ui.painter();
    painter.rect(shape.rect, shape.rounding, shape.fill, shape.stroke)
}

fn distance_between_two_points(p1: Pos2, p2: Pos2) -> f32 {
    let distance = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
    return distance;
}

fn check_valid_dimension_figure(pos_start: Pos2,pos_mouse: Pos2, x1: f32,y1: f32,x2: f32,y2: f32,width: f32, figure: i32) -> bool {
    let mut ray = 0.;
    match figure {
        0 => ray = distance_between_two_points(pos_start, pos_mouse),
        _=> ()
    }
    if pos_start.x - ray - width / 2. < x1 {
        return false;
    } else if pos_start.y - ray - width / 2. < y1 {
        return false;
    } else if pos_start.x + ray + width / 2. > x2 {
        return false;
    } else if pos_start.y + ray + width / 2. > y2 {
        return false;
    }
    return true;
}