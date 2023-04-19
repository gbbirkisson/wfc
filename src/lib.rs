// Inspired by this video: https://www.youtube.com/watch?v=2SuvO4Gi7uY

pub enum CellState<V> {
    Entropy(usize),
    Value(V),
}

pub trait Cell<V> {
    fn state(&self) -> CellState<V>;
    fn constrain(&mut self, value: &V);
    fn collapse(&mut self, value: Option<V>) -> V;
}

pub trait Wfc<Id, Value>
where
    Id: Clone,
{
    fn get_cell_with_lowest_entropy(&self) -> Option<(Id, usize)>;
    fn get_cell_neighbours(&self, id: &Id) -> Vec<&Id>;
    fn get_cell_state(&self, id: &Id) -> CellState<Value>;

    fn cell_collapse(&mut self, id: &Id, value: Option<Value>) -> Value;
    fn cell_constrain(&mut self, id: &Id, value: &Value);

    fn collapse_and_propagate(&mut self, id: &Id, value: Option<Value>) {
        let value = self.cell_collapse(id, value);
        let neighbours: Vec<Id> = self
            .get_cell_neighbours(id)
            .into_iter()
            .map(|v| v.clone())
            .collect();
        for id in neighbours {
            self.cell_constrain(&id, &value);
        }
    }

    fn iterate(&mut self) -> bool {
        match self.get_cell_with_lowest_entropy() {
            Some((_, entropy)) if entropy == 0 => {
                // We have a cell with no options, lets stop iterating
                true
            }
            Some((id, _)) => {
                self.collapse_and_propagate(&id, None);
                false
            }
            None => true,
        }
    }

    fn collapse_all(&mut self) {
        loop {
            if self.iterate() {
                break;
            }
        }
    }
}
