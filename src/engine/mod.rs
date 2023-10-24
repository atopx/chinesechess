mod book;
pub mod pregen;
pub mod util;
pub mod core;
pub mod state;
pub mod search;


#[cfg(test)]
mod tests {
    use super::core::Engine;
    use super::search::Search;

    #[test]
    fn test_fen() {
        let fen = "9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w";
        let mut engine = Engine::new();
        engine.from_fen(fen);
        assert_eq!(fen, engine.to_fen());
    }

    #[test]
    fn test_engine() {
        let mut engine = Engine::new();
        let mut search = Search::new(engine.clone(), 16);
        engine.from_fen("9/2Cca4/3k1C3/4P1p2/4N1b2/4R1r2/4c1n2/3p1n3/2rNK4/9 w");
        // println!("{:?}", engine.squares);
        let mv = search.search_main(64, 5000);
        assert_eq!(mv, 26215);
    }
}