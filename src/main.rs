// todo:
    // stabiler integrator (velocity-verlet/leapfrog)
    // kein screen wrapping mehr (muss eig nicht aber für mehr realismus und so)
    // Center-of-mass correction — subtract COM motion so the whole system doesn’t drift: ka was das heißt meinte chatty
    // Virialization step (googlen oder so)
    // globale rotation damit disks oder spheres entstehen
    
    // irgendwas Milchstraßen ähnliches

    // anscheinend kann man das nicht gut simulieren mit orbits in systemen ohne zentralen körper

use macroquad::prelude::*;
use macroquad::ui::*;
extern crate rand as external_rand;
use external_rand::{Rng, thread_rng};

#[derive(Clone)]
struct Planet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    size: f32,
    mass: f32,
    colour: Color,
}

const GRAVITY: f32 = 3.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Moving Planets".to_string(),
        window_width: 1200,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    Color::new(
        rng.gen::<f32>(), // Red
        rng.gen::<f32>(), // Green
        rng.gen::<f32>(), // Blue
        1.0               // Alpha
    )
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut planets: Vec<Planet> = vec![];
    let mut slider_value: f32 = 20.0;

    loop {
        clear_background(BLACK);
        // planet pro click
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            let mut rng = thread_rng();

            planets.push(Planet {
                x,
                y,
                vx: rng.gen_range(-2.0..2.0), // random starting speed
                vy: rng.gen_range(-2.0..2.0), 
                size: slider_value,            // größe vom slider
                mass: slider_value * 5.0,      // größe vom slider * 5 = masse
                colour: random_color(),        // obige Funktion für random farbe wird aufgerufen
            });
        }

        // Movement effect accumulator, also einfach für jeden planeten diesen vektor: (0.0, 0.0) -> (vx, vy)
        let mut accelerations = vec![(0.0, 0.0); planets.len()];

        // force ausrechnen
        for i in 0..planets.len() {
            for j in 0..planets.len() {
                if i == j {
                    continue;
                }

                let eps = 0.1; //macht die gravity ein bisschen kleiner wenn die planeten zu nahe kommen

                let dx = planets[j].x - planets[i].x;
                let dy = planets[j].y - planets[i].y;
                let dist_sq = dx * dx + dy * dy + eps * eps;
                let dist = dist_sq.sqrt().max(1.0); // prevent division by 0


                
                // gravitational force magnitude
                let force = GRAVITY * planets[i].mass * planets[j].mass / dist_sq;

                // normalized direction
                let fx = force * dx / dist;
                let fy = force * dy / dist;

                // accumulate acceleration (force / mass)
                accelerations[i].0 += fx / planets[i].mass;
                accelerations[i].1 += fy / planets[i].mass;
            }
        }

        let dt = 0.5; // schnelligkeit der simulation

        // alles addieren
        for (i, planet) in planets.iter_mut().enumerate() {
            planet.vx += accelerations[i].0 * dt; // update positions mit dem speed vector
            planet.vy += accelerations[i].1* dt;
            planet.x += planet.vx * dt;
            planet.y += planet.vy * dt;
        }


        // draw
        for planet in &mut planets {
            
            // screen größe dynamisch checken
            let screen_w = screen_width();
            let screen_h = screen_height();

            // gucken ob ball über screen hinausgeht bei x
            if planet.x > screen_w {
                planet.x = 0.0;
            } 
            else if planet.x < 0.0 {
                planet.x = screen_w;
            }

            // gucken ob ball über screen hinaUSGEHT bei y
            if planet.y > screen_h {
                planet.y = 0.0;
            } 
            else if planet.y < 0.0 {
                planet.y = screen_h;
            }
            
            draw_circle(planet.x, planet.y, planet.size as f32, planet.colour);
        }

        draw_text(
            &format!("{}", planets.len()),
            20.0,
            50.0,
            40.0,
            WHITE,
        );

        // ui elemte zeichnen

        root_ui().slider(hash!("my_slider"), "Planet Size", 5.0..100.0, &mut slider_value);
            
        root_ui().label(None, &format!("Value: {:.2}", slider_value));
        
        next_frame().await;
    }
}
