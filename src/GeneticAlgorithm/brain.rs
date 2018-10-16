use rand;
use rand::{thread_rng, Rng};

use std::f32::consts;
use std::cmp::Ordering::Equal;

pub const CROSSOVER_RATE: f32 = 0.5;
pub const MUTATE_RATE: f32 = 0.5;
pub const MUTATE_AMOUNT: f32 = 0.01;

#[derive(Clone)]
pub struct Genome {
  num_connections_per_gene: usize,
  weights: Vec<f32>,
  total_value: f32,
}

impl Genome {
  pub fn new(num_connections_per_gene: usize) -> Genome {
    let mut rng = thread_rng();
    
    let mut new_weights = Vec::with_capacity(num_connections_per_gene);
    for i in 0..num_connections_per_gene {
      new_weights.push(rng.gen_range(0.0, 1.0));
    }
    
    Genome {
      num_connections_per_gene: num_connections_per_gene,
      weights: new_weights,
      total_value: 0.0,
    }
  }
  
  pub fn new_filled(input: f32) -> Genome {
    Genome {
      num_connections_per_gene: 0,
      weights: Vec::new(),
      total_value: input,
    }
  }
  
  pub fn new_with_weights(weights: Vec<f32>, num_connections_per_gene: usize) -> Genome {
    Genome {
      num_connections_per_gene: num_connections_per_gene,
      weights: weights,
      total_value: 0.0,
    }
  }
  
  pub fn recieve_input(&mut self, genomes: Vec<Genome>) {
    // summation()
    // activation()
    let final_summation = self.summation(genomes);
    self.total_value = self.activation(final_summation);
  }
  
  pub fn summation(&mut self, genomes: Vec<Genome>) -> f32 {
    // inputs weighted and added up
    println!("{:?}", self.weights.len());
    let mut weighted_sum = 0.0;
    for i in 0..genomes.len() as usize {
      weighted_sum += self.weights[i] * genomes[i].genome_strength();
    }
    weighted_sum
  }
  
  pub fn activation(&mut self, summation: f32) -> f32 {
    // 1 / (1 + e^(-x))
    let mut strength = -1.0;
    strength = 1.0 / (1.0 + (-summation).exp());
    
    strength
  }
  
  pub fn get_weights(&self) -> Vec<f32> {
    self.weights.clone()
  }
  
  pub fn genome_strength(&self) -> f32 {
    self.total_value
  }
  
  pub fn print_weights(&self) {
    for i in 0..self.num_connections_per_gene as usize {
      print!("{}, ", self.weights[i]);
    }
    println!("");
  }
}

#[derive(Clone)]
pub struct Layers {
  fitness: f32,
  num_input_nodes: usize,
  num_output_nodes: usize,
  num_hidden_nodes: usize,
  num_hidden_layers: usize,
  input_layer: Vec<Genome>,
  output_layer: Vec<Genome>,
  hidden_layers: Vec<Vec<Genome>>,
}

impl Layers {
  pub fn new(num_input_nodes: usize, num_output_nodes: usize, num_hidden_nodes: usize, num_hidden_layers: usize,) -> Layers {
    let mut input_layer = Vec::with_capacity(num_input_nodes);
    let mut hidden_layers = Vec::with_capacity(num_hidden_layers);
    let mut output_layer = Vec::with_capacity(num_output_nodes);
    
    for i in 0..num_input_nodes {
      input_layer.push(Genome::new(0));
    }
    for i in 0..num_hidden_layers {
      if i == 0 {
        hidden_layers.push(Vec::with_capacity(num_input_nodes));
      } else {
        hidden_layers.push(Vec::with_capacity(num_hidden_nodes));
      }
      
      for j in 0..num_hidden_nodes {
        if i == 0 {
          hidden_layers[i].push(Genome::new(num_input_nodes));
        } else {
          hidden_layers[i].push(Genome::new(num_hidden_nodes));
        }
      }
    }
    
    for i in 0..num_output_nodes {
      output_layer.push(Genome::new(num_hidden_nodes));
    }
    
    Layers {
      fitness: 0.0,
      num_input_nodes: num_input_nodes,
      num_output_nodes: num_output_nodes,
      num_hidden_nodes: num_hidden_nodes,
      num_hidden_layers: num_hidden_layers,
      input_layer: input_layer,
      output_layer: output_layer,
      hidden_layers: hidden_layers,
    }
  }
  
