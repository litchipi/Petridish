pub type GenalgoData = Vec<f64>;

pub trait DatasetHandler {
    fn prepare(&mut self);
    fn get_next_data(&mut self) -> Option<GenalgoData>;
}

pub struct EmptyDataset {
    pub nemission: usize,
    data_emitted: usize,
}

impl EmptyDataset {
    pub fn new(nemission: usize) -> EmptyDataset {
        EmptyDataset {
            nemission: nemission,
            data_emitted: 0,
        }
    }
}

impl DatasetHandler for EmptyDataset {
    fn prepare(&mut self) {
        self.data_emitted = 0;
    }

    fn get_next_data(&mut self) -> Option<GenalgoData> {
        if self.data_emitted < self.nemission {
            self.data_emitted += 1;
            Option::Some(vec![])
        } else {
            Option::None
        }
    }
}
