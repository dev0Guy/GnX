use ordered_float::OrderedFloat;
use std::cmp;
use std::fmt::Debug;

type FitnessOption = Option<OrderedFloat<f64>>;

#[derive(Debug)]
pub struct Individual<T: Debug>{
    chromosome: T,
    fitness_score: FitnessOption,
}

impl<T: Debug> Individual<T> {

    pub fn new(chromosome: T) -> Self{
        Individual{
            chromosome,
            fitness_score: None,
        }
    }

    pub fn get_fittness_score(&self) -> FitnessOption {
        self.fitness_score
    }

    pub fn set_fittness_score(&mut self, value: f64){
        self.fitness_score = Some(OrderedFloat(value));
    }

    pub fn get_mut_chromosome(&mut self) -> &mut T{
        &mut self.chromosome
    }

    pub fn get_chromosome(&self) -> &T{
        & self.chromosome
    }
}

impl <T: Debug> cmp::Eq for Individual<T> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl <T: Debug> cmp::PartialEq for Individual<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness_score == other.fitness_score
    }
}

impl<T: Debug> cmp::PartialOrd for Individual<T>{

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.fitness_score{
            Some(val_1) => {
                match other.fitness_score {
                    Some(val_2) => {Some(val_1.cmp(&val_2))},
                    None => Some(cmp::Ordering::Greater)
                }
            },
            None => Some(cmp::Ordering::Less),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(cmp::Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        !matches!(self.partial_cmp(other), None | Some(cmp::Ordering::Greater))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(cmp::Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(cmp::Ordering::Greater | cmp::Ordering::Equal))
    }
}


impl<T: Debug> cmp::Ord for Individual<T>{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.fitness_score{
            Some(val_1) => {
                match other.fitness_score {
                    Some(val_2) => {val_1.cmp(&val_2)},
                    None => cmp::Ordering::Greater
                }
            },
            None => cmp::Ordering::Less,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn init_array(){
        let chromosome: [i32;5] = [0;5];
        let indv = Individual::new(chromosome);
        assert_eq!(indv.fitness_score, None);
        assert_eq!(indv.chromosome, chromosome);
    }

    #[test]
    fn init_vector(){
        let chromosome: Vec<i32> = vec![1,2,3,4,5];
        let chromosome_cpy = chromosome.clone();
        let indv = Individual::new(chromosome);
        assert_eq!(indv.fitness_score, None);
        assert_eq!( indv.chromosome
                    .iter()
                    .zip(chromosome_cpy.iter())
                    .all(|(a,b)| a == b ), true);
    }

    #[test]
    fn init_i32(){
        let chromosome: i32 = 5;
        let indv = Individual::new(chromosome);
        assert_eq!(indv.fitness_score, None);
        assert_eq!( indv.chromosome == chromosome, true);
    }

    #[test]
    fn cmp_lt_none() {
        let mut indv_1 = Individual::new([0;40]);
        let indv_2 = Individual::new([1;40]);
        indv_1.set_fittness_score(-50.0);
        assert_eq!(indv_2 < indv_1, true);
    }

    #[test]
    fn cmp_lt() {
        let mut indv_1 = Individual::new([0;40]);
        let mut indv_2 = Individual::new([1;40]);
        indv_1.set_fittness_score(-50.0);
        indv_2.set_fittness_score(10.0);
        assert_eq!(indv_1 < indv_2, true);
    }

    #[test]
    fn cmp_gt_none() {
        let mut indv_1 = Individual::new([0;40]);
        let indv_2 = Individual::new([1;40]);
        indv_1.set_fittness_score(-50.0);
        assert_eq!(indv_1 > indv_2, true);
    }

    #[test]
    fn cmp_gt() {
        let mut indv_1 = Individual::new([0;40]);
        let mut indv_2 = Individual::new([1;40]);
        indv_1.set_fittness_score(-50.0);
        indv_2.set_fittness_score(10.0);
        assert_eq!(indv_2 > indv_1, true);
    }

    #[test]
    fn cmp_eq_both_none() {
        let indv_1 = Individual::new([0;40]);
        let indv_2 = Individual::new([1;40]);
        assert_eq!(indv_1 == indv_2, true);
        assert_eq!(indv_1 != indv_2, false);
    }

    #[test]
    fn cmp_eq_one_none() {
        let mut indv_1 = Individual::new([0;40]);
        let indv_2 = Individual::new([1;40]);
        indv_1.set_fittness_score(10.0);
        assert_eq!(indv_1 == indv_2, false);
        assert_eq!(indv_1 != indv_2, true);
    }

    #[test]
    fn cmp_eq() {
        let mut indv_1 = Individual::new([0;40]);
        let mut indv_2 = Individual::new([1;40]);
        indv_1.set_fittness_score(10.0);
        indv_2.set_fittness_score(10.0);
        assert_eq!(indv_1 == indv_2, true);
        assert_eq!(indv_1 != indv_2, false);
    }


}