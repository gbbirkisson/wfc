use rand::seq::SliceRandom;

pub type Error = String;
pub type Entropy = usize;

/// Cells that the wave function collapse operates on
pub trait Cell<V> {
    fn collapse(&mut self, value: Option<V>) -> Result<V, Error>;
    fn constrain(&mut self, value: &V) -> Result<(), Error>;
}

pub trait WaveFunctionCollapse<Id, Value>
where
    Id: Clone,
{
    /// Returns all cells that have not been collapsed already
    fn cells_to_collapse(&self) -> Vec<(Id, Entropy)>;

    /// Returns cell neighbours
    fn cell_neighbours(&self, id: &Id) -> Vec<&Id>;

    /// Collapses a single cell
    fn cell_collapse(&mut self, id: &Id, value: Option<Value>) -> Result<Value, Error>;

    /// Constrains a single cell
    fn cell_constrain(&mut self, id: &Id, value: &Value) -> Result<(), Error>;

    /// Picks one random cell out of the group that has the lowest entropy
    fn cell_with_lowest_entropy(&self) -> Option<(Id, Entropy)> {
        let mut collapsable_cells = self.cells_to_collapse();
        collapsable_cells.sort_by(|(_, entropy1), (_, entropy2)| entropy1.cmp(entropy2));

        let lowest_entropy = collapsable_cells.iter().next().map(|(_, e)| e).cloned();

        if let Some(lowest_entropy) = lowest_entropy {
            collapsable_cells
                .into_iter()
                .filter(|(_, entropy)| entropy == &lowest_entropy)
                .collect::<Vec<_>>()
                .choose(&mut rand::thread_rng())
                .cloned()
        } else {
            None
        }
    }

    /// Collapses one cell with and constrains its neightbours
    fn collapse_one(&mut self, id: &Id, value: Option<Value>) -> Result<(), Error> {
        let value = self.cell_collapse(id, value)?;
        let neighbours: Vec<Id> = self
            .cell_neighbours(id)
            .into_iter()
            .map(|v| v.clone())
            .collect();
        for id in neighbours {
            self.cell_constrain(&id, &value)?;
        }
        Ok(())
    }

    /// Collapses one random cell that has the lowest entropy
    fn collapse_lowest_entropy(&mut self) -> Result<bool, Error> {
        match self.cell_with_lowest_entropy() {
            Some((_, entropy)) if entropy == 0 => Err(format!("Got uncollapsable cell")),
            Some((id, _)) => {
                self.collapse_one(&id, None)?;
                Ok(false)
            }
            None => Ok(true),
        }
    }

    /// Collapses all cells, one by one
    fn collapse_all(&mut self) -> Result<(), Error> {
        loop {
            if self.collapse_lowest_entropy()? {
                break;
            }
        }
        Ok(())
    }
}
