trait Game {
    fn new() -> Self;

    fn play(&mut self);

    fn is_over(&self) -> bool;

    fn display(&self);
}

fn game_loop<G : Game> (mut game : G) {
    while !game.is_over() {
        game.display();
        game.play();
    }
}