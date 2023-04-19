pub type WfcError = String;

pub enum CellState<V> {
    Entropy(usize),
    Value(V),
}

pub trait Cell<V> {
    fn state(&self) -> CellState<V>;
    fn constrain(&mut self, value: &V) -> Result<(), WfcError>;
    fn collapse(&mut self, value: Option<V>) -> Result<V, WfcError>;
}

pub trait Wfc<Id, Value>
where
    Id: Clone,
{
    fn get_cell_with_lowest_entropy(&self) -> Option<(Id, usize)>;
    fn get_cell_neighbours(&self, id: &Id) -> Vec<&Id>;

    fn cell_constrain(&mut self, id: &Id, value: &Value) -> Result<(), WfcError>;
    fn cell_collapse(&mut self, id: &Id, value: Option<Value>) -> Result<Value, WfcError>;

    fn collapse_and_propagate(&mut self, id: &Id, value: Option<Value>) -> Result<(), WfcError> {
        let value = self.cell_collapse(id, value)?;
        let neighbours: Vec<Id> = self
            .get_cell_neighbours(id)
            .into_iter()
            .map(|v| v.clone())
            .collect();
        for id in neighbours {
            self.cell_constrain(&id, &value)?;
        }
        Ok(())
    }

    fn iterate(&mut self) -> Result<bool, WfcError> {
        match self.get_cell_with_lowest_entropy() {
            Some((_, entropy)) if entropy == 0 => Err(format!("Got uncollapsable cell")),
            Some((id, _)) => {
                self.collapse_and_propagate(&id, None)?;
                Ok(false)
            }
            None => Ok(true),
        }
    }

    fn collapse_all(&mut self) -> Result<(), WfcError> {
        loop {
            if self.iterate()? {
                break;
            }
        }
        Ok(())
    }
}
