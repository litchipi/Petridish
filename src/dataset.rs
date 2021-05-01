
pub type GenalgoData = Vec<f64>;

pub trait DatasetHandler{
    fn get_next_data(&mut self) -> Option<GenalgoData>;      // Panics if no more data
}
