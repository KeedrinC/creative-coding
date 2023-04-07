//! The environment, the entities that inhabit it, and the rules of our world are modeled here.
//! 
//! The main entities are the Player and Enemy.
//! Enemies are hazardous to the player, and will kill them on collision.
//! The Player's purpose in life is to float around this environment and avoid death until it cannot.
//! The Enemy's purpose in life is to wiggle around randomly until the end of time.

use nannou::{color::Rgb, rand::random_range, App, Draw};

/// 2D Coordinates of an entity
pub struct Position {pub x: f32, pub y: f32}

/// Keeps track of all entities: the player and their enemies.
pub struct World {pub player: Player, pub enemies: Vec<Enemy>}

/// Plays, learns, and evolves.
pub struct Player {
	pub position: Position,
	pub radius: f32,
	pub color: Rgb,
	pub alive: bool,
}

impl Default for Player {
	fn default() -> Self {
		Self {
			position: Position {x: 0., y: 0.},
			radius: 5.0,
			color: Rgb::new(255.0, 255.0, 255.0),
			alive: true,
		}
	}
}

/// Obstacle to Player
pub struct Enemy {
	pub position: Position,
	pub radius: f32,
	pub color: Rgb,
	pub alive: bool,
}

impl Default for Enemy {
	fn default() -> Self {
		Self {
			position: Position {x: 0., y: 0.},
			radius: 5.0,
			color: Rgb::new(255.0, 0.0, 0.0),
			alive: true,
		}
	}
}

/// Draws entities from the world to the nannou window.
/// This function is called throughout the program to redraw
/// each entity's positions, color, etc. as they are updated.
/// 
/// Arguments
/// * `draw`: nannou::draw instance.
/// * `world`: the world struct.
pub fn draw_view(draw: &Draw, world: &World) {
	let World {player, enemies}: &World = world;
	let quick_draw = |position: &Position, &radius, &color| {
		draw.ellipse()
			.x_y(position.x, position.y)
			.radius(radius)
			.color(color);
	};
	quick_draw(&player.position, &player.radius, &player.color);
    for enemy in enemies.iter() {
		quick_draw(&enemy.position, &enemy.radius, &enemy.color);
    }
}

pub fn update(app: &App, world: &mut World) {
	controls(app, world);
	if world.player.alive {
		gameplay(app, world);
		detect_collisions(world);
		handle_bounds(app, world);
	}
}

/// Creates an instance of the world struct.
/// 
/// Returns
/// * `world`: the world struct.
pub fn setup_world() -> World {
	let player: Player = Player::default();
	// spawn enemies and scatter them across the environment
	let num_enemies: i32 = 500;
    let enemies: Vec<Enemy> = (0..num_enemies)
        .map(|_| Enemy {position: enemy_spawn_position(&player), ..Default::default()})
        .collect();
	World {player, enemies}
}

/// User inputs to control world attributes
/// 
/// Arguments
/// * `app`: nannou::app instance.
/// * `world`: the world struct.
pub fn controls(app: &App, world: &mut World) {
	// Left Click: Restart
	if app.mouse.buttons.left().is_down() && !world.player.alive {
		*world = setup_world();
	}
}

/// Handles actions that should happen while in-game.
/// 
/// Arguments
/// * `app`: nannou::app instance.
/// * `world`: the world struct.
pub fn gameplay(app: &App, world: &mut World) {
	let World {player, enemies}: &mut World = world;
    player.position = Position {x: app.mouse.x, y: app.mouse.y}; // Follow Mouse
    for enemy in enemies.iter_mut() {
		// make enemies move randomly in any direction
        enemy.position.x = random_range(enemy.position.x - 1., enemy.position.x + 1.) ;
        enemy.position.y = random_range(enemy.position.y - 1., enemy.position.y + 1.) ;
	}
}

/// Detects enemy collision with a player.
/// If a collision is detected, the player is killed, their color changes to black
/// and gameplay stops updating.
///
/// Arguments
/// * `world`: the world struct.
fn detect_collisions(world: &mut World) {
	let World {player, enemies}: &mut World = world;
	for enemy in enemies.iter_mut() {
		let radius: f32 = player.radius + enemy.radius; // collision distance
		let x: f32 = (player.position.x - enemy.position.x).abs(); // actual x distance
		let y: f32 = (player.position.y - enemy.position.y).abs(); // actual y distance
		if x < radius && y < radius { // Collision detected
			player.color = Rgb::new(0.0, 0.0, 0.0);
			player.alive = false;
			// note: maybe modify a game struct here in the future
		}
	}
}

/// Handles which bounds affect which entities.
///
/// Arguments
/// * `app`: nannou::app instance.
/// * `world`: the world struct.
fn handle_bounds(app: &App, world: &mut World) {
	let World {player, enemies}: &mut World = world;
	// both player and enemies are affected by the world boundary
	world_boundary(app, &mut player.position);
	for enemy in enemies.iter_mut() {
		world_boundary(app, &mut enemy.position);
    }
}

/// Basic world boundary. This prevents all entities from moving beyond the window.
/// 
/// Arguments
/// * `app`: nannou::app instance.
/// * `position`: the 2D position struct.
fn world_boundary(app: &App, position: &mut Position) {
	if position.y > app.window_rect().top() {
		position.y = app.window_rect().top();
	}
	if position.y < app.window_rect().bottom() {
		position.y = app.window_rect().bottom();
	}
	if position.x < app.window_rect().left() {
		position.x = app.window_rect().left();
	}
	if position.x > app.window_rect().right() {
		position.x = app.window_rect().right();
	}
}

/// Creates a random position with a minimum distance from the player.
/// By default, no enemy will spawn within 2x the player's radius.
/// 
/// Arguments
/// * `player`: the main player struct.
/// Returns
/// * `position`: a random position

fn enemy_spawn_position(player: &Player) -> Position {
	let Player {position, radius, ..} = player;
	let radius = radius * 2.;
	let size = 512. / 2. * 0.80;
	Position {
		x: match random_range(0., 1.) > 0.5 {
			true => random_range(-size, position.x - radius),
			false => random_range(position.x + radius, size)
		},
		y: match random_range(0., 1.) > 0.5 {
			true => random_range(-size, position.y - radius),
			false => random_range(position.y + radius, size)
		}
	}
}