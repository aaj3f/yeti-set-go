use crate::game::state::Game;

pub fn check_collisions(game: &mut Game) {
    let mut items_to_remove = Vec::new();
    let mut should_game_over = false;

    let (yeti_x, yeti_y, yeti_w, yeti_h) = game.yeti.get_collision_rect();

    for (i, item) in game.items.iter().enumerate() {
        let (item_x, item_y, item_w, item_h) = item.get_collision_rect();

        if yeti_x < item_x + item_w
            && yeti_x + yeti_w > item_x
            && yeti_y < item_y + item_h
            && yeti_y + yeti_h > item_y
        {
            if item.is_good {
                game.score += 10;
                game.checks_completed += 1;
            } else {
                should_game_over = true;
            }

            items_to_remove.push(i);
        }
    }

    for &i in items_to_remove.iter().rev() {
        game.items.remove(i);
    }

    if should_game_over {
        game.game_over();
    }
}
