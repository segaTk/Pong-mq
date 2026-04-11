use macroquad::prelude::*;
use macroquad::camera::Camera2D;

// === Константы ===
const WINDOW_W: f32 = 640.;
const WINDOW_H: f32 = 480.;
const PADDLE_W: f32 = 15.;
const PADDLE_H: f32 = 80.;
const BALL_S: f32 = 15.;
const WINNING_SCORE: i32 = 5;

// === Счёт ===
struct Score {
    left: i32,
    right: i32,
}

impl Score {
    fn new() -> Self { Self { left: 0, right: 0 } }
    fn left_scored(&mut self) { self.left += 1; }
    fn right_scored(&mut self) { self.right += 1; }
    fn winner(&self) -> Option<&'static str> {
        if self.left >= WINNING_SCORE { Some("LEFT PLAYER WINS!") }
        else if self.right >= WINNING_SCORE { Some("RIGHT PLAYER WINS!") }
        else { None }
    }
}

// === Эффекты ===
struct Effects {
    goal_flash: f32,
    flash_color: Color,
}

impl Effects {
    fn new() -> Self { Self { goal_flash: 0., flash_color: WHITE } }
    fn trigger_goal(&mut self, color: Color) {
        self.goal_flash = 0.5;
        self.flash_color = color;
    }
    fn update(&mut self, dt: f32) {
        if self.goal_flash > 0. { self.goal_flash -= dt; }
    }
    fn is_flashing(&self) -> bool { self.goal_flash > 0. }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut left_y = (WINDOW_H - PADDLE_H) / 2.;
    let mut right_y = (WINDOW_H - PADDLE_H) / 2.;
    let mut ball_x = WINDOW_W / 2.;
    let mut ball_y = WINDOW_H / 2.;
    let mut ball_dx = 4.;
    let mut ball_dy = 4.;
    
    let mut score = Score::new();
    let mut effects = Effects::new();
    let mut is_fullscreen = false;

    
    // Счётчик кадров для "псевдо-рандома"
    let mut frame_counter: u64 = 0;
    
