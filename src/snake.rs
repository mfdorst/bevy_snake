use bevy::{core::FixedTimestep, prelude::*};

use crate::{
    components::{Direction, HeadDirection, Position, Size, SnakeHead},
    consts::*,
    resources::{Materials, Snake},
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage("spawn_snake", SystemStage::single(spawn_snake.system()))
            .add_system(
                snake_input
                    .system()
                    .label(Snake::Input)
                    .before(Snake::Movement),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.150))
                    .with_system(snake_move.system().label(Snake::Movement)),
            );
    }
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(HeadDirection {
            current: Direction::Right,
            next: Direction::Right,
        })
        .insert(Position::new(STARTING_POSITION_X, STARTING_POSITION_Y))
        .insert(Size::square(SNAKE_HEAD_SIZE));
}

fn snake_input(key: Res<Input<KeyCode>>, mut query: Query<(&mut HeadDirection, &SnakeHead)>) {
    // There is only one snake head. Get it:
    let (mut direction, _) = query.iter_mut().next().expect("The snake has no head");
    let new_direction = if key.just_pressed(KeyCode::Left) {
        Direction::Left
    } else if key.just_pressed(KeyCode::Right) {
        Direction::Right
    } else if key.just_pressed(KeyCode::Up) {
        Direction::Up
    } else if key.just_pressed(KeyCode::Down) {
        Direction::Down
    } else {
        direction.next
    };
    if new_direction != direction.current.opposite() {
        direction.next = new_direction;
    }
}

fn snake_move(mut query: Query<(&SnakeHead, &mut HeadDirection, &mut Position)>) {
    use Direction::*;
    for (_, mut direction, mut position) in query.iter_mut() {
        match direction.next {
            Up => position.y += 1,
            Down => position.y -= 1,
            Right => position.x += 1,
            Left => position.x -= 1,
        }
        direction.current = direction.next;
    }
}
