use crate::game::ConsoleGame;
use std::{thread, time::Duration};
use crossterm::event::{poll, KeyCode, Event, read};

pub fn run_pong() {
    const WIN_HEIGHT: isize = 36;
    const WIN_WIDTH: isize = 64;

    let mut graphics = ConsoleGame::new(WIN_WIDTH as usize, WIN_HEIGHT as usize);
    graphics.init();

    const PADDLE_SIZE: isize = 5;

    let mut y_left =  (WIN_HEIGHT / 2) - PADDLE_SIZE;
    let mut y_right = (WIN_HEIGHT / 2) - PADDLE_SIZE;
    let mut ball_pos = (WIN_WIDTH / 2, WIN_HEIGHT / 2);
    let mut ball_vel = (1isize, 1isize);
    let mut score = (0, 0);

    loop {
        while poll(Duration::ZERO).unwrap() {
            if let Event::Key(key) = read().unwrap() {
                match key.code {
                    KeyCode::Char('w') => y_left -= 1,
                    KeyCode::Char('s') => y_left += 1,
                    KeyCode::Up =>        y_right -= 1,
                    KeyCode::Down =>      y_right += 1,
                    _ => {},
                };
            }
        }

        let old_ball_pos = ball_pos;
        let mut collide = false;
        ball_pos = (ball_pos.0 + ball_vel.0, ball_pos.1 + ball_vel.1);

        if ball_pos.1 >= WIN_HEIGHT || ball_pos.1 < 0 {
            ball_vel = (ball_vel.0, -ball_vel.1);
            collide = true;
        }

        if ball_pos.0 == 5 && ball_pos.1 >= y_left && ball_pos.1 <= y_left + PADDLE_SIZE {
            ball_vel = (-ball_vel.0, ball_vel.1);
            collide = true;
        }
        if ball_pos.0 == WIN_WIDTH - 5 && ball_pos.1 >= y_right && ball_pos.1 <= y_right + PADDLE_SIZE {
            ball_vel = (-ball_vel.0, ball_vel.1);
            collide = true;
        }

        if collide {
            ball_pos = (old_ball_pos.0 + ball_vel.0, old_ball_pos.1 + ball_vel.1);
        }

        if ball_pos.0 >= WIN_WIDTH || ball_pos.0 < 0 {
            if ball_pos.0 >= WIN_WIDTH {
                score.0 += 1;
            } else {
                score.1 += 1;
            }
            ball_pos = (WIN_WIDTH / 2, WIN_HEIGHT / 2);
            ball_vel = (1isize, 1isize);
        }

        graphics.clear_back_buffer();

        for i in 0..PADDLE_SIZE {
            graphics.set_pixel(5,  (y_left + i) as usize, 255, 255, 255);
            graphics.set_pixel(59, (y_right + i) as usize, 255, 255, 255);
        }

        graphics.set_pixel(ball_pos.0 as usize, ball_pos.1 as usize, 255, 255, 255);

        graphics.flush();

        graphics.write_text((WIN_WIDTH / 2 - 4) as usize, 2, format!("{}", score.0));
        graphics.write_text((WIN_WIDTH / 2 + 2) as usize, 2, format!("{}", score.1));


        thread::sleep(Duration::from_millis(80));
    }
}