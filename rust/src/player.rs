use godot::{
    builtin::Vector2,
    classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Input, ProjectSettings},
    global::{godot_print, move_toward},
    obj::{Base, WithBaseField},
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    jump_velocity: f64,

    base: Base<CharacterBody2D>,
}

enum MovementDirection {
    Left,
    Neutral,
    Right,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Initialise player Rust class");
        Self {
            speed: 130.0,
            jump_velocity: -300.0,

            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let Vector2 {
            x: velocity_x,
            y: velocity_y,
        } = self.base().get_velocity();

        let input = Input::singleton();

        // handle jump and gravity
        let new_velocity_y = if self.base().is_on_floor() {
            if input.is_action_pressed("jump") {
                #[allow(clippy::cast_possible_truncation)]
                {
                    self.jump_velocity as f32
                }
            } else {
                velocity_y
            }
        } else {
            let gravity = ProjectSettings::singleton()
                .get_setting("physics/2d/default_gravity")
                .try_to::<f64>()
                .expect("Should be able to represent default gravity as a 32-bit float");
            #[allow(clippy::cast_possible_truncation)]
            {
                velocity_y + (gravity * delta) as f32
            }
        };

        // Get input direction
        let direction = input.get_axis("move_left", "move_right");
        let movement_direction = match direction {
            val if val < -f32::EPSILON => MovementDirection::Left,
            val if (-f32::EPSILON..f32::EPSILON).contains(&val) => MovementDirection::Neutral,
            val if val >= f32::EPSILON => MovementDirection::Right,
            _ => unreachable!(),
        };

        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        // Flip the sprite to match movement direction
        match movement_direction {
            MovementDirection::Left => animated_sprite.set_flip_h(true),
            MovementDirection::Neutral => {}
            MovementDirection::Right => animated_sprite.set_flip_h(false),
        }

        // Play animation
        let animation = if self.base().is_on_floor() {
            match movement_direction {
                MovementDirection::Neutral => "idle",
                MovementDirection::Left | MovementDirection::Right => "run",
            }
        } else {
            "jump"
        };
        animated_sprite.play_ex().name(animation).done();

        // Apply movement
        #[allow(clippy::cast_possible_truncation)]
        let new_velocity_x = match movement_direction {
            MovementDirection::Neutral => {
                move_toward(f64::from(velocity_x), 0.0, self.speed) as f32
            }
            MovementDirection::Left | MovementDirection::Right => direction * (self.speed) as f32,
        };

        self.base_mut().set_velocity(Vector2 {
            x: new_velocity_x,
            y: new_velocity_y,
        });

        self.base_mut().move_and_slide();
    }
}
