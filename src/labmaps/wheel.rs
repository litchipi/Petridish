/*
use crate::algo::AlgoConfiguration;
use crate::errors::Errcode;
use crate::utils::JsonData;
*/
use crate::genalgomethods::GenalgoMethodsAvailable;
use crate::labmaps::{LabMapCore, LabMapFormat};

//TODO  IMPORTANT   Implement WheelFormat
pub struct WheelFormat;
impl LabMapFormat for WheelFormat {
    fn generate_mixes(&self, core: &mut LabMapCore, method: GenalgoMethodsAvailable){
        todo!();
    }

    fn add_objective(&self, score_opti: Vec<usize>, priority: f64,
        core: &mut LabMapCore, methods: &[GenalgoMethodsAvailable; 4]){
        todo!();
    }

    fn setup_gives(&self, core: &mut LabMapCore) {
        todo!();
    }

    fn setup_objectives(&self, core: &mut LabMapCore) {
        todo!();
    }
}