    loop {
        // Переключение полноэкранного режима
        if is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        // Обновление таймингов
        frame_counter = frame_counter.wrapping_add(1);
        let dt = get_frame_time();
        
        // === Управление ===
        if score.winner().is_none() {
            if is_key_down(KeyCode::W) { left_y += 7.; }
            if is_key_down(KeyCode::S) { left_y -= 7.; }
            if is_key_down(KeyCode::Up) { right_y += 7.; }
            if is_key_down(KeyCode::Down) { right_y -= 7.; }
            
            left_y = left_y.clamp(0., WINDOW_H - PADDLE_H);
            right_y = right_y.clamp(0., WINDOW_H - PADDLE_H);
        }
        
        // === Логика мяча ===
        if score.winner().is_none() {
            ball_x += ball_dx;
            ball_y += ball_dy;
            
            if ball_y <= 0. || ball_y >= WINDOW_H - BALL_S {
                ball_dy = -ball_dy;
            }
            
            if ball_dx < 0. && ball_x <= 35. && 
               ball_y + BALL_S > left_y && ball_y < left_y + PADDLE_H {
                ball_dx = -ball_dx * 1.05;
                ball_dy += (ball_y - (left_y + PADDLE_H/2.)) * 0.1;
            }
            
            if ball_dx > 0. && ball_x >= WINDOW_W - 50. && 
               ball_y + BALL_S > right_y && ball_y < right_y + PADDLE_H {
                ball_dx = -ball_dx * 1.05;
                ball_dy += (ball_y - (right_y + PADDLE_H/2.)) * 0.1;
            }
            
            if ball_x < 0. {
                score.right_scored();
                effects.trigger_goal(GREEN);
                reset_ball(&mut ball_x, &mut ball_y, &mut ball_dx, &mut ball_dy, -1., frame_counter);
            }
            else if ball_x > WINDOW_W {
                score.left_scored();
                effects.trigger_goal(RED);
                reset_ball(&mut ball_x, &mut ball_y, &mut ball_dx, &mut ball_dy, 1., frame_counter);
            }
        }
        
        effects.update(dt);

        set_camera(&Camera2D::from_display_rect(Rect::new(0., 0., WINDOW_W, WINDOW_H)));

        // Отрисовка ИГРЫ (в координатах 0..640 × 0..480)
        let bg_color = if effects.is_flashing() {
            let t = (effects.goal_flash / 0.5).min(1.);
            Color::new(effects.flash_color.r * t, effects.flash_color.g * t, effects.flash_color.b * t, 0.3 * t)
        } else { BLACK };
        clear_background(bg_color);

        // Мяч (свечение + основной)
        draw_circle(ball_x + BALL_S/2., ball_y + BALL_S/2., BALL_S/2. + 3., Color::new(1., 1., 1., 0.3));
        draw_circle(ball_x + BALL_S/2., ball_y + BALL_S/2., BALL_S/2., WHITE);

        // Ракетки в игровых координатах
        draw_rectangle(20., left_y, PADDLE_W, PADDLE_H, Color::new(0.8, 0.2, 0.2, 1.));  // левая
        draw_rectangle(WINDOW_W - 35., right_y, PADDLE_W, PADDLE_H, Color::new(0.2, 0.2, 1., 1.));  // правая

        // Сетка
        for y in (0..(WINDOW_H as i32)).step_by(25) {
            draw_rectangle(WINDOW_W/2. - 2., y as f32, 4., 15., Color::new(0.5, 0.5, 0.5, 0.5));
        }

        // Камера для UI
        set_default_camera();
        
        // Счёт
        let score_text = format!("{} : {}", score.left, score.right);
        draw_text_ex(
            &score_text,
            screen_width() / 2. - 50.0,
            40.,
            TextParams {font_size: 40, color: WHITE, ..Default::default()}
        );
        
        draw_text_ex("LEFT", 60.0, 20.0, TextParams { font_size: 18, color: RED, ..Default::default() });
        draw_text_ex("RIGHT", screen_width() - 120.0, 20.0, TextParams { font_size: 18, color: BLUE, ..Default::default() });
        
        // Победа
        if let Some(winner_msg) = score.winner() {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));
            draw_text_ex(
                winner_msg, screen_width() / 2.0 - 120.0,
                screen_height() / 2.0 - 20.0, TextParams {
                font_size: 32, color: GOLD, ..Default::default()
            });
            draw_text_ex(
                "Press SPACE to restart", screen_width() / 2.0 - 80.0, 
                screen_height() / 2.0 + 15.0, 
                TextParams { font_size: 18, color: WHITE, ..Default::default() });
            if is_key_pressed(KeyCode::Space) {
                score = Score::new();
                reset_ball(&mut ball_x, &mut ball_y, &mut ball_dx, &mut ball_dy, 1., frame_counter);
            }
        }
        
        // Подсказки + индикатор режима
        if score.winner().is_none() {
            draw_text_ex("W/S — Left | ↑/↓ — Right | Esc — Quit | F11 — Fullscreen", 
                        10.0,
                        screen_height() - 25.0, 
                        TextParams { font_size: 14, color: GRAY, ..Default::default() });
            
            // Индикатор режима
            let mode_text = if is_fullscreen { "FULLSCREEN" } else { "WINDOWED" };
            draw_text_ex(mode_text, 
                screen_width() - 180.0, 
                screen_height() - 20.0, 
                TextParams { font_size: 14, color: Color::new(0.7, 0.7, 1.0, 0.8), ..Default::default() });
        }
        
        if is_key_pressed(KeyCode::Escape) { break; }
        next_frame().await;
    }
}

// === Вспомогательные функции ===
fn reset_ball(x: &mut f32, y: &mut f32, dx: &mut f32, dy: &mut f32, direction: f32, frame: u64) {
    *x = WINDOW_W / 2.0 - BALL_S / 2.0;
    *y = WINDOW_H / 2.0 - BALL_S / 2.0;
    *dx = 4.0 * direction;
    // "Псевдо-рандом" на основе номера кадра
    *dy = if (frame % 2) == 0 { 4.0 } else { -4.0 };
}