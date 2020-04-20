use macroquad::*;

struct Rocket {
    vel: Vec2,
    pos: Vec2,
}

struct House {
    house_angle: f32,
    house_length: f32,
    house_tall: f32,
}

struct Planet {
    radius: f32,
    pos: Vec2,
    houses: Vec<House>,
}
impl Planet {
    fn new(radius: f32, pos: Vec2) -> Planet {
        let mut houses = vec![];
        for i in 0..1500 {
            let house_angle = rand::gen_range(0., std::f32::consts::PI * 2.);
            let mut house_tall = rand::gen_range(20., 120.);
            let mut house_length = rand::gen_range(20., 320.);

            if rand::gen_range(0, 2)
                * rand::gen_range(0, 2)
                * rand::gen_range(0, 2)
                * rand::gen_range(0, 2)
                * rand::gen_range(0, 2)
                == 1
            {
                house_tall += rand::gen_range(500., 1000.);
                house_length += rand::gen_range(300., 700.);
            }

            if house_angle > std::f32::consts::FRAC_PI_2 + 0.01
                || house_angle < std::f32::consts::FRAC_PI_2 - 0.01
            {
                houses.push(House {
                    house_angle,
                    house_length,
                    house_tall,
                })
            }
        }

        Planet {
            pos,
            radius,
            houses,
        }
    }
}
impl Planet {
    fn draw(&self, line_thikness: f32) {
        draw_circle_lines(
            self.pos.x(),
            self.pos.y(),
            self.radius,
            line_thikness,
            WHITE,
        );
        for house in &self.houses {
            draw_house(self.pos, self.radius, house, line_thikness);
        }
    }
}
struct Particle {
    pos: Vec2,
    vel: Vec2,
    rotation: f32,
    time_left: f32,
}

impl Particle {
    fn random_smoke(pos: Vec2, vel: Vec2) -> Particle {
        Particle {
            pos,
            vel,
            rotation: rand::gen_range(0., 360.),
            time_left: rand::gen_range(0.3, 0.6),
        }
    }
    fn draw(&self) {
        let r = 5.;
        let rx0 = self.pos.x() + self.rotation.to_radians().cos() * r;
        let ry0 = self.pos.y() + self.rotation.to_radians().sin() * r;

        let rx1 = self.pos.x() + (self.rotation + 180.).to_radians().cos() * r;
        let ry1 = self.pos.y() + (self.rotation + 180.).to_radians().sin() * r;

        draw_line(rx0, ry0, rx1, ry1, 2., WHITE);
    }
}

fn draw_house(planet_center: Vec2, planet_radius: f32, house: &House, line_thikness: f32) {
    let House {
        house_angle,
        house_length,
        house_tall,
    } = house;
    let arc_len = 2. * std::f32::consts::PI * planet_radius;
    let angle = house_length / arc_len;

    let a0 = house_angle;
    let a1 = a0 + angle;

    let p0 = vec2(
        planet_center.x() + a0.cos() * planet_radius,
        planet_center.y() + a0.sin() * planet_radius,
    );
    let p1 = vec2(
        planet_center.x() + a0.cos() * (planet_radius + house_tall),
        planet_center.y() + a0.sin() * (planet_radius + house_tall),
    );
    let p2 = vec2(
        planet_center.x() + a1.cos() * (planet_radius + house_tall),
        planet_center.y() + a1.sin() * (planet_radius + house_tall),
    );
    let p3 = vec2(
        planet_center.x() + a1.cos() * planet_radius,
        planet_center.y() + a1.sin() * planet_radius,
    );
    let p4 = vec2(
        planet_center.x() + a1.cos() * planet_radius,
        planet_center.y() + a1.sin() * planet_radius,
    );
    draw_line(p0.x(), p0.y(), p1.x(), p1.y(), line_thikness, WHITE);
    draw_line(p1.x(), p1.y(), p2.x(), p2.y(), line_thikness, WHITE);
    draw_line(p2.x(), p2.y(), p3.x(), p3.y(), line_thikness, WHITE);
    draw_line(p3.x(), p3.y(), p4.x(), p4.y(), line_thikness, WHITE);
}

#[macroquad::main("LD46!!!!")]
async fn main() {
    let texture = load_texture("assets/rocket.png").await;
    set_texture_filter(texture, FilterMode::Linear);

    let mut particles = Vec::new();

    let mut player = Rocket {
        vel: vec2(0., 0.),
        pos: vec2(0., 10.),
    };

    let mut world_camera = Camera2D {
        target: player.pos,
        offset: vec2(0., -0.5),
        zoom: 0.003,
        ..Default::default()
    };
    let mut player_camera = world_camera;
    let planet = Planet::new(10000., vec2(0., -10000.));

    loop {
        let delta = get_frame_time();

        clear_background(BLACK);

        begin_mode_2d(world_camera);

        if is_key_down(KeyCode::W) {
            *player.vel.y_mut() += delta * 0.5;
            particles.push(Particle::random_smoke(
                player.pos + vec2(rand::gen_range(-40., 40.), rand::gen_range(-10., 10.)),
                vec2(rand::gen_range(-2., 2.), rand::gen_range(-100., -1000.)),
            ));
        }

        *player.pos.y_mut() += player.vel.y();

        let min_zoom = 0.002;
        let one_screen_time = 0.5;
        let zoom;
        if player.vel.length() < 0.001 {
            zoom = min_zoom;
        } else {
            let screen_distance = one_screen_time / player.vel.length();
            zoom = (screen_distance / screen_width()).min(min_zoom);
        }
        world_camera.zoom = world_camera.zoom * 0.99 + zoom * 0.01;
        // world_camera.target = player.pos + vec2(0., texture.height() / 2.);
        // player_camera.target = world_camera.target;
        player_camera.zoom = world_camera.zoom.max(0.0006);

        for particle in particles.iter_mut() {
            particle.rotation += delta * 100.;
            particle.pos += particle.vel * delta;
            if particle.pos.y() <= 0. {
                *particle.vel.y_mut() *= -1.;
                particle.time_left = 0.05;
            }
            particle.time_left -= delta;
        }
        particles.retain(|particle| particle.time_left > 0.);

        let p0 = world_camera.screen_to_world(vec2(0.0, 0.));
        let p1 = world_camera.screen_to_world(vec2(5.0 / screen_width(), 0.));
        let line_thickness = (p0 - p1).abs().x();

        planet.draw(line_thickness);

        end_mode_2d();

        begin_mode_2d(player_camera);
        draw_texture_ex(
            texture,
            player.pos.x() - texture.width() / 2.,
            player.pos.y() + texture.height(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture.width(), -texture.height())),
                ..Default::default()
            },
        );
        for particle in particles.iter_mut() {
            particle.draw();
        }

        end_mode_2d();
        next_frame().await
    }
}