  pub fn new_with_weights(&mut self, weights: Vec<f32>) {
    let mut pos = 0;
    
    let mut hidden_weights = Vec::new();
    let mut output_weights = Vec::new();
    
    for i in 0..self.num_hidden_layers {
      hidden_weights.push(Vec::new());
      for j in 0..self.num_hidden_layers {
        hidden_weights[i].push(weights[pos]);
        pos += 1;
      }
    }
    
    for i in 0..self.num_output_nodes {
      output_weights.push(weights[pos]);
      pos += 1;
    }
    
    for i in 0..self.num_hidden_layers {
      for j in 0..self.num_hidden_nodes {
        if i == 0 {
          self.hidden_layers[i][j] = Genome::new_with_weights(hidden_weights[i].clone(), self.num_input_nodes);
        } else {
          self.hidden_layers[i][j] = Genome::new_with_weights(hidden_weights[i].clone(), self.num_hidden_nodes);
        }
      }
    }
    
    for i in 0..self.num_output_nodes {
      self.output_layer.push(Genome::new_with_weights(output_weights.clone(), self.num_hidden_nodes));
    }
  }
  
  pub fn set_fitness(&mut self, new_fitness: f32) {
    self.fitness = new_fitness;
  }
  
  pub fn get_fitness(&self) -> f32 {
    self.fitness
  }
  
  pub fn get_output(&self) -> Vec<f32> {
    let mut weights = Vec::with_capacity(self.output_layer.len());
    for genome in &self.output_layer {
      weights.push(genome.genome_strength());
    }
    
    weights
  }
  
  pub fn get_weights(&self) -> Vec<f32> {
    let mut weights = Vec::with_capacity(self.num_hidden_layers*self.num_hidden_nodes + self.num_output_nodes);
    for layer in &self.hidden_layers {
      for genome in layer {
        for weight in genome.get_weights() {
          weights.push(weight);
        }
      }
    }
    
    for genome in &self.output_layer {
      for weight in genome.get_weights() {
        weights.push(weight);
      }
    }
   // println!("{}", weights.len());
    weights
  }
  
  pub fn calculate_output(&mut self, input: Vec<f32>) {
    for i in 0..input.len() {
      self.input_layer[i] = Genome::new_filled(input[i]);
    }
    
    let i_layer = self.input_layer.clone();
    let h_layers = self.hidden_layers.clone();
    for i in 0..h_layers.len() {
      for j in 0..h_layers[i].len() {
        if i == 0 {
          self.hidden_layers[i][j].recieve_input(i_layer.to_vec())
        } else {
          self.hidden_layers[i][j].recieve_input(h_layers[i-1].clone());
        }
      }
    }
    
    let last_h_layer = &self.hidden_layers[self.num_hidden_layers-1];
    for layer in &mut self.output_layer {
      layer.recieve_input(last_h_layer.to_vec());
    }
  }
  
  pub fn raw_genome_collection(&self) -> Vec<Genome> {
    let input = self.input_layer.clone();
    let hidden = self.hidden_layers.clone();
    let output = self.output_layer.clone();
    
    let total_num_genomes: usize = (self.num_input_nodes + self.num_hidden_nodes*self.num_hidden_layers + self.num_output_nodes) as usize;
    
    let mut all_genomes = Vec::with_capacity(total_num_genomes);
    
    for genome in input {
      all_genomes.push(genome);
    }
    
    for layer in hidden {
      for genome in layer {
        all_genomes.push(genome);
      }
    }
    
    for genome in output {
      all_genomes.push(genome);
    }
    
    all_genomes
  }
  
  pub fn print_weights(&self) {
    for i in 0..self.num_hidden_layers as usize {
      for j in 0..self.num_hidden_nodes as usize {
        self.hidden_layers[i][j].print_weights();
      }
    }
    for i in 0..self.num_output_nodes as usize {
      self.output_layer[i].print_weights();
    }
  }
}

pub struct Population {
  generation: i32,
  population_size: u32,
  population: Vec<Layers>,
  target: Vec<f32>,
  best_fitness: f32,
  best_layer: Layers,
}

impl Population {
  pub fn new(population_size: u32, input_size: usize, output_size: usize, target: Vec<f32>) -> Population {
    let generation = 0;
    let best_fitness = 999.99;
    
    let num_input_nodes = input_size;
    let num_output_nodes = output_size;
    let num_hidden_nodes = num_input_nodes*2;
    let num_hidden_layers = 1;
    
    let mut population = Vec::with_capacity(population_size as usize);
    for i in 0..population_size as usize {
      population.push(Layers::new(num_input_nodes, num_output_nodes, num_hidden_nodes, num_hidden_layers));
    }
    let best_layer = population[0].clone();
    
    Population {
      generation: generation,
      population_size: population_size,
      population: population,
      target: target,
      best_fitness: best_fitness,
      best_layer: best_layer,
    }
  }
  
