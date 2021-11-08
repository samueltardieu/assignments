use anyhow::Result;
use pathfinding::prelude::{kuhn_munkres, Matrix};
use std::collections::{BTreeMap, BTreeSet, HashSet};

type Choices = Vec<(String, Vec<usize>)>;

fn load_csv_file(path: &str) -> Result<Choices> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;
    let mut rows = Choices::new();
    for result in rdr.records() {
        let record = result?;
        let mut record = record.into_iter();
        let student = record
            .next()
            .ok_or_else(|| anyhow::anyhow!("No student name"))?;
        let choices = record
            .map(|choice| choice.parse())
            .collect::<Result<Vec<_>, _>>()?;
        rows.push((student.to_owned(), choices));
    }
    Ok(rows)
}

fn check_prefs(prefs: &Choices, num_choices: usize) -> Result<()> {
    let acceptable = 1..=num_choices;
    let mut seen_students = HashSet::new();
    for (student, choices) in prefs {
        if seen_students.contains(&student) {
            return Err(anyhow::anyhow!("Duplicate student {}", student));
        }
        seen_students.insert(student);
        let mut seen_choices = BTreeSet::new();
        for choice in choices {
            if !acceptable.contains(choice) {
                return Err(anyhow::anyhow!(
                    "{} has made an unacceptable choice: {} (not in [1..{}])",
                    student,
                    choice,
                    num_choices,
                ));
            }
            if seen_choices.contains(choice) {
                return Err(anyhow::anyhow!(
                    "{} has a duplicate choice: {}",
                    student,
                    choice,
                ));
            }
            seen_choices.insert(*choice);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let app = clap::App::new("assignments")
    .about("Assign student to choices by maximizing the global satisfaction")
    .author("Samuel Tardieu <sam@rfc1149.net>")
    .after_help(concat!("The CSV file must contain no header and have the student name followed by the choices (1..n).\n",
      "Cost model is ((rank-1)*mult)^power. Unranked papers have rank n+1."))
    .args_from_usage(
        r#"
      -v                     'Be verbose'
      -m, --mult=[coeff]     'Multiplicative coefficient for rank [default: 1]'
      -p, --power=[coeff]    'Power coefficient for rank [default: 2]'
      -n, --num-choices=[n]  'Number of choices [default: the number of students]'
      <INPUT>                'CSV file'
    "#,
    );
    let matches = app.get_matches();
    let verbose = matches.is_present("v");
    let m: isize = matches.value_of("mult").unwrap_or("4").parse()?;
    let p: u32 = matches.value_of("power").unwrap_or("1").parse()?;
    let prefs = load_csv_file(matches.value_of("INPUT").unwrap())?;
    let num_choices = matches
        .value_of("num-choices")
        .map(|n| n.parse())
        .transpose()?
        .unwrap_or(prefs.len());
    check_prefs(&prefs, num_choices)?;
    let mut weights = Matrix::new(prefs.len(), num_choices, 0);
    for (i, (_, choices)) in prefs.iter().enumerate() {
        for (r, choice) in choices.iter().enumerate() {
            weights[&(i, *choice - 1)] = ((num_choices - r) as isize * m).pow(p);
        }
    }
    let (total_cost, assignments) = kuhn_munkres(&weights);
    let mut stats = BTreeMap::new();
    let mut unranked = 0;
    for ((student, choices), assignment) in prefs.iter().zip(assignments) {
        let assignment = assignment + 1;
        if verbose {
            println!(
                "{} -> {} ({})",
                student,
                assignment,
                choices
                    .iter()
                    .position(|&c| c == assignment)
                    .map(|r| {
                        *stats.entry(r + 1).or_insert(0) += 1;
                        format!("choice ranked {}", r + 1)
                    })
                    .unwrap_or_else(|| {
                        unranked += 1;
                        String::from("unranked")
                    })
            );
        } else {
            println!("{} -> {}", student, assignment);
        }
    }
    if verbose {
        println!("\nTotal satisfaction: {}\nRanks:", total_cost);
        for (rank, count) in stats {
            println!("  - rank {}: {}", rank, count);
        }
        if unranked != 0 {
            println!("  - unranked: {}", unranked);
        }
    }
    Ok(())
}
