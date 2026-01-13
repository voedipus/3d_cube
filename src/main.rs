use iced::{
    widget::{button, container, row, text},
    Padding,
};
use std::time::{Duration, Instant};

use iced::{
    mouse,
    widget::{
        canvas::{self, Frame, Geometry, Stroke},
        column, Canvas,
    },
    Element, Length, Rectangle, Renderer, Theme,
};

struct CubeApp {
    last: Instant,
    angle: f32,
    rotation_speed: f32,
    rotate_xz: bool,
    rotate_yz: bool,
    rotate_xy: bool,
}

impl Default for CubeApp {
    fn default() -> Self {
        Self {
            last: Instant::now(),
            angle: 1.0,
            rotation_speed: 1.0,
            rotate_xz: true,
            rotate_yz: false,
            rotate_xy: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Tick,
    IncreaseAngle,
    DecreaseAngle,
    ToggleRotateXZ,
    ToggleRotateYZ,
    ToggleRotateXY,
}

impl CubeApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                let now = Instant::now();
                let dt = now.duration_since(self.last).as_secs_f32();
                self.last = now;
                self.angle += dt * self.rotation_speed;
            }
            Message::IncreaseAngle => {
                self.rotation_speed += 1.0;
            }
            Message::DecreaseAngle => {
                self.rotation_speed -= 1.0;
            }
            Message::ToggleRotateXZ => {
                self.rotate_xz = !self.rotate_xz;
            }
            Message::ToggleRotateYZ => {
                self.rotate_yz = !self.rotate_yz;
            }
            Message::ToggleRotateXY => {
                self.rotate_xy = !self.rotate_xy;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let cube = Cube {
            angle: self.angle,
            rotate_xz: self.rotate_xz,
            rotate_yz: self.rotate_yz,
            rotate_xy: self.rotate_xy,
            ..Default::default()
        };
        let canvas = Canvas::new(cube).width(Length::Fill).height(Length::Fill);
        let rotation_text = text("Rotation Speed");
        let rotation_speed_text = text(format!("{:.1}", self.rotation_speed)).size(20);
        let increase_btn = button("Increase").on_press(Message::IncreaseAngle);
        let decrease_btn = button("Decrease").on_press(Message::DecreaseAngle);
        let middlerow = container(row![rotation_text]).center_x(Length::Fill);
        let row = row![decrease_btn, rotation_speed_text, increase_btn].spacing(20);
        let centered_row = container(row)
            .center_x(Length::Fill)
            .padding(Padding::from(20));
        let rotate_xz_btn = button("Rotate XZ").on_press(Message::ToggleRotateXZ);
        let rotate_yz_btn = button("Rotate YZ").on_press(Message::ToggleRotateYZ);
        let rotate_xy_btn = button("Rotate XY").on_press(Message::ToggleRotateXY);
        let rotate_row = container(row![rotate_xz_btn, rotate_yz_btn, rotate_xy_btn].spacing(20))
            .center_x(Length::Fill)
            .padding(Padding::from(20));
        column![canvas, middlerow, centered_row, rotate_row].into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_millis(20)).map(|_| Message::Tick)
    }
}

fn main() -> iced::Result {
    iced::application(CubeApp::default, CubeApp::update, CubeApp::view)
        .subscription(CubeApp::subscription)
        .run()
}

#[derive(Debug, Clone)]
struct Cube {
    dz: f32,
    angle: f32,
    rotate_xz: bool,
    rotate_yz: bool,
    rotate_xy: bool,
    vs: [(f32, f32, f32); 8],
    fs: [&'static [u8]; 6],
}

const CUBE_VERTICES: [(f32, f32, f32); 8] = [
    (0.25, 0.25, 0.25),
    (-0.25, 0.25, 0.25),
    (-0.25, -0.25, 0.25),
    (0.25, -0.25, 0.25),
    (0.25, 0.25, -0.25),
    (-0.25, 0.25, -0.25),
    (-0.25, -0.25, -0.25),
    (0.25, -0.25, -0.25),
];

const CUBE_FACES: [&[u8]; 6] = [
    &[0, 1, 2, 3],
    &[4, 5, 6, 7],
    &[0, 4],
    &[1, 5],
    &[2, 6],
    &[3, 7],
];

impl Default for Cube {
    fn default() -> Self {
        Self {
            angle: 1.0,
            dz: 1.0,
            rotate_xz: true,
            rotate_yz: false,
            rotate_xy: false,
            vs: CUBE_VERTICES,
            fs: CUBE_FACES,
        }
    }
}

impl Cube {
    fn screen(&self, (x, y): (f32, f32), bounds: Rectangle) -> iced::Point {
        iced::Point::new(
            ((x + 1.0) / 2.0) * bounds.width,
            (1.0 - ((y + 1.0) / 2.0)) * bounds.height,
        )
    }

    fn project(&self, (x, y, z): (f32, f32, f32)) -> (f32, f32) {
        (x / z, y / z)
    }

    fn translate_z(&self, (x, y, z): (f32, f32, f32), dz: f32) -> (f32, f32, f32) {
        (x, y, z + dz)
    }

    fn rotate_xz(&self, (x, y, z): (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
        let cos = angle.cos();
        let sin = angle.sin();
        let new_x = x * cos - z * sin;
        let new_z = x * sin + z * cos;
        (new_x, y, new_z)
    }

    fn rotate_yz(&self, (x, y, z): (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
        let cos = angle.cos();
        let sin = angle.sin();
        let new_y = y * cos - z * sin;
        let new_z = y * sin + z * cos;
        (x, new_y, new_z)
    }

    fn rotate_xy(&self, (x, y, z): (f32, f32, f32), angle: f32) -> (f32, f32, f32) {
        let cos = angle.cos();
        let sin = angle.sin();
        let new_x = x * cos - y * sin;
        let new_y = x * sin + y * cos;
        (new_x, new_y, z)
    }

    fn rotate(&self, (x, y, z): (f32, f32, f32)) -> (f32, f32, f32) {
        let (x, y, z) = if self.rotate_xz {
            self.rotate_xz((x, y, z), self.angle)
        } else {
            (x, y, z)
        };
        let (x, y, z) = if self.rotate_yz {
            self.rotate_yz((x, y, z), self.angle)
        } else {
            (x, y, z)
        };
        let (x, y, z) = if self.rotate_xy {
            self.rotate_xy((x, y, z), self.angle)
        } else {
            (x, y, z)
        };

        (x, y, z)
    }
}

impl<Message> canvas::Program<Message> for Cube {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        for v in self.vs {
            let (x, y, z) = self.rotate(v);
            let screen = self.screen(self.project(self.translate_z((x, y, z), self.dz)), bounds);
            let circle = canvas::Path::circle(screen, 5.0);
            frame.fill(&circle, iced::Color::WHITE);
        }

        for f in self.fs {
            for i in 0..f.len() {
                let a = self.vs[f[i] as usize];
                let b = self.vs[f[(i + 1) % f.len()] as usize];
                let (x1, y1, z1) = self.rotate(a);
                let (x2, y2, z2) = self.rotate(b);
                let p1 = self.screen(
                    self.project(self.translate_z((x1, y1, z1), self.dz)),
                    bounds,
                );
                let p2 = self.screen(
                    self.project(self.translate_z((x2, y2, z2), self.dz)),
                    bounds,
                );
                let line = canvas::Path::line(p1, p2);
                let stroke = Stroke::default().with_color(iced::Color::WHITE);
                frame.stroke(&line, stroke);
            }
        }

        vec![frame.into_geometry()]
    }
}
