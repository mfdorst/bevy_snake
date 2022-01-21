use bevy::{core::FixedTimestep, prelude::*};

use crate::{
    components::{Direction, Position, Size, TailSegment},
    consts::*,
    resources::{CurrentDirection, InputDirection, Snake, SnakeSystem},
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("spawn_snake", SystemStage::single(spawn_snake))
            .insert_resource(InputDirection(INITIAL_DIRECTION))
            .insert_resource(CurrentDirection(INITIAL_DIRECTION))
            .add_system(
                snake_input
                    .label(SnakeSystem::Input)
                    .before(SnakeSystem::Movement),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.150))
                    .with_system(snake_move.label(SnakeSystem::Movement)),
            );
    }
}

fn spawn_snake(mut commands: Commands) {
    let head = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: HEAD_COLOR,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position::new(STARTING_POSITION_X, STARTING_POSITION_Y))
        .insert(Size::square(SNAKE_HEAD_SIZE))
        .id();

    let tail = (1..INITIAL_SNAKE_LENGTH)
        .map(|i| {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: TAIL_COLOR,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TailSegment)
                .insert(INITIAL_DIRECTION)
                .insert(Position::new(STARTING_POSITION_X - i, STARTING_POSITION_Y))
                .insert(Size::square(SNAKE_TAIL_SIZE))
                .id()
        })
        .collect();
    commands.insert_resource(Snake { head, tail });
}

fn snake_input(
    key: Res<Input<KeyCode>>,
    mut input_direction: ResMut<InputDirection>,
    current_direction: Res<CurrentDirection>,
) {
    // InputDirection is the direction most recently input by the user.
    // CurrentDirection is the direction that the snake head moved on the most recent timestep.
    // If the user pressed two buttons in between timesteps, these values may be different.
    let new_direction = if key.just_pressed(KeyCode::Left) {
        Direction::Left
    } else if key.just_pressed(KeyCode::Right) {
        Direction::Right
    } else if key.just_pressed(KeyCode::Up) {
        Direction::Up
    } else if key.just_pressed(KeyCode::Down) {
        Direction::Down
    } else {
        input_direction.0
    };
    if new_direction != current_direction.0.opposite() {
        input_direction.0 = new_direction;
    }
}

fn snake_move(
    mut snake: ResMut<Snake>,
    mut positions: Query<&mut Position>,
    mut current_direction: ResMut<CurrentDirection>,
    input_direction: Res<InputDirection>,
) {
    // Update the move direction
    current_direction.0 = input_direction.0;
    // Move the head in the move direction
    let mut head_pos = positions
        .get_mut(snake.head)
        .expect("Snake head should have a position");
    let prev_head_pos = head_pos.clone();
    match current_direction.0 {
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
    }

    // Pop off the end of the tail
    let tail_end = snake.tail.pop_back().expect("The tail should not be empty");

    // Set the position of the tail end to where the head was
    *positions
        .get_mut(tail_end)
        .expect("All snake segments should have positions") = prev_head_pos;

    // Push the tail end to the front of the tail
    snake.tail.push_front(tail_end);
}
