extern crate nalgebra_glm as glm;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::thread;

mod framebuffer;

use crate::framebuffer::Framebuffer;

// Colores del juego
const ALIVE_COLOR: u32 = 0x66BAFF;
const DEAD_COLOR: u32 = 0xFFD1FB;

fn place_pattern(fb: &mut Framebuffer, start_x: usize, start_y: usize, pattern: &[&str]) {
    fb.set_current_color(ALIVE_COLOR);
    for (row, line) in pattern.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                let px = (start_x + col) % fb.width;
                let py = (start_y + row) % fb.height;
                fb.point(px, py);
            }
        }
    }
}

fn count_neighbors(fb: &Framebuffer, cx: usize, cy: usize) -> usize {
    let mut count = 0;
    let w = fb.width as isize;
    let h = fb.height as isize;
    let cx = cx as isize;
    let cy = cy as isize;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (cx + dx + w) % w;
            let ny = (cy + dy + h) % h;
            
            if fb.get_color(nx as usize, ny as usize) == ALIVE_COLOR {
                count += 1;
            }
        }
    }
    count
}

fn render(fb: &mut Framebuffer) {
    let current_buffer = fb.buffer.clone();
    
    // Objeto temporal de Framebuffer para no mutar mientras leemos
    let mut temp_fb = Framebuffer::new(fb.width, fb.height);
    temp_fb.buffer = current_buffer;

    for y in 0..fb.height {
        for x in 0..fb.width {
            let is_alive = temp_fb.get_color(x, y) == ALIVE_COLOR;
            let neighbors = count_neighbors(&temp_fb, x, y);

            if is_alive {
                if neighbors < 2 || neighbors > 3 {
                    fb.set_current_color(DEAD_COLOR);
                    fb.point(x, y); // Muere por underpopulation u overpopulation
                } else {
                    fb.set_current_color(ALIVE_COLOR);
                    fb.point(x, y); // Sobrevive
                }
            } else {
                if neighbors == 3 {
                    fb.set_current_color(ALIVE_COLOR);
                    fb.point(x, y); // Nace por reproduction
                } else {
                    fb.set_current_color(DEAD_COLOR);
                    fb.point(x, y); // Sigue muerta
                }
            }
        }
    }
}

fn init_scene(fb: &mut Framebuffer) {
    // 1. Block (Still life)
    place_pattern(fb, 2, 2, &[
        "**",
        "**"
    ]);

    // 2. Blinker (Oscillator)
    place_pattern(fb, 10, 2, &[
        "***"
    ]);

    // 3. Toad (Oscillator)
    place_pattern(fb, 20, 2, &[
        " ***",
        "*** "
    ]);

    // 4. Beacon (Oscillator)
    place_pattern(fb, 30, 2, &[
        "**  ",
        "**  ",
        "  **",
        "  **"
    ]);

    // 5. Pulsar (Oscillator)
    place_pattern(fb, 45, 5, &[
        "  ***   ***  ",
        "             ",
        "*    * *    *",
        "*    * *    *",
        "*    * *    *",
        "  ***   ***  ",
        "             ",
        "  ***   ***  ",
        "*    * *    *",
        "*    * *    *",
        "*    * *    *",
        "             ",
        "  ***   ***  "
    ]);

    // 6. Glider (Spaceship)
    place_pattern(fb, 5, 15, &[
        " * ",
        "  *",
        "***"
    ]);

    // 7. LWSS (Spaceship)
    place_pattern(fb, 5, 25, &[
        " *  *",
        "*    ",
        "*   *",
        "**** "
    ]);

    // 8. Penta-decathlon (Oscillator)
    place_pattern(fb, 25, 20, &[
        "  *    *  ",
        "** **** **",
        "  *    *  "
    ]);

    // 9. Gosper Glider Gun
    place_pattern(fb, 5, 45, &[
        "........................*",
        "......................*.*",
        "............**......**............**",
        "...........*...*....**............**",
        "**........*.....*...**",
        "**........*...*.**....*.*",
        "..........*.....*.......*",
        "...........*...*",
        "............**"
    ]);

    // 10. True Puffer Train (Gosper's Puffer 1)
    place_pattern(fb, 40, 70, &[
        "  *    *      * * ",
        "   *    *  *     *",
        "   *    ***      *",
        "*  *          *  *",
        " ***            ***"
    ]);

    // --- NUEVOS ORGANISMOS DE LA IMAGEN ---

    // 14. Bee-hive (Still life)
    place_pattern(fb, 2, 8, &[
        " ** ",
        "*  *",
        " ** "
    ]);

    // 15. Loaf (Still life)
    place_pattern(fb, 10, 8, &[
        " ** ",
        "*  *",
        " * *",
        "  * "
    ]);

    // 16. Boat (Still life)
    place_pattern(fb, 18, 8, &[
        "** ",
        "* *",
        " * "
    ]);

    // 17. Tub (Still life)
    place_pattern(fb, 26, 8, &[
        " * ",
        "* *",
        " * "
    ]);

    // 18. MWSS - Middle-weight spaceship (Spaceship)
    place_pattern(fb, 5, 35, &[
        "  *** ",
        "*    *",
        "     *",
        "*   * ",
        " *    "
    ]);

    // 19. HWSS - Heavy-weight spaceship (Spaceship)
    place_pattern(fb, 20, 35, &[
        "  **** ",
        "*     *",
        "      *",
        "*    * ",
        " *     "
    ]);

    // 11. Fleet de Gliders
    for i in 0..5 {
        place_pattern(fb, 20 + i * 5, 60, &[
            " * ",
            "  *",
            "***"
        ]);
    }

    // 12. Más osciladores dispersos
    place_pattern(fb, 80, 10, &[
        "  ***   ***  ",
        "             ",
        "*    * *    *",
        "*    * *    *",
        "*    * *    *",
        "  ***   ***  ",
        "             ",
        "  ***   ***  ",
        "*    * *    *",
        "*    * *    *",
        "*    * *    *",
        "             ",
        "  ***   ***  "
    ]);

    // 13. Sopa primordial (Ruido aleatorio) en el cuadrante inferior
    // Usamos un generador pseudoaleatorio simple (LCG)
    let mut seed: u32 = 42;
    fb.set_current_color(ALIVE_COLOR);
    for y in 80..115 {
        for x in 10..150 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            if seed % 3 == 0 { // 33% de probabilidad de estar viva
                fb.point(x, y);
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 160;
    let framebuffer_height = 120;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Fondo inicial (muerto)
    framebuffer.set_background_color(DEAD_COLOR);
    framebuffer.clear();

    // Inicializar organismos
    init_scene(&mut framebuffer);

    // Loop del juego
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        // Dibujar a la ventana
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        // Calcular siguiente generacion
        render(&mut framebuffer);

        // Controlar la velocidad del frame
        thread::sleep(Duration::from_millis(100));
    }
}
