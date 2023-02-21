use crate::Craft;
use core::fmt::Display;
use clap::Parser;

#[cfg(not(feature="no_python"))]
use pyo3::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Parameters {
    pub threads: usize,
    pub verbose: u8,
    pub depth: u32,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the receipe
    #[arg(short, long, default_value_t = String::from("default_recipe"))]
    pub recipe_name: String,

    /// Name of the character
    #[arg(short, long, default_value_t = String::from("default_character"))]
    pub character_name: String,

    /// The ml file name
    #[arg(short, long, default_value_t = String::from("craft.toml"))]
    pub file_name: String,
   
    /// The verbose flag
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// The depth of the first pass
    #[arg(short, long, default_value_t = 8)]
    pub depth: u32,

    /// Thread counts, default is 4 (can you even run ff with less ?)
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,
}


/// A final stripped down version of a craft
/// used for final print and talking with python
#[derive(Debug)]
#[pyclass]
pub struct SolverResult{
    #[pyo3(get)]
    pub steps: u32,
    #[pyo3(get)]
    pub quality: u32,
    #[pyo3(get)]
    pub progression: u32,
    #[pyo3(get)]
    pub durability: i32,
    #[pyo3(get)]
    pub actions: Vec<String>,
    #[pyo3(get)]
    pub step1_solutions: usize,
    #[pyo3(get)]
    pub step2_solutions: usize,
    #[pyo3(get)]
    pub found_100_percent: bool,
}

/// Pretty display for SolverResult
impl Display for SolverResult{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        println!("{:?}",self.actions);
        Ok(())
    }
}

impl SolverResult{
    pub fn from_craft(craft: & Craft,step1_solutions : usize,step2_solutions : usize, found_100_percent: bool)->SolverResult{
        // Todo: recreate actions
        // Where steps ?
        let mut actions = craft.actions.iter().map(|action| 
            format!("{}",action.short_name)
        ).collect::<Vec<String>>();
        let arg = (craft.recipe.progress as f32 - craft.progression as f32) / craft.get_base_progression() as f32;
        if 0.0 < arg && arg < 1.2 { actions.push("basicSynth2".to_string()); }
        if 1.2 <= arg && arg < 1.8 { actions.push("carefulSynthesis".to_string()); }
        if 1.8 <= arg && arg < 2.0 {
            actions.push("observe".to_string());
            actions.push("focusedSynthesis".to_string());
        }
        SolverResult{
            steps:craft.step_count,
            progression: craft.progression,
            quality:craft.quality,
            durability:craft.durability,
            actions:actions,
            step1_solutions,
            step2_solutions,
            found_100_percent,
        }

    }
    pub fn default()->Self{
        Self{
            steps:0,
            progression: 0,
            quality:0,
            durability:0,
            actions:vec!["Act1".to_string(),"Act2".to_string()],
            step1_solutions:0,
            step2_solutions:0,
            found_100_percent:false,
        }
    }
}

/// Python Bindings

/// A Python module implemented in Rust.
#[cfg(not(feature="no_python"))]
#[pymodule]
fn xiv_craft_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pouet, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn pouet()-> SolverResult {
    SolverResult::default()
}