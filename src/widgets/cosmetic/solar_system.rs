// most of this code is taken from https://github.com/hecrj/iced/blob/master/examples/solar_system/src/main.rs

use iced::canvas::{Cursor, Path, Stroke};
use iced::{canvas, window, Color, Point, Rectangle, Size, Vector};
use std::time::Instant;

#[derive(Debug)]
pub struct SolarSystem {
    space_cache: canvas::Cache,
    system_cache: canvas::Cache,
    cursor_position: Point,
    start: Instant,
    now: Instant,
    stars: Vec<(Point, f32)>,
}

impl Clone for SolarSystem {
    fn clone(&self) -> Self {
        Self {
            space_cache: Default::default(),
            system_cache: Default::default(),
            cursor_position: self.cursor_position.clone(),
            start: self.start.clone(),
            now: self.now.clone(),
            stars: self.stars.clone(),
        }
    }
}

impl SolarSystem {
    const SUN_RADIUS: f32 = 70.0;
    const ORBIT_RADIUS: f32 = 150.0;
    const EARTH_RADIUS: f32 = 12.0;
    const MOON_RADIUS: f32 = 4.0;
    const MOON_DISTANCE: f32 = 28.0;

    pub fn new() -> SolarSystem {
        let (width, height) = window::Settings::default().size;
        let now = Instant::now();
        SolarSystem {
            space_cache: Default::default(),
            system_cache: Default::default(),
            cursor_position: Point::ORIGIN,
            start: now,
            now,
            stars: Self::generate_stars(width, height),
        }
    }

    pub fn tick(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }

    fn generate_stars(width: u32, height: u32) -> Vec<(Point, f32)> {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        (0..100)
            .map(|_| {
                (
                    Point::new(
                        rng.gen_range((-(width as f32) / 2.0)..(width as f32 / 2.0)),
                        rng.gen_range((-(height as f32) / 2.0)..(height as f32 / 2.0)),
                    ),
                    rng.gen_range((0.5..1.0)),
                )
            })
            .collect()
    }
}

impl<Message> canvas::Program<Message> for SolarSystem {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<canvas::Geometry> {
        use std::f32::consts::PI;

        let background = self.space_cache.draw(bounds.size(), |frame| {
            let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());

            let stars = Path::new(|path| {
                for (p, size) in &self.stars {
                    path.rectangle(*p, Size::new(*size, *size));
                }
            });

            frame.fill(&space, Color::BLACK);

            frame.translate(frame.center() - Point::ORIGIN);
            frame.fill(&stars, Color::WHITE);
        });

        let system = self.system_cache.draw(bounds.size(), |frame| {
            let center = frame.center();

            let sun = Path::circle(center, Self::SUN_RADIUS);
            let orbit = Path::circle(center, Self::ORBIT_RADIUS);

            frame.fill(&sun, Color::from_rgb8(0xF9, 0xD7, 0x1C));
            frame.stroke(
                &orbit,
                Stroke {
                    width: 1.0,
                    color: Color::from_rgba8(0, 153, 255, 0.1),
                    ..Stroke::default()
                },
            );

            let elapsed = self.now - self.start;
            let rotation = (2.0 * PI / 60.0) * elapsed.as_secs() as f32
                + (2.0 * PI / 60_000.0) * elapsed.subsec_millis() as f32;

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));
                frame.rotate(rotation);
                frame.translate(Vector::new(Self::ORBIT_RADIUS, 0.0));

                let earth = Path::circle(Point::ORIGIN, Self::EARTH_RADIUS);
                let shadow = Path::rectangle(
                    Point::new(0.0, -Self::EARTH_RADIUS),
                    Size::new(Self::EARTH_RADIUS * 4.0, Self::EARTH_RADIUS * 2.0),
                );

                frame.fill(&earth, Color::from_rgb8(0x6B, 0x93, 0xD6));

                frame.with_save(|frame| {
                    frame.rotate(rotation * 10.0);
                    frame.translate(Vector::new(0.0, Self::MOON_DISTANCE));

                    let moon = Path::circle(Point::ORIGIN, Self::MOON_RADIUS);
                    frame.fill(&moon, Color::WHITE);
                });

                frame.fill(
                    &shadow,
                    Color {
                        a: 0.7,
                        ..Color::BLACK
                    },
                );
            });
        });

        vec![background, system]
    }
}
