use crate::prelude::individual::Individual;
use std::fmt::Debug;

#[allow(dead_code)]
pub struct SolverWrapper<T: Debug>{
    population: Vec<Individual<T>>,
    pub mutation: fn(&mut T),
    pub mait: fn(&[&T]) -> T,
    pub selection: fn(&mut Vec<Individual<T>>),
    pub maiting_group: fn(&Vec<Individual<T>>) -> Vec<&[Individual<T>]>,
    pub fittness: fn(&T) -> f64,
}

impl<T:Debug> SolverWrapper<T> { 
    pub fn new(
        population: Vec<Individual<T>>,
        mutation: fn(&mut T),
        fittness: fn(&T) -> f64,
        mait: fn(&[&T]) -> T,
        maiting_group: fn(&Vec<Individual<T>>) ->Vec<&[Individual<T>]>,
        selection: fn(&mut Vec<Individual<T>>),
    ) -> Self{
        SolverWrapper{
            population,
            mutation,
            fittness,
            mait,
            maiting_group,
            selection,
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    fn runner<T: Debug>(solver: &mut SolverWrapper<T>){
        for indvidual in solver.population.iter_mut() {
            (solver.mutation)(indvidual.get_mut_chromosome());
            let value = (solver.fittness)(indvidual.get_chromosome());
            indvidual.set_fittness_score(value);
        }
        let maiting_group = (solver.maiting_group)(&solver.population);
        let mut offsprings: Vec<Individual<T>> = Vec::new();
        for parents in  maiting_group.into_iter(){
            let parents: Vec<&T> = parents
                        .into_iter()
                        .map(|val| val.get_chromosome())
                        .collect();
            let offspring = (solver.mait)(&parents);
            offsprings.push(Individual::new(offspring));
        }
        offsprings
            .into_iter()
            .for_each(|offspring| solver.population.push(offspring));
        println!("{:?}", solver.population);
    }

    #[test]
    fn simple_run_i32_array_size_2(){
        fn mutation(val: &mut [i32;2]){
            val[0] += 1;
            val[1] += 1;
        }
        fn fittness(val: &[i32;2]) -> f64{
            let sum : i32=  val.iter().sum();
            return sum as f64;
        }
        fn selection(population: &mut Vec<Individual<[i32;2]>>){
            population.retain(
                |val| {
                    match val.get_fittness_score() {
                        Some(_) => true,
                        None => false,
                    }
                });    
        }
        fn mait(parents: &[&[i32;2]])-> [i32;2]{
            let mut new_indv = [0;2];
            new_indv[0] = parents[0][1];
            new_indv[1] = parents[1][1];
            return new_indv;
        }
        fn maiting_group(population: &Vec<Individual<[i32;2]>>) -> Vec<&[Individual<[i32;2]>]>{
            let mut maiting_group : Vec<&[Individual<[i32;2]>]> = Vec::new();
            for group in population.chunks(2){
                maiting_group.push(group);
            }
            return maiting_group;
        }
        let mut population = Vec::<Individual<[i32;2]>>::new();
        for i in 0..20{
            let mut var = [0;2];
            var[0] = i;
            var[1] = i + 1;
            population.push(Individual::new(var));
        }
        let mut solver = SolverWrapper::<[i32;2]>::new(
            population,
            mutation,
            fittness,
            mait,
            maiting_group,
            selection,
        );
        runner(&mut solver);
    }

}