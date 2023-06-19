
use nonogram::Nonogram;
use nonogram::init_nonogram_from_file;

use varisat::solver::Solver;
use varisat::{ExtendFormula};
use varisat::Lit;

pub mod nonogram;

fn main() {
    let filename = String::from("puzzles/easy0.non");
    if let Some(mut puzzle) = init_nonogram_from_file(&filename) {

        // The SAT solver
        let mut solver: Solver<'_> = Solver::new();

        // sat has the literals in the shape of the board to make it easier to think about
        let mut sat : Vec<Vec<Lit>> = Vec::new();
        for _x in 0..puzzle.size {
            let mut row : Vec<Lit> = Vec::new();
            for _y in 0..puzzle.size {
                row.push(solver.new_lit());
            }
            sat.push(row);
        }

        // Just a list of 1..puzzle size
        // Each number is a literal sat variable
        let list_left: Vec<i32> = (1..=puzzle.size.try_into().unwrap()).collect();

        // Generates every possible literal combination for a row/col
        // Basically all bitstrings
        // 2^puzzlesize vectors in all
        let all = make_all_combos(Vec::new(), &list_left[..]);

        add_row_rules(&puzzle, &mut solver, &sat, &all);
        add_col_rules(&puzzle, &mut solver, &sat, &all);
        
        // Do the solution!
        let solution = solver.solve().unwrap();

        // Hopefully it went well
        assert_eq!(solution, true);

        // Translate the solution model given by the sat solver back into the Nonogram puzzle
        let model = solver.model().unwrap();
        println!("Model: {:?}", model);
        let mut counter = 0;
        for i in 0..puzzle.size {
            for j in 0..puzzle.size {
                if model[counter].is_positive() {
                    puzzle.set(i, j, true);
                } else {
                    puzzle.set(i, j, false);
                }
                counter += 1;
            }
        }

        // Is it a correct solution?
        let valid : bool = puzzle.validate(); 
        println!("Nonogram valid: {:?}", valid);

        puzzle.print();
    }
}

fn add_row_rules(puzzle: &Nonogram, solver: &mut Solver, sat : &[Vec<Lit>], all_combos: &[Vec<i32>]) {
    for i in 0..puzzle.size {
        // Pull out the correct literals for the row
        let mut vars: Vec<i32> = Vec::new();
        for j in 0..puzzle.size {
            vars.push(sat[i][j].to_dimacs().try_into().unwrap())
        }
        
        // Add a rule to the solver for this row
        add_group_rule(solver, all_combos, &vars[..], &(puzzle.row_rules[i][..]));
    }
}

fn add_col_rules(puzzle: &Nonogram, solver: &mut Solver, sat : &[Vec<Lit>], all_combos: &[Vec<i32>]) {
    for i in 0..puzzle.size {
        let mut vars: Vec<i32> = Vec::new();
        for j in 0..puzzle.size {
            vars.push(sat[j][i].to_dimacs().try_into().unwrap())
        }
        add_group_rule(solver, all_combos, &vars[..], &(puzzle.col_rules[i][..]));
    }
}

// Work with a column or a row rule
fn add_group_rule(solver: &mut Solver, all_combos: &[Vec<i32>], vars: &[i32], sets: &[usize]) {

    // Get a CNF clause representing the rule
    let cnf = make_group_cnf(all_combos, vars.len(), sets);

    // Translate the clause into the correct representation for the SAT solver
    for generic_clause in cnf.iter() {
        let mut f: Vec<Lit> = Vec::new();
        for i in 0..vars.len() {
            let num = (vars[i]).try_into().unwrap();
            let literal = Lit::from_dimacs(num);

            if generic_clause[i] > 0 {
                f.push(literal);
            } else {
                f.push(!literal);
            }
        }
        solver.add_clause(&f);
    }
}

// Negate all literals
fn negate(mut vars: Vec<i32>) -> Vec<i32> {
    for i in vars.iter_mut() {
        *i = -(*i);
    }
    vars
}

// Make a SAT rule in CNF that represents the nonogram's set row/column rule
// CNF: A conjunction of disjunction of literals 
// ex. (x || !y || z) && (a || b)
fn make_group_cnf(all_combos: &[Vec<i32>], size: usize, sets: &[usize]) -> Vec<Vec<i32>>{
    // Make all the literals
    let vars: Vec<i32> = (1..=size.try_into().unwrap()).collect();

    // Make every combination of literals that follows the set rule
    let possible = make_possible_combos( 0, vars, sets);
    let mut cnf : Vec<Vec<i32>> = Vec::new();

    // Go through all combinations these variables (the entire truth table)
    // Look for combinations that are NOT possble by the ruleset
    // Negate them and add them to the CNF clause
    // Basically - make_possible_combos represents Disjunctive Normal Form
    // (a && !b && c) || (!a && b && c)
    // But we need CNF
    // So we leverage Demorgan's Law get the equivalent CNF
    for combo in all_combos.iter() {
        if !possible.contains(combo) {
            cnf.push(negate(combo.clone()));
        }
    }

    cnf
}


// Find every possible combination of literals that satisfies the given ruleset
fn make_possible_combos(search_start_index: usize, vars : Vec<i32> , sets: &[usize]) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    if sets.is_empty() {
        // no more sets to place - we're done!
        result.push(vars);
        return result;
    }

    // Pull out the size of the first set
    let set_size = sets[0];

    // This is the last index where this set could start and still fit in the amount of vars
    // Optimization - we could prune this further by reasoning about how many sets are left
    // But this isn't the bottleneck of the program so not going to spend time on it
    let search_end_index = vars.len() - set_size+1 ;

    // Create a branch for every possible place the set could start
    // It's possible that search_start_index >= (vars.len() - set_size+1), in which case the range will be empty and this possibility will be pruned
    // Means that the set cannot fit in the amount of space left, so return an empty set
    for this_start_index in search_start_index..search_end_index {
        // Create a copy for this branch 
        let mut vars_copy = vars.clone();

        // Fill in empty space from search start to before the start of this set
        for j in search_start_index..this_start_index {
            vars_copy[j] = -(vars_copy[j].abs());
        }

        // Fill in filled space for this set
        for j in this_start_index..this_start_index+set_size {
            vars_copy[j] = vars_copy[j].abs();
        }

        // Fill in empty space for the rest of the group
        for j in this_start_index+set_size..vars.len() {
            vars_copy[j] = -(vars_copy[j].abs());
        }

        // Move on to the next set
        let next_set_start_index = this_start_index+set_size+1;
        let next_sets = &sets[1..];
        result.append (& mut make_possible_combos(next_set_start_index, vars_copy, next_sets));
    }

    result
}

// Generate all the possible literal solutions
// Treat each literal as bit (on = positive, off= negative), generate all possible combinations
fn make_all_combos(current_list : Vec<i32> , list_left: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    if list_left.is_empty() {
        // Nothing left - we're done!
        result.push(current_list);
        return result;
    }

    let mut vec_pos = current_list.clone();
    let mut vec_neg = current_list;

    // Build a sequence for each branch - positive and negative
    vec_pos.push(list_left[0].abs());
    vec_neg.push(-list_left[0].abs());

    // Chope the list
    let list_left = &list_left[1..];

    // Recurse!
    result.append(&mut make_all_combos(vec_pos, list_left));
    result.append(&mut make_all_combos(vec_neg, list_left));

    // We done did recursed
    // Result now has all of the full sequences that begin with vec_pos and vec_neg
    return result;
}