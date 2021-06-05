use crate::labmaps::*;

pub struct WheelFormat;

impl LabMapFormat for WheelFormat{

    //TODO  IMPORTANT   Map generation process
    fn generate_map(&self, iso_algos: &Vec<AlgoConfiguration>,
        random_opti: &AlgoConfiguration, final_tail: &Vec<AlgoConfiguration>,
        priorities: &[f64; 3], mix_method: String) -> Result<JsonData, Errcode>{

        let mut res = vec![];
        res.push(random_opti.clone());
        let start_ind =  if final_tail.len() == 0{
            res.push(self.generate_final_algo());
            2
        }else{
            res.extend(final_tail.clone());
            1 + final_tail.len()
        };

        res.extend(iso_algos.clone());
        if iso_algos.len() <= 2{    // No Mix layer

        } else {                    // Create Mix layers
            self.generate_mixes(&mut res, priorities[1], start_ind, mix_method, 1);
        }

        // Give the new best cell to every ISO
        res[1].give = (start_ind..(start_ind+iso_algos.len())).collect();
        self.to_json(res)
    }
}
impl WheelFormat{
    pub fn new() -> WheelFormat{ WheelFormat {} }

    fn generate_final_algo(&self) -> AlgoConfiguration{
        AlgoConfiguration::default()
    }

    fn generate_mixes(&self, algos: &mut Vec<AlgoConfiguration>, priority: f64,
        start: usize, method: String, output: usize) {

        for i in 0..algos.len(){
            for j in 0..algos.len(){
                if j >= i { break; }
                let ind = algos.len();
                algos[start+i].give.push(ind);
                algos[start+j].give.push(ind);
                algos.push(self.create_mix(algos.len(), &algos[start+i], &algos[start+j],
                    priority, method.clone(), output))
            }
        }
    }

    fn create_mix(&self, ind: usize, isoA: &AlgoConfiguration, isoB: &AlgoConfiguration,
        priority: f64, method: String, output: usize) -> AlgoConfiguration {
        let mut algo = AlgoConfiguration::method_default(method).unwrap();

        algo.impr_genes = Some(self.extract_impr_genes(isoA, isoB));
        algo.give = vec![output];
        algo
    }

    fn extract_impr_genes(&self, isoA: &AlgoConfiguration, isoB: &AlgoConfiguration
        ) -> Vec<usize>{

        let mut res = vec![];
        if let Some(g) = &isoA.impr_genes{
            res.extend(g);
        }
        if let Some(g) = &isoB.impr_genes{
            res.extend(g);
        }
        res.sort();
        res.dedup();
        res
    }

    fn to_json(&self, data: Vec<AlgoConfiguration>) -> Result<JsonData, Errcode> {
        Ok(serde_json::to_string(&data)?)
    }
}

