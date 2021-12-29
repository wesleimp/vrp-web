use actix_web::web;
use serde::Deserialize;
use std::sync::Arc;
use vrp_cli::extensions::solve::config::create_builder_from_config;
use vrp_cli::pragmatic::format::problem::*;
use vrp_cli::pragmatic::format::solution::*;

use vrp_cli::extensions::solve::config::Config as CliConfig;

#[derive(Clone, Deserialize, Debug)]
pub struct ApiProblem {
    pub plan: Plan,
    pub fleet: Fleet,
    pub objectives: Option<Vec<Vec<Objective>>>,
}

pub async fn solution(api_problem: web::Json<ApiProblem>) -> web::Json<Solution> {
    let problem = Arc::new(
        Problem {
            plan: api_problem.plan.clone(),
            fleet: api_problem.fleet.clone(),
            objectives: api_problem.objectives.clone(),
        }
        .read_pragmatic()
        .unwrap(),
    );

    let cli_config = CliConfig::default();

    let (solution, _, metrics) = create_builder_from_config(problem.clone(), &cli_config)
        .and_then(|builder| builder.build())
        .and_then(|solver| solver.solve())
        .unwrap();

    let api_solution = create_solution(problem.as_ref(), &solution, metrics.as_ref());

    web::Json(api_solution)
}
