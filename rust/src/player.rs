use godot::engine::{CharacterBody2D, ICharacterBody2D, InputEvent, InputEventMouseButton};
use godot::global::MouseButton;
use godot::obj::{Base, WithBaseField};
use godot::prelude::*;
use godot::register::GodotClass;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    engine_power: f32,
    #[export]
    space_friction: f32,
    is_moving: bool,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            // Força bruta aplicada continuamente enquanto o botão é pressionado
            engine_power: 300.0,
            // Taxa de desaceleração quando o motor está desligado
            space_friction: 150.0,
            is_moving: false,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // O uso do 'if let' substitui os blocos 'match' aninhados,
        // tornando o tratamento de cast de eventos muito mais legível no Rust.
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if mouse_event.get_button_index() == MouseButton::RIGHT {
                self.is_moving = mouse_event.is_pressed();
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // 1. Rotação Constante
        // Não precisamos mais armazenar o mouse_pos no struct.
        // Buscamos a posição em tempo real a cada frame da física.
        let mouse_pos = self.base().get_global_mouse_position();
        self.base_mut().look_at(mouse_pos);

        // Extraímos a velocidade atual e convertemos o delta para f32
        // para garantir compatibilidade matemática com os vetores 2D da Godot.
        let mut velocity = self.base().get_velocity();
        let delta_f32 = delta as f32;

        // 2. Aplicação da Inércia (Leis de Newton)
        if self.is_moving {
            // No GDExtension, o eixo X (frente da nave) é representado pela coluna 'a' da Transform2D
            let forward = self.base().get_transform().a;

            // Calculamos a força de empuxo deste frame e ADICIONAMOS à velocidade existente
            let thrust = forward * self.engine_power * delta_f32;
            velocity += thrust;
        } else {
            // A função move_toward interpola o vetor atual em direção a um vetor alvo (Zero).
            // Isso simula o atrito sutil do espaço sem zerar a velocidade bruscamente.
            velocity = velocity.move_toward(Vector2::ZERO, self.space_friction * delta_f32);
        }

        // 3. Execução do Movimento
        // Devolvemos o vetor modificado para a engine e ordenamos o processamento da física.
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
