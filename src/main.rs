use macroquad::*;

struct Rocket {
    vel: Vec2,
    pos: Vec2,
    rotation: f32,
}

struct House {
    house_angle: f32,
    house_length: f32,
    house_tall: f32,
}

struct Tree {
    tree_angle: f32,
    tree_tall: f32,
    tree_levels: u32,
    tree_length: f32,
}

struct Planet {
    radius: f32,
    pos: Vec2,
    houses: Vec<House>,
    trees: Vec<Tree>,
}
impl Planet {
    fn new(radius: f32, pos: Vec2, buildings: bool, plants: bool) -> Planet {
        let mut houses = vec![];
        let mut trees = vec![];

        if buildings {
            for _ in 0..1500 {
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
        }

        if plants {
            for _ in 0..500 {
                let tree_angle = rand::gen_range(0., std::f32::consts::PI * 2.);
                let tree_tall = rand::gen_range(20., 50.);
                let tree_length = rand::gen_range(20., 320.);
                let tree_levels = rand::gen_range(1i32, 4) as u32;

                if tree_angle > std::f32::consts::FRAC_PI_2 + 0.01
                    || tree_angle < std::f32::consts::FRAC_PI_2 - 0.01
                {
                    trees.push(Tree {
                        tree_angle,
                        tree_tall,
                        tree_levels,
                        tree_length,
                    })
                }
            }
        }
        Planet {
            pos,
            radius,
            houses,
            trees,
        }
    }
}
impl Planet {
    fn draw(&self, line_thikness: f32) {
        draw_circle(self.pos.x(), self.pos.y(), self.radius, BLACK);

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

        for tree in &self.trees {
            draw_tree(self.pos, self.radius, tree, line_thikness);
        }
    }
}
struct Particle {
    pos: Vec2,
    vel: Vec2,
    rotation: f32,
    time_left: f32,
    size: f32,
    thickness: f32,
}

impl Particle {
    fn random_smoke(pos: Vec2, vel: Vec2) -> Particle {
        Particle {
            pos,
            vel,
            rotation: rand::gen_range(0., 360.),
            time_left: rand::gen_range(0.3, 0.4),
            size: 5.,
            thickness: 2.,
        }
    }

    fn star(pos: Vec2, line_thikness: f32) -> Particle {
        Particle {
            pos,
            vel: vec2(0., 0.),
            rotation: rand::gen_range(0., 360.),
            time_left: rand::gen_range(1., 3.0),
            thickness: line_thikness,
            size: line_thikness * 2.,
        }
    }

