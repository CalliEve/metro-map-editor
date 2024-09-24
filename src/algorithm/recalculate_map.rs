use super::order_edges::order_edges;
use crate::{
    models::Map,
    utils::Result,
};

/// Recalculate the map, all the positions of the stations and the edges between
/// them, as a whole. This is the Recalculate Map algorithm in the paper.
pub fn recalculate_map(map: &mut Map) -> Result<()> {
    let mut _edges = order_edges(map)?;

    // TODO: Implement the rest of the algorithm

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Map;

    #[test]
    fn test_recalculate_map() {
        let mut map = Map::new();
        recalculate_map(&mut map).unwrap();
    }
}
