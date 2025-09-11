mod aocclient;
mod autosubmit;
mod day01;
mod day02;
mod solver;

use std::env;

use clap::Parser;

use solver::Solver;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    submit: bool,

    #[arg(short, long)]
    cookie: Option<String>,

    #[arg(short, long)]
    part_two_only: bool,

    day: i8,
}

fn solver_for_day(day: i8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day01::solver())),
        2 => Some(Box::new(day02::solver())),
        3 => Some(Box::new(day02::solver())),
        _ => None,
    }
}

fn main() {
    pretty_env_logger::init();
    log::info!("Advent of Code 2018 Solver");

    let args = Args::parse();

    let client = aocclient::AocClient::new().expect("creating AoC client");

    // Get the AoC cookie, either from the command line, or from the env variable.
    if let Some(cookie) = args.cookie {
        client.set_cookie(cookie.as_str());
    } else if let Ok(cookie) = env::var("AOC_COOKIE") {
        client.set_cookie(&cookie);
    } else {
        log::warn!("you must specify the session cookie with --cookie or AOC_COOKIE env variable");
        return;
    }
    let solver = solver_for_day(args.day);
    if solver.is_none() {
        log::error!("this solver cannot solve day {}", args.day);
        return;
    }
    let mut solver = solver.unwrap();
    let solver = solver.as_mut();
    log::info!("solving Advent of Code day {}", args.day);
    log::info!("retrieving puzzle input...");
    match client.get_puzzle_input(args.day) {
        Ok(input) => {
            solver.presolve(input.as_str());
            if !args.part_two_only {
                log::info!("solving part one...");
                let answer = solver.solve_part_one();
                log::info!("part one answer: {answer}");
                if args.submit {
                    log::info!("submitting part one...");
                    let result =
                        autosubmit::submit_with_cache(args.day, 1, answer.as_str(), |d, l, a| {
                            client.submit_answer(d, l, a).unwrap()
                        });
                    log::info!("part one submission result: {result:?}");
                }
            }
            log::info!("solving part two...");
            let answer = solver.solve_part_two();
            log::info!("part two answer: {answer}");
            if args.submit {
                log::info!("submitting part two...");
                let result =
                    autosubmit::submit_with_cache(args.day, 2, answer.as_str(), |d, l, a| {
                        client.submit_answer(d, l, a).unwrap()
                    });
                log::info!("part one submission result: {result:?}");
            }
        }
        Err(e) => {
            log::error!("error retrieving puzzle input: {e:#?}");
        }
    }
}
