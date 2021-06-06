use crate::labmaps::*;

pub struct WheelFormat;

impl LabMapFormat for WheelFormat{

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
            for algo in res[start_ind..].iter_mut(){
                algo.give.push(start_ind-1);
            }
        } else {                    // Create Mix layers
            self.generate_mixes(&mut res, priorities[1], start_ind, iso_algos.len(), mix_method, 1);
        }

        // Give the new best cell to every ISO
        res[1].id = "Final".to_string();
        res[1].give = (start_ind..(start_ind+iso_algos.len())).collect();
        res[1].population = AlgoPopulation::WeightofTot(priorities[2]);
        // Give RandomOpti to every ISO
        res[0].id = "Random".to_string();
        res[0].give = (start_ind..(start_ind+iso_algos.len())).collect();
        res[0].population = AlgoPopulation::WeightofTot(priorities[0]);
        self.to_json(res)
    }
}
impl WheelFormat{
    pub fn new() -> WheelFormat{ WheelFormat {} }

    fn generate_final_algo(&self) -> AlgoConfiguration{
        AlgoConfiguration::default()
    }

    fn generate_mixes(&self, algos: &mut Vec<AlgoConfiguration>, priority: f64,
        start: usize, nb_iso: usize, method: String, output: usize) {

        for i in 0..nb_iso{
            let j = if i == nb_iso-1 { 0 } else { i + 1 };
            let ind = algos.len();
            algos[start+i].give.push(ind);
            algos[start+j].give.push(ind);

            let mut algo_mix = self.create_mix(&algos[start+i], &algos[start+j],
                priority, method.clone(), output);
            if i > 0{
                algo_mix.give.push(algos.len() - 1);
            }
            algo_mix.population = AlgoPopulation::WeightofTot(priority + 
                (if let AlgoPopulation::WeightofTot(p) = algos[start+i].population { p }
                else { unreachable!() } *
                if let AlgoPopulation::WeightofTot(p) = algos[start+j].population { p }
                else { unreachable!() }));
            algos.push(algo_mix);
        }
        let nmix = algos.len() - 1;
        algos[start+nb_iso].give.push(nmix);
    }

    fn create_mix(&self, iso_a: &AlgoConfiguration, iso_b: &AlgoConfiguration,
        priority: f64, method: String, output: usize) -> AlgoConfiguration {
        let mut algo = AlgoConfiguration::method_default(method).unwrap();

        algo.id = format!("Mix_{}_{}", iso_a.id, iso_b.id);
        algo.impr_genes = Some(self.extract_impr_genes(iso_a, iso_b));
        algo.give = vec![output];
        algo.population = AlgoPopulation::WeightofTot(priority);
        algo
    }

    fn extract_impr_genes(&self, iso_a: &AlgoConfiguration, iso_b: &AlgoConfiguration
        ) -> Vec<usize>{

        let mut res = vec![];
        if let Some(g) = &iso_a.impr_genes{
            res.extend(g);
        }
        if let Some(g) = &iso_b.impr_genes{
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