  fn order_population(&mut self) {
    self.population.sort_by(|a, b| {
                            a.get_fitness().partial_cmp(&b.get_fitness()).unwrap_or(Equal)
                            });
  }
  
  pub fn next_generation(&mut self) {
    println!("Next Generation");
    let mut rng = thread_rng();
    self.generation += 1;
    
    self.order_population();
    let (parent_1, parent_2) = self.selection();
    let parents = vec!(parent_1.clone(), parent_2.clone());
    
    for i in 0..self.population_size {
      let parent_num = rng.gen_range(0, 1);
      let mut child: Vec<f32> = parents[parent_num].clone();
      let should_crossover = rng.gen_range(0.0, 1.0);
      let should_mutate = rng.gen_range(0.0, 1.0);
      if should_crossover < CROSSOVER_RATE {
        child = self.crossover(parents[0].clone(), parents[1].clone());
      }
      if should_mutate < MUTATE_RATE {
        child = self.mutate(child);
      }
      
      self.population[i as usize].new_with_weights(child);
    }
  }
  
  //1. selection
  pub fn selection(&mut self) -> (Vec<f32>, Vec<f32>) {
    let mut temp_pop = self.population.clone();
    
    let mut parents = Vec::with_capacity(2);
    parents.push(self.population[0].clone());
    parents.push(self.population[1].clone());
    
    for p in 0..parents.len() {
      let mut probability = Vec::new();
      let mut remaining = 100.0;
      
      let mut total_fitness = 0.0;
      for i in 0..temp_pop.len() as usize {
        total_fitness += temp_pop[i].get_fitness();
      }
      
      for i in 0..temp_pop.len() {
        let percentage = (temp_pop[i].get_fitness() / total_fitness);
        let one_minus = 1.0 - percentage;
        let probability_percentage = one_minus * remaining;
        if i == temp_pop.len() {
          probability.push(remaining);
        } else {
          remaining -= probability_percentage;
          probability.push(probability_percentage);
        }
      }
      
      let mut rng = thread_rng();
      let rand = rng.gen_range(0.0, 1.0);
      
      let mut percentage_so_far = 0.0;
      for k in 0..probability.len() {
        percentage_so_far += probability[k];
        if rand < percentage_so_far {
          parents[p] = temp_pop.remove(k);
          break;
        }
      }
    }
    
    let parent_1_weights = parents[0].get_weights();
    let parent_2_weights = parents[1].get_weights();
    
    (parent_1_weights, parent_2_weights)
  }
  
  //2. crossover
  pub fn crossover(&mut self, parent_1: Vec<f32>, parent_2: Vec<f32>) -> Vec<f32> {
    let mut rng = thread_rng();
    let pos = rng.gen_range(0, parent_1.len()-1);
    
    let mut child = Vec::new();
    
    for i in 0..parent_1.len() {
      if i < pos {
        child.push(parent_1[i]);
      } else {
        child.push(parent_2[i]);
      }
    }
    
    child
  }
  
  //3. mutation
  pub fn mutate(&mut self, child: Vec<f32>) -> Vec<f32> {
    let mut rng = thread_rng();
    let pos = rng.gen_range(0, child.len()-1);
    let increase_decrease = rng.gen_range(0.0, 1.0);
    
    let mut child = child;
    
    if increase_decrease > 0.5 {
      child[pos] += MUTATE_AMOUNT;
    } else {
      child[pos] -= MUTATE_AMOUNT;
    }
    
    child
  }
  
  pub fn run_generation(&mut self, input: Vec<f32>) {
    for pop in &mut self.population {
      pop.calculate_output(input.clone());
    }
  }
  
  pub fn calculate_fitness(&mut self) {
    for layer in &mut self.population {
      let mut score = 0.0;
      
      let target = &self.target;
      let actual = layer.get_output();
      
      let mut rms = Vec::with_capacity(target.len());
      for i in 0..target.len() {
        rms.push((actual[i]-target[i])*(actual[i]-target[i]));
      }
      
      for i in 0..rms.len() {
        score += rms[i];
      }
      
      score = score.sqrt();
      layer.set_fitness(score);
      if score < self.best_fitness {
        self.best_fitness = score;
        self.best_layer = layer.clone();
      }
    }
  }
  
  pub fn print_best_fitness_info(&self) {
    let output = self.best_layer.get_output();
    let fitness = self.best_layer.get_fitness();
    
    println!("Fitness: {}, Output: {:?}", fitness, output);
  }
  
  pub fn print_best_fitness_output(&self) {
    println!("{:?}", self.best_layer.get_output());
  }
  
  pub fn print_best_fitness_weights(&self) {
    println!("{:?}", self.best_layer.get_weights());
  }
}

