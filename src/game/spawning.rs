use crate::entities::Item;
use crate::game::state::Game;

pub fn spawn_items(game: &mut Game, dt: f32) {
    game.spawn_timer += dt;

    if game.spawn_timer >= game.spawn_rate {
        game.spawn_timer = 0.0;
        spawn_random_item(game);
    }
}

fn spawn_random_item(game: &mut Game) {
    let item = Item::random(&game.textures);
    game.items.push(item);
}