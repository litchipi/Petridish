
use serde_json::*;
pub type JsonData = String;

pub (crate) fn format_error(msg: &str, code: &str, add_data: serde_json::Value) -> JsonData {
    serde_json::to_string(&json!({
        "error":msg,
        "errcode":code,
        "add_data":add_data
    })).expect("Cannot format error message")
}

pub struct MeanCompute{
    sumweights: f64,
    pub result: f64
}

impl MeanCompute{
    pub fn new() -> MeanCompute{
        MeanCompute { sumweights: 0.0, result: 0.0}
    }

    pub fn add_el(&mut self, element: f64, weight: f64) -> f64 {
        let res = ((self.result*self.sumweights) + (element*weight))/(self.sumweights+weight);
        self.result = res;
        self.sumweights += weight;
        res
    }
}


pub struct MeanComputeVec{
    sumweights: f64,
    pub result: Vec<f64>
}

impl MeanComputeVec{
    pub fn new(nb: usize) -> MeanComputeVec{
        MeanComputeVec {
            sumweights: 0.0,
            result: {
                let mut res = vec![];
                for i in 0..nb{
                    res.push(0.0);
                }
                res}
        }
    }

    pub fn add_el(&mut self, element: &Vec<f64>, weight: f64){
        for i in 0..element.len(){
            let res = ((self.result[i]*self.sumweights) + (element[i]*weight))/(self.sumweights+weight);
            self.result[i] = res;
        }
        self.sumweights += weight;
    }
}

pub struct StddevComputeVec{
    count: u32,
    pub result: Vec<f64>,
    pub ex2: Vec<f64>,
    pub mean: Vec<f64>
}

impl StddevComputeVec{
    pub fn new(mean: Vec<f64>) -> StddevComputeVec{
        StddevComputeVec {
            count: 0,
            result: {
                let mut res = vec![];
                for i in 0..mean.len(){
                    res.push(0.0);
                }
                res
            },
            ex2: {
                let mut res = vec![];
                for i in 0..mean.len(){
                    res.push(0.0);
                }
                res
            },
            mean: mean
        }
    }

    pub fn add_el(&mut self, element: &Vec<f64>){
        self.count += 1;
        for i in 0..self.result.len(){
            self.result[i] = {
                if self.count < 2{
                    0.0
                }else{
                    let diff = element[i] - self.mean[i];
                    let diff2 = diff*diff;
                    self.ex2[i] += diff2;
                    (self.ex2[i] - (diff2/ (self.count as f64))) / ((self.count - 1) as f64)
                }
            };
            if self.result[i] == f64::NAN {
                println!("{} {} {} {}", self.count, self.mean[i], self.ex2[i], self.result[i]);
                panic!("Got a NAN on variance calculation");
            }
        }
 //       println!("{:?}", self.result);
    }
}

#[test]
fn test_average_util(){
    let mut avg = MeanCompute::new();
    assert_eq!(avg.add_el(2.0, 1.0), 2.0);
    assert_eq!(avg.add_el(1.0, 1.0), 1.5);
    assert_eq!(avg.add_el(4.5, 2.0), 3.0);
    assert_eq!(avg.add_el(9.0, 4.0), 6.0);
}

#[test]
fn test_average_vec_util(){
    let mut avg = MeanComputeVec::new(3);
    avg.add_el(&vec![1.0, 1.0, 1.0], 1.0);
    assert_eq!(avg.result, vec![1.0, 1.0, 1.0]);
    avg.add_el(&vec![2.0, 1.0, 0.0], 1.0);
    assert_eq!(avg.result, vec![1.5, 1.0, 0.5]);
}
