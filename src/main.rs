mod ventti;

fn main() {
    for max in (6950..7000).step_by(50) {
        ventti::Game::new(max).find();
    }
}