    fn draw(&self) {
        let r = self.size;
        let rx0 = self.pos.x() + self.rotation.to_radians().cos() * r;
        let ry0 = self.pos.y() + self.rotation.to_radians().sin() * r;

        let rx1 = self.pos.x() + (self.rotation + 180.).to_radians().cos() * r;
        let ry1 = self.pos.y() + (self.rotation + 180.).to_radians().sin() * r;

        draw_line(rx0, ry0, rx1, ry1, self.thickness, WHITE);
    }
}

fn draw_house(planet_center: Vec2, planet_radius: f32, house: &House, line_thikness: f32) {
    let House {
        house_angle,
        house_length,
        house_tall,
    } = house;
    let arc_len = 2.0 * std::f32::consts::PI * planet_radius;
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

fn draw_tree(planet_center: Vec2, planet_radius: f32, tree: &Tree, line_thikness: f32) {
    let Tree {
        tree_length,
        tree_levels,
        tree_angle,
        tree_tall,
    } = tree;
    let arc_len = 2. * std::f32::consts::PI * planet_radius;

    let a0 = tree_angle;
    let a1 = a0 + 0.001;

    for i in 0..*tree_levels {
        let p1 = vec2(
            planet_center.x() + a0.cos() * (planet_radius + i as f32 * tree_tall),
            planet_center.y() + a0.sin() * (planet_radius + i as f32 * tree_tall),
        );

        // Trees used jam-magic macroquad
        //draw_circle_lines(
        //    p1.x(),
        //    p1.y(),
        //    (*tree_levels as f32 - i as f32) * 10.,
        //    line_thikness,
        //    3,
        //    GREEN,
        //);
    }
}

fn magic_zoomed_camera(camera: Camera2D, magic_point: Vec2, zoom: f32) -> Camera2D {
    let mp_screen = camera.world_to_screen(magic_point);
    let dx = mp_screen.x() / 2.;
    let dy = (mp_screen.y() - camera.offset.y()) / 2.;

    Camera2D {
        zoom: vec2(zoom, zoom),
        target: vec2(
            magic_point.x() - dx * (2. / zoom),
            magic_point.y() - dy * (2. / zoom),
        ),
        offset: camera.offset,
        rotation: camera.rotation,
    }
}

fn line_thickness(camera: Camera2D) -> f32 {
    let p0 = camera.screen_to_world(vec2(0.0, 0.));
    let p1 = camera.screen_to_world(vec2(5.0 / screen_width(), 0.));
    (p0 - p1).abs().x()
}
#[macroquad::main("LD46!!!!")]
async fn main() {
    let texture = load_texture("assets/rocket.png").await;
    set_texture_filter(texture, FilterMode::Linear);

    let mut particles = Vec::new();

    let mut player = Rocket {
        vel: vec2(0., 0.),
        pos: vec2(0., 10.),
        rotation: 0.,
    };

    let mut world_camera = Camera2D {
        target: player.pos,
        offset: vec2(0., -0.5),
        zoom: vec2(0.003, 0.003),
        ..Default::default()
    };

    let planets = vec![
        Planet::new(10000., vec2(0., -10000.), true, false),
        Planet::new(5000., vec2(20000., 40000.), false, true),
    ];

    loop {
        let delta = get_frame_time();

        clear_background(BLACK);

	let magic_zoom = world_camera.zoom.x().max(0.0005);
        let magic_camera = magic_zoomed_camera(
            world_camera,
            vec2(player.pos.x(), player.pos.y() + texture.height() / 2.),
            magic_zoom,
        );
        begin_mode_2d(magic_camera);

        {
            let rand_x = rand::gen_range(-1., 1.);
            let rand_y = rand::gen_range(-1., 1.);

            let pos = magic_camera.screen_to_world(vec2(rand_x, rand_y));
            particles.push(Particle::star(pos, line_thickness(magic_camera) / 5.));
        }

        draw_texture_ex(
            texture,
            player.pos.x() - texture.width() / 2.,
            player.pos.y() + texture.height(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture.width(), -texture.height())),
                rotation: -player.rotation,
                ..Default::default()
            },
        );
        for particle in particles.iter_mut() {
            particle.draw();
        }

        end_mode_2d();

        begin_mode_2d(world_camera);

        if is_key_down(KeyCode::W) {
            let dir = vec2(player.rotation.sin(), player.rotation.cos());
            let m = vec2(player.pos.x(), player.pos.y() + texture.height() / 2.);
            let right = vec2(dir.y(), -dir.x());

            particles.push(Particle::random_smoke(
                m - dir * 150. + right * rand::gen_range(-40., 40.),
                dir * rand::gen_range(-100., -1000.) + right * rand::gen_range(-2., 2.),
            ));

            player.vel += dir.normalize() * delta * 3.;
        }

        if is_key_down(KeyCode::S) {
            let dir = vec2(player.rotation.sin(), player.rotation.cos());

            player.vel -= dir.normalize() * delta * 3.;
            let m = vec2(player.pos.x(), player.pos.y() + texture.height() / 2.);
            let right = vec2(dir.y(), -dir.x());

            particles.push(Particle::random_smoke(
                m + dir * 150. + right * rand::gen_range(-40., 40.),
                dir * rand::gen_range(-100., -1000.) + right * rand::gen_range(-2., 2.),
            ));
        }

        if is_key_down(KeyCode::A) {
            player.rotation -= delta * 2.;
        }
        if is_key_down(KeyCode::D) {
            player.rotation += delta * 2.;
        }

        if is_key_down(KeyCode::Up) {
            world_camera.zoom *= 1.01;
        }
        if is_key_down(KeyCode::Down) {
            world_camera.zoom *= 0.99;
        }

        let min_zoom = 0.002;
        let one_screen_time = 0.5;
        let zoom;
        if player.vel.length() < 0.001 {
            zoom = min_zoom;
        } else {
            let screen_distance = one_screen_time / player.vel.length();
            zoom = (screen_distance / screen_width()).min(min_zoom);
        }
	let zoom = vec2(zoom, zoom);
        world_camera.zoom = world_camera.zoom * 0.99 + zoom * 0.01;

        for particle in particles.iter_mut() {
            particle.rotation += delta * 100.;
            particle.pos += particle.vel * delta;
            if particle.pos.y() <= 0. {
                *particle.vel.y_mut() *= -1.;
                particle.time_left = 0.05f32.min(particle.time_left);
            }
            particle.time_left -= delta;
        }
        particles.retain(|particle| particle.time_left > 0.);

        for planet in &planets {
            planet.draw(line_thickness(world_camera));
        }

        end_mode_2d();

        let world_player_pos = vec2(
            player.pos.x() - texture.width() / 2.,
            player.pos.y() + texture.height() / 2.,
        );
        let screen_pos = world_camera.world_to_screen(world_player_pos);

        if screen_pos.y() >= 0.4 {
            let world_pos = world_camera.screen_to_world(screen_pos);

            world_camera.target +=
                world_pos - world_camera.screen_to_world(vec2(screen_pos.x(), 0.4));
        }
        if screen_pos.y() <= -0.4 {
            let world_pos = world_camera.screen_to_world(screen_pos);

            world_camera.target +=
                world_pos - world_camera.screen_to_world(vec2(screen_pos.x(), -0.4));
        }
        if screen_pos.x() <= -0.4 {
            let world_pos = world_camera.screen_to_world(screen_pos);

            world_camera.target +=
                world_pos - world_camera.screen_to_world(vec2(-0.4, screen_pos.y()));
        }
        if screen_pos.x() >= 0.4 {
            let world_pos = world_camera.screen_to_world(screen_pos);

            world_camera.target +=
                world_pos - world_camera.screen_to_world(vec2(0.4, screen_pos.y()));
        }

        player.pos += player.vel;

        next_frame().await
    }
}
