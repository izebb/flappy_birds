use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();

        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, player_x: i32, ctx: &mut BTerm) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in (self.gap_y + half_size)..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let is_x_equal = player.x == self.x;

        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        is_x_equal && (player_above_gap || player_below_gap)
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    player: Player,
    score: i32,
}

fn print_play_or_quit(ctx: &mut BTerm) {
    ctx.print_centered(8, "(P) Play game");
    ctx.print_centered(9, "(Q) Quit game");
}

fn print_welcome(ctx: &mut BTerm) {
    ctx.print_centered(5, "Welcome to Flappy bird");
}

fn print_control_key(ctx: &mut BTerm) {
    ctx.print(0, 0, "Press SPACE key to flap your wings");
}

fn print_score(ctx: &mut BTerm, score: i32) {
    ctx.print(0, 1, &format!("Your Score {} points", score))
}

fn print_game_over(ctx: &mut BTerm, score: i32) {
    ctx.print_centered(5, "Game over! you lost :(");
    ctx.print(0, 1, &format!("Your earn {} points", score));
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25),
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.frame_time = 0.0;
        self.score = 0;
        self.player = Player::new(5, 25);
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);

        print_control_key(ctx);
        print_score(ctx, self.score);

        self.obstacle.render(self.player.x, ctx);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        print_welcome(ctx);
        print_play_or_quit(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        print_game_over(ctx, self.score);
        print_play_or_quit(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy birds")
        .build()?;
    main_loop(context, State::new())
}
