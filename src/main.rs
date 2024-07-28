fn main() {

    // Generates a random dice roll
    fn rand_int_gen() -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(1..7);
        return num;
    }
    // Loops through each person to simulate each person's roll and update the population accordingly
    fn round(curr_pop:i32) -> i32{
        let mut pop = curr_pop;
        let mut neo_pop:i32 = pop;
        for _ in 0..pop {
            let roll = rand_int_gen();
            if roll == 6{
                neo_pop -= 1;
            }
        }
        return neo_pop;
    }

    fn sim(trials:i32, populat:i32) -> Vec<i32>{
        let mut pop = populat;
        let mut round_num = Vec::new();
        let mut count:i32;
        for _ in 0..trials {
            count = 0;
            pop = populat;
            while pop > 1 {
                pop = round(pop);
                count += 1;
            }
            round_num.push(count);
        }
        return round_num;
    }

    fn format_output(sim_out:Vec<i32>) -> std::collections::HashMap<i32, i32>{
        use std::collections::HashMap;
        let mut data_map = HashMap::new();
        for &i in sim_out.iter() { // Use `&i` to get the value
        let count = data_map.entry(i).or_insert(0); // Use entry API
        *count += 1;
    }
        return data_map;
    }

    use plotters::prelude::*;
    use std::collections::HashMap;

    fn graph_hashmap(data: HashMap<i32, i32>) -> Result<(), Box<dyn std::error::Error>> {
            // Calculate the bounds
        let (x_min, x_max) = data.keys().fold((i32::MAX, i32::MIN), |(min, max), &x| (min.min(x), max.max(x)));
        let (y_min, y_max) = data.values().fold((i32::MAX, i32::MIN), |(min, max), &y| (min.min(y), max.max(y)));

        // Create the drawing area
        let root_area = BitMapBackend::new("graph.png", (1280, 960)).into_drawing_area();
        root_area.fill(&WHITE)?;

        // Build the chart
        let mut chart = ChartBuilder::on(&root_area)
            .caption("Dice Simulation", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d((x_min - 1)..(x_max + 1), (y_min - 1)..(y_max + 1))?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            data.iter().map(|(&x, &y)| {
                Circle::new((x, y), 5, ShapeStyle {
                    color: BLUE.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                })
            })
        )?;

        root_area.present()?;
        Ok(())
    }

    fn userInput(message: &str) -> i32 {
        use std::io::{stdin,stdout,Write};
        let mut s=String::new();
        print!("{}: ", message);
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        return s.parse::<i32>().unwrap();
    }


    let sim_out = format_output(sim(
        userInput("Trials: "),
        userInput("Population: ")));
    graph_hashmap(sim_out);
}
