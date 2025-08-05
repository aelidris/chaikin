use macroquad::prelude::*;

#[macroquad::main("Chaikin Curve",
    window_width = 800,
    window_height = 600,
    window_resizable = true,
)]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();
    let mut is_animating = false;
    let mut animation_step = 0;
    let mut animation_timer = 0.0;
    let animation_delay = 0.5;
    let max_steps = 7;
    let mut current_curve = Vec::new();
    
    loop {
        clear_background(BLACK);
        if is_animating {
            animation_timer += get_frame_time(); 
            if animation_timer >= animation_delay {
                animation_timer = 0.0;
                animation_step += 1;  
                if animation_step > max_steps {
                    animation_step = 0;
                    current_curve = points.clone();
                } else if animation_step == 0 {
                    current_curve = points.clone();
                } else {
                    current_curve = chaikin_subdivision(&current_curve);
                }
            }
        }
        
        draw_text("Click to add points, Enter to animate, Delete to clear, Escape to exit", 10.0, 20.0, 20.0, WHITE);
        if is_animating {
            draw_text(&format!("Animation step: {}/{}", animation_step, max_steps), 10.0, 45.0, 20.0, YELLOW);
        }
        if is_animating && !current_curve.is_empty() {
            draw_curve(&current_curve, if animation_step == 0 { WHITE } else { GREEN });
            for point in &points {
                draw_circle(point.x, point.y, 2.0, GRAY);
            }
        } else {
            for point in &points {
                draw_circle(point.x, point.y, 2.0, WHITE);
            }
            if points.len() >= 2 {
                for i in 0..points.len() - 1 {
                    draw_line(points[i].x, points[i].y, points[i+1].x, points[i+1].y, 1.0, WHITE);
                }
            }
        }
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Delete) {
            points.clear();
            is_animating = false;
            animation_step = 0;
            animation_timer = 0.0;
            current_curve.clear();
        } else if is_key_pressed(KeyCode::Enter) && points.len() >= 2 {
            is_animating = true;
            animation_step = 0;
            animation_timer = 0.0;
            current_curve = points.clone();
        } 
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            points.push(vec2(x, y));
            println!("Point added at: ({})", points.len());
            // Stop animation when adding new points
            if is_animating {
                is_animating = false;
                animation_step = 0;
                animation_timer = 0.0;
            }
        }
        
        next_frame().await;
    }
}

fn draw_curve(points: &[Vec2], color: Color) {
    for point in points {
        draw_circle(point.x, point.y, 2.0, color);
    }
    if points.len() >= 2 {
        for i in 0..points.len() - 1 {
            draw_line(points[i].x, points[i].y, points[i+1].x, points[i+1].y, 2.0, color);
        }
    }
}

fn chaikin_subdivision(points: &[Vec2]) -> Vec<Vec2> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i+1];
        
        new_points.push(vec2(
            0.75 * p1.x + 0.25 * p2.x,
            0.75 * p1.y + 0.25 * p2.y,
        ));
        new_points.push(vec2(
            0.25 * p1.x + 0.75 * p2.x,
            0.25 * p1.y + 0.75 * p2.y,
        ));
    }
    new_points
}