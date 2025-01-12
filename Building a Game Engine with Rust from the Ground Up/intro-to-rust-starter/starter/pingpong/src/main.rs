use std::ffi::CString;
use std::thread;
use std::time;

// Constants
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const THICKNESS: i32 = 20;
const PADDLE_HEIGHT: i32 = 100;
const BALL_SPEED: f32 = 2.0;

fn main() {
    // Initial positions
    let left_paddle_x: f32 = (THICKNESS * 2) as f32;
    let mut left_paddle_y: f32 = (SCREEN_HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32;

    let right_paddle_x: f32 = (SCREEN_WIDTH - THICKNESS * 3) as f32;
    let mut right_paddle_y: f32 = (SCREEN_HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32;

    let mut ball_x: f32 = (3 * THICKNESS) as f32;
    let mut ball_y: f32 = left_paddle_y + (PADDLE_HEIGHT / 2 - THICKNESS / 2) as f32;

    let mut ball_dx: f32 = BALL_SPEED;
    let mut ball_dy: f32 = BALL_SPEED;

    let mut paddle_up = false;
    let mut paddle_down = false;

    let title = CString::new("Project Ping Pong - Press UP and DOWN to play").unwrap();

    // Create game objects
    let mut upper_wall = opengl_game::opengl_ffi::rust_create_sprite(
        0.0, 0.0, SCREEN_WIDTH, THICKNESS, 255, 255, 255,
    );
    let mut lower_wall = opengl_game::opengl_ffi::rust_create_sprite(
        0.0, (SCREEN_HEIGHT - THICKNESS) as f32, SCREEN_WIDTH, THICKNESS, 255, 255, 255,
    );
    let mut left_paddle = opengl_game::opengl_ffi::rust_create_sprite(
        left_paddle_x, left_paddle_y, THICKNESS, PADDLE_HEIGHT, 255, 0, 0,
    );
    let mut right_paddle = opengl_game::opengl_ffi::rust_create_sprite(
        right_paddle_x, right_paddle_y, THICKNESS, PADDLE_HEIGHT, 255, 255, 255,
    );
    let mut ball = opengl_game::opengl_ffi::rust_create_sprite(
        ball_x, ball_y, THICKNESS, THICKNESS, 255, 255, 255,
    );

    // Start game loop
    opengl_game::start_window_and_game_loop!(title.as_ptr(), SCREEN_WIDTH, SCREEN_HEIGHT, {
        // Clear the screen
        opengl_game::opengl_ffi::rust_clear_screen();

        // Render objects
        opengl_game::opengl_ffi::rust_render_sprite(&mut ball);
        opengl_game::opengl_ffi::rust_render_sprite(&mut upper_wall);
        opengl_game::opengl_ffi::rust_render_sprite(&mut lower_wall);
        opengl_game::opengl_ffi::rust_render_sprite(&mut left_paddle);
        opengl_game::opengl_ffi::rust_render_sprite(&mut right_paddle);

        // Ball movement
        ball_x += ball_dx;
        ball_y += ball_dy;

        // Ball collision with walls
        if ball_y > (SCREEN_HEIGHT - 2 * THICKNESS) as f32 {
            ball_dy = -BALL_SPEED;
        }
        if ball_y < THICKNESS as f32 {
            ball_dy = BALL_SPEED;
        }

        // Ball collision with paddles
        if ball_x < (2 * THICKNESS) as f32 && ball_x > (THICKNESS) as f32 {
            if ball_y + THICKNESS as f32 > left_paddle_y && ball_y < left_paddle_y + PADDLE_HEIGHT as f32 {
                ball_dx = BALL_SPEED;
            }
        }

        if ball_x > (SCREEN_WIDTH - 4 * THICKNESS) as f32 {
            ball_dx = -BALL_SPEED;
        }

        // Reset ball if out of bounds
        if ball_x < -10.0 * THICKNESS as f32 {
            ball_x = (3 * THICKNESS) as f32;
            ball_y = left_paddle_y + (PADDLE_HEIGHT / 2 - THICKNESS / 2) as f32;
            ball_dx = BALL_SPEED;
        }

        // Move right paddle (AI logic)
        right_paddle_y = ball_y - (PADDLE_HEIGHT / 2 - THICKNESS / 2) as f32;
        right_paddle_y = right_paddle_y.clamp(THICKNESS as f32, (SCREEN_HEIGHT - THICKNESS - PADDLE_HEIGHT) as f32);

        // Move left paddle (player input)
        if paddle_up {
            left_paddle_y -= BALL_SPEED;
        }
        if paddle_down {
            left_paddle_y += BALL_SPEED;
        }
        left_paddle_y = left_paddle_y.clamp(THICKNESS as f32, (SCREEN_HEIGHT - THICKNESS - PADDLE_HEIGHT) as f32);

        // Update object positions
        opengl_game::opengl_ffi::rust_update_sprite_position(&mut ball, ball_x, ball_y);
        opengl_game::opengl_ffi::rust_update_sprite_position(&mut right_paddle, right_paddle_x, right_paddle_y);
        opengl_game::opengl_ffi::rust_update_sprite_position(&mut left_paddle, left_paddle_x, left_paddle_y);

        // Timing and input
        opengl_game::tick!(10);
        opengl_game::conditional_break!();
        opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, { break; });
        opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_DOWN, { paddle_down = true; });
        opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_UP, { paddle_up = true; });
        opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_DOWN, { paddle_down = false; });
        opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_UP, { paddle_up = false; });
    });
}
