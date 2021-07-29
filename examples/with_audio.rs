use std::time::Instant;

fn main() {
    println!("Building dex...");
    let start = Instant::now();
    firecore_pokedex_builder::compile("assets/pokedex/pokemon", "assets/pokedex/moves", "assets/pokedex/items", "assets/pokedex/trainers", Some("assets/pokedex/battle_moves"), Some("output/dex.bin"));
    println!("Finished in {}ms!", start.elapsed().as_millis());
}