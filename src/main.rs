//! This program is a creative coding exercise used as a visual representation
//! of an environment created to test the capabilities of a genetic algorithm.
/// 
/// The goal is to have a player entity navigate through the environment avoiding collision with
/// opposing entities, which will kill the player.

use nannou::prelude::*;
use self::world::World;

mod world;

/// Defines the app's state in nannou.
struct Model {world: World}

/// Main entry point. Builds the app, passes a function to retrieve state
/// and passes a function to call after every update.
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

/// Draws a window and passes state to the app.
fn model(app: &App) -> Model {
    app
        .new_window()
        .size(512, 512)
        .title("Environment")
        .view(view)
        .build()
        .unwrap();
    Model {world: world::setup_world()}
}

/// Called after every update.
fn update(app: &App, model: &mut Model, _update: Update) {
    world::update(app, &mut model.world);
}

/// Draw entities to the canvas.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKSLATEGRAY);
    world::draw_view(&draw, &model.world);
    draw.to_frame(app, &frame).unwrap();
}