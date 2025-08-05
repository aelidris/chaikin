use macroquad::prelude::*;

#[macroquad::main("Chaikin Curve",
    window_width = 800,
    window_height = 600,
    window_resizable = true,
)]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();
    loop {
        clear_background(BLACK);
        draw_text("Click to add points, press Delete to clear, Escape to exit", 10.0, 20.0, 20.0, WHITE);
        for point in &points {
            draw_circle(point.x, point.y, 2.0, WHITE);
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Delete){
            points.clear();
            clear_background(BLACK);
        }
        if is_mouse_button_pressed(MouseButton::Left)  {
            let (x, y) = mouse_position();
                points.push(vec2(x, y));
                println!("Point added at: ({})", points.len());
        }
        next_frame().await;
    }
}
