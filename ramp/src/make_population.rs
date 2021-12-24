use std::collections::HashMap;
use std::fs::File;

use anyhow::Result;
use serde::Deserialize;

use crate::population::{Activity, Household, HouseholdID, Person, PersonID, Population, VenueID};
use crate::quant::{load_venues, quant_get_flows, Threshold};
use crate::utilities::print_count;
use crate::MSOA;

// population_initialisation.py
pub fn initialize() -> Result<Population> {
    let mut population = Population {
        households: Vec::new(),
        people: Vec::new(),
        venues_per_activity: HashMap::new(),
    };
    read_individual_time_use_and_health_data(&mut population)?;

    // pshop
    setup_venue_flows(Activity::Retail, Threshold::TopN(10), &mut population)?;
    // pleisure
    setup_venue_flows(Activity::Nightclub, Threshold::TopN(10), &mut population)?;
    // pschool-primary
    setup_venue_flows(Activity::PrimarySchool, Threshold::TopN(5), &mut population)?;
    // pschool-secondary
    setup_venue_flows(
        Activity::SecondarySchool,
        Threshold::TopN(5),
        &mut population,
    )?;

    // TODO Commuting

    // TODO Lots of commented stuff, then rounding

    Ok(population)
}

fn read_individual_time_use_and_health_data(population: &mut Population) -> Result<()> {
    let mut households: Vec<Household> = Vec::new();
    let mut people: Vec<Person> = Vec::new();
    let mut household_lookup: HashMap<(MSOA, isize), HouseholdID> = HashMap::new();

    let mut no_household = 0;
    // TODO Read from the combined TU file, not this hardcoded thing
    for rec in
        csv::Reader::from_reader(File::open("raw_data/tus_hse_west-yorkshire.csv")?).deserialize()
    {
        // TODO Progress bar
        if people.len() % 1000 == 0 {
            info!("{} people so far", print_count(people.len()));
            /*if !people.is_empty() {
                break;
            }*/
        }

        let rec: TuPerson = rec?;
        // Strip out people that weren't matched to a household
        // No such examples in this file:
        // > xsv search -s hid '\-1' county_data/tus_hse_west-yorkshire.csv
        if rec.hid == -1 {
            no_household += 1;
            continue;
        }

        let household_id = household_lookup
            .entry((rec.msoa.clone(), rec.hid))
            .or_insert_with(|| {
                let id = HouseholdID(households.len());
                households.push(Household {
                    id,
                    msoa: rec.msoa,
                    orig_hid: rec.hid,
                    members: Vec::new(),

                    disease_danger: 0.0,
                });
                id
            });
        let household = &mut households[household_id.0];
        let person_id = PersonID(people.len());
        household.members.push(person_id);

        let mut duration_per_activity: HashMap<Activity, f64> = HashMap::new();
        duration_per_activity.insert(Activity::Retail, rec.pshop);
        duration_per_activity.insert(Activity::Home, rec.phome);
        duration_per_activity.insert(Activity::Work, rec.pwork);
        duration_per_activity.insert(Activity::Nightclub, rec.pleisure);

        // Use pschool and age to calculate primary/secondary school
        if rec.age < 11 {
            duration_per_activity.insert(Activity::PrimarySchool, rec.pschool);
            duration_per_activity.insert(Activity::SecondarySchool, 0.0);
        } else if rec.age < 19 {
            duration_per_activity.insert(Activity::PrimarySchool, 0.0);
            duration_per_activity.insert(Activity::SecondarySchool, rec.pschool);
        } else {
            // TODO Seems like we need a University activity
            duration_per_activity.insert(Activity::PrimarySchool, 0.0);
            duration_per_activity.insert(Activity::SecondarySchool, 0.0);
        }
        pad_durations(&mut duration_per_activity)?;

        people.push(Person {
            id: person_id,
            household: household.id,
            orig_pid: rec.pid,

            age_years: rec.age,

            flows_per_activity: HashMap::new(),
            duration_per_activity,
        });
    }
    if no_household > 0 {
        warn!(
            "{} people skipped, no household originally",
            print_count(no_household)
        );
    }

    // TODO Strip out households with >10 people and fix up all the IDs

    population.households = households;
    population.people = people;
    info!(
        "{} people across {} households, and {} MSOAs",
        print_count(population.people.len()),
        print_count(population.households.len()),
        print_count(population.unique_msoas().len())
    );
    Ok(())
}

#[derive(Deserialize)]
struct TuPerson {
    #[serde(rename = "MSOA11CD")]
    msoa: MSOA,
    hid: isize,
    pid: isize,

    phome: f64,
    pwork: f64,
    pleisure: f64,
    pshop: f64,
    pschool: f64,
    age: usize,
}

fn setup_venue_flows(
    activity: Activity,
    threshold: Threshold,
    population: &mut Population,
) -> Result<()> {
    info!("Reading {:?} flow data...", activity);

    population
        .venues_per_activity
        .insert(activity, load_venues(activity)?);

    // Per MSOA, a list of venues and the probability of going from the MSOA to that venue
    let flows_per_msoa: HashMap<MSOA, Vec<(VenueID, f64)>> =
        quant_get_flows(activity, population.unique_msoas(), threshold)?;

    // Now let's assign these flows to the people. Near as I can tell, this just copies the flows
    // to every person in the MSOA. That's loads of duplication -- we could just keep it by (MSOA x
    // activity), but let's follow the Python for now.
    info!("Copying {:?} flows to the people", activity);
    for person in &mut population.people {
        let msoa = &population.households[person.household.0].msoa;
        if let Some(flows) = flows_per_msoa.get(msoa) {
            person.flows_per_activity.insert(activity, flows.clone());
        } else {
            // TODO I think this is an error; not happening for the small input
            warn!("No flows for {:?} in {}", activity, msoa.0);
        }
    }

    Ok(())
}

// If the durations don't sum to 1, pad Home
fn pad_durations(durations: &mut HashMap<Activity, f64>) -> Result<()> {
    let total: f64 = durations.values().sum();
    // TODO Check the rounding in the Python version
    let epsilon = 0.00001;
    if total > 1.0 + epsilon {
        bail!("Someone's durations sum to {}", total);
    } else if total < 1.0 {
        durations.insert(Activity::Home, 1.0 - total);
    }
    Ok(())
}
