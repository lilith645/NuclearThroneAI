use rand;
use rand::{thread_rng, Rng};

use std::f32::consts;

pub const P_LEVEL_WEIGHT: f32 =  1.0;
pub const G_LEVEL_WEIGHT: f32 = 0.5;
pub const P_HEALTH_WEIGHT: f32 = 0.2;

#[derive(Clone)]
pub struct Gene {
  weight: f32,
}

impl Gene {
  pub fn new(weight: f32) -> Gene {
    Gene {
      weight: weight,
    }
  }
  
  pub fn new_random() -> Gene {
    let mut rng = thread_rng();
    Gene {
      weight: rng.gen_range(0.0, 1.0),
    }
  }
  
  pub fn weight_value(&self) -> f32 {
    self.weight
  }
}

#[derive(Clone)]
pub struct Genome {
  num_connections_per_gene: u32,
  genes: Vec<Gene>,
  total_value: f32,
}

impl Genome {
  pub fn new(num_connections_per_gene: u32) -> Genome {
    let mut new_genes = Vec::with_capacity(num_connections_per_gene as usize);
    for i in 0..num_connections_per_gene {
      new_genes.push(Gene::new_random());
    }
    
    Genome {
      num_connections_per_gene: num_connections_per_gene,
      genes: new_genes,
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
    let mut weighted_sum = 0.0;
    for i in 0..genomes.len() as usize {
      weighted_sum += self.genes[i].weight_value() * genomes[i].genome_strength();
    }
    weighted_sum
  }
  
  pub fn activation(&mut self, summation: f32) -> f32 {
    // 1 / (1 + e^(-x))
    let mut strength = -1.0;
    strength = 1.0 / (1.0 + consts::E.powf(-summation));
    
    strength
  }
  
  pub fn genome_strength(&self) -> f32 {
    self.total_value
  }
  
  pub fn print_weights(&self) {
    for i in 0..self.num_connections_per_gene as usize {
      print!("{}, ", self.genes[i].weight_value());
    }
    println!("");
  }
}

#[derive(Clone)]
pub struct Layers {
  num_input_nodes: u32,
  num_output_nodes: u32,
  num_hidden_nodes: u32,
  num_hidden_layers: u32,
  input_layer: Vec<Genome>,
  output_layer: Vec<Genome>,
  hidden_layers: Vec<Vec<Genome>>,
}

impl Layers {
  pub fn new(num_input_nodes: u32, num_output_nodes: u32, num_hidden_nodes: u32, num_hidden_layers: u32,) -> Layers {
    let mut input_layer = Vec::with_capacity(num_input_nodes as usize);
    let mut hidden_layers = Vec::with_capacity(num_hidden_layers as usize);
    let mut output_layer = Vec::with_capacity(num_output_nodes as usize);
    
    for i in 0..num_input_nodes {
      input_layer.push(Genome::new(0));
    }
    for i in 0..num_hidden_layers as usize {
      hidden_layers.push(Vec::with_capacity(num_input_nodes as usize));
      for j in 0..num_hidden_nodes as usize {
        hidden_layers[i].push(Genome::new(num_input_nodes));
      }
    }
    for i in 0..num_output_nodes {
      output_layer.push(Genome::new(num_hidden_nodes));
    }
    
    Layers {
      num_input_nodes: num_input_nodes,
      num_output_nodes: num_output_nodes,
      num_hidden_nodes: num_hidden_nodes,
      num_hidden_layers: num_hidden_layers,
      input_layer: input_layer,
      output_layer: output_layer,
      hidden_layers: hidden_layers,
    }
  }
  
  pub fn print_weights(&self) {
    for i in 0..self.num_input_nodes as usize {
      self.input_layer[i].print_weights();
    }
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
  best_fitness: f32,
  best_layer: Layers,
}

impl Population {
  pub fn new(population_size: u32) -> Population {
    let generation = 0;
    let best_fitness = 0.0;
    
    let num_input_nodes = 7;
    let num_output_nodes = 11;
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
      best_fitness: best_fitness,
      best_layer: best_layer,
    }
  }
  
  pub fn print_best_fitness_weights(&self) {
    self.best_layer.print_weights();
  }
  
  fn fitness(player_level: i32, health: i32, actual_level: i32) -> i32 {
    let mut score = 0.0;
    
    score = (player_level as f32 * P_LEVEL_WEIGHT + health as f32 * P_HEALTH_WEIGHT) * actual_level as f32;
    
    score.floor() as i32
  }
}

