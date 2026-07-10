// src/neirachain.rs

use crate::{WINDOW_H, BALL_S, PADDLE_H};

/// Предсказывает Y-позицию, куда прилетит мяч на правую сторону,
/// с учётом отскоков от верхней/нижней границы.
pub fn predict_ball_y(
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    right_paddle_x: f32,
) -> f32 {
    if ball_dx <= 0.0 {
        // Мяч летит влево — не нужно предсказывать
        return ball_y;
    }

    let mut future_x = ball_x;
    let mut future_y = ball_y;
    let mut future_dy = ball_dy;

    // Симулируем полёт до правой ракетки
    while future_x < right_paddle_x {
        // Шаг по X до следующего события (стенка или ракетка)
        let dx_to_paddle = right_paddle_x - future_x;
        let time_to_paddle = dx_to_paddle / ball_dx;

        let new_y = future_y + future_dy * time_to_paddle;

        // Проверяем, ударится ли мяч о верх/низ до достижения ракетки
        if new_y < 0.0 || new_y > WINDOW_H - BALL_S {
            // Находим точку отскока
            let time_to_top_or_bottom = if future_dy > 0.0 {
                (WINDOW_H - BALL_S - future_y) / future_dy
            } else {
                -future_y / future_dy
            };

            if time_to_top_or_bottom < time_to_paddle {
                // Отскок от стены
                future_x += ball_dx * time_to_top_or_bottom;
                future_y = if future_dy > 0.0 { WINDOW_H - BALL_S } else { 0.0 };
                future_dy = -future_dy;
            } else {
                // Долетает до ракетки без отскока
                future_x = right_paddle_x;
                future_y = new_y.clamp(0.0, WINDOW_H - BALL_S);
                break;
            }
        } else {
            // Долетает до ракетки без отскока
            future_x = right_paddle_x;
            future_y = new_y;
            break;
        }
    }

    future_y
}

/// Обновляет позицию правой ракетки, чтобы она двигалась к предсказанной точке.
pub fn update_ai_paddle(
    paddle_y: &mut f32,
    target_y: f32,
    speed: f32,
) {
    let center_offset = PADDLE_H / 2.0;
    let ideal_center = target_y + BALL_S / 2.0 - center_offset;

    if *paddle_y < ideal_center {
        *paddle_y += speed;
    } else if *paddle_y > ideal_center {
        *paddle_y -= speed;
    }

    *paddle_y = paddle_y.clamp(0.0, WINDOW_H - PADDLE_H);
}