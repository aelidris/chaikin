use macroquad::prelude::*;

#[macroquad::main(
    "Chaikin Curve",
    window_width = 800,
    window_height = 600,
    window_resizable = true
)]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();
    let mut is_animating = false;
    let mut animation_step = 1;
    let mut animation_timer = 0.0;
    let animation_delay = 0.5;
    let max_steps = 7;
    let mut current_curve = Vec::new();

    let mut show_no_points_message = false;
    let mut message_timer = 0.0;
    let message_duration = 2.0;

    // Drag state
    let mut dragging_index: Option<usize> = None;
    let mut drag_offset = vec2(0.0, 0.0);
    let point_radius = 5.0;

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time();
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        if show_no_points_message {
            message_timer -= delta_time;
            if message_timer <= 0.0 {
                show_no_points_message = false;
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mut clicked_point = false;
            for (i, point) in points.iter().enumerate() {
                let distance = (mouse_pos - *point).length();
                if distance <= point_radius {
                    dragging_index = Some(i);
                    drag_offset = *point - mouse_pos;
                    clicked_point = true;
                    break;
                }
            }   
            if !clicked_point {
                points.push(mouse_pos);
                if is_animating {
                    current_curve.push(mouse_pos);
                }
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(index) = dragging_index {
                points[index] = mouse_pos + drag_offset;
                if points.len() >= 2 {
                    if is_animating {
                        current_curve = points.clone();
                        for _ in 1..animation_step {
                            current_curve = chaikin_subdivision(&current_curve);
                        }
                    } else {
                        current_curve = points.clone();
                    }
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging_index = None;
        }

        if is_animating {
            animation_timer += delta_time;
            if animation_timer >= animation_delay {
                animation_timer = 0.0;
                animation_step += 1;
                if animation_step > max_steps {
                    animation_step = 1;
                    current_curve = points.clone();
                } else if animation_step == 1 {
                    current_curve = points.clone();
                } else {
                    current_curve = chaikin_subdivision(&current_curve);
                }
            }
        }

        draw_text(
            "Click to add points, drag to move, Enter to animate, Delete to clear, Escape to exit",
            10.0,
            20.0,
            20.0,
            WHITE
        );
        if is_animating {
            draw_text(
                &format!("Animation step: {}/{}", animation_step, max_steps),
                10.0,
                45.0,
                20.0,
                YELLOW
            );
        }
        if show_no_points_message {
            draw_text("You forgot to draw any points?", 30.0, 70.0, 30.0, RED);
        }

        if !current_curve.is_empty() {
            draw_curve(&current_curve, if animation_step == 1 { WHITE } else { GREEN });
        }

        for point in points.iter() {          
            draw_circle_lines(point.x, point.y, point_radius, 2.0, DARKGRAY);
                  
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Delete) {
            points.clear();
            is_animating = false;
            animation_step = 1;
            animation_timer = 0.0;
            current_curve.clear();
            dragging_index = None;
        } else if is_key_pressed(KeyCode::Enter) {
            if points.len() >= 2 {
                if points.len() > 2 {
                    is_animating = true;
                }
                animation_step = 1;
                animation_timer = 0.0;
                current_curve = points.clone();
            } else if points.len() == 0 {
                show_no_points_message = true;
                message_timer = message_duration;
            }
        }

        next_frame().await;
    }
}

fn draw_curve(points: &[Vec2], color: Color) {
    if points.len() >= 2 {
        for i in 0..points.len() - 1 {
            draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, 2.0, color);
        }
    }
}

fn chaikin_subdivision(points: &[Vec2]) -> Vec<Vec2> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();

    new_points.push(points[0]);

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        new_points.push(vec2(0.75 * p1.x + 0.25 * p2.x, 0.75 * p1.y + 0.25 * p2.y));

        new_points.push(vec2(0.25 * p1.x + 0.75 * p2.x, 0.25 * p1.y + 0.75 * p2.y));
    }
    new_points.push(points[points.len() - 1]);

    new_points
}
