struct AverageCompute{
    sumweights: f64,
    result: f64
}

impl AverageCompute{
    fn new() -> AverageCompute{
        AverageCompute { sumweights: 0.0, result: 0.0}
    }

    fn add_el(&mut self, element: f64, weight: f64) -> f64 {
        let res = ((self.result*self.sumweights) + (element*weight))/(self.sumweights+weight);
        self.result = res;
        self.sumweights += weight;
        res
    }
}

#[test]
fn test_average_util(){
    let mut avg = AverageCompute::new();
    assert_eq!(avg.add_el(2.0, 1.0), 2.0);
    assert_eq!(avg.add_el(1.0, 1.0), 1.5);
    assert_eq!(avg.add_el(4.5, 2.0), 3.0);
    assert_eq!(avg.add_el(9.0, 4.0), 6.0);
}
