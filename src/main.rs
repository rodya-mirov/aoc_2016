use std::env;

mod day_01;

fn main() -> Result<(), ()> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: [run command] day");
        println!("  Example: cargo run --release -- 12b");
        return Err(());
    }

    let start = std::time::SystemTime::now();

    let answer = match args.nth(1).unwrap().as_str() {
        "1a" => day_01::day_1a(),
        "1b" => day_01::day_1b(),
        other => {
            println!("Unknown day variant {:?}", other);
            return Err(());
        }
    };

    let elapsed = start.elapsed().unwrap();

    println!("Answer: {}", answer);
    println!("Elapsed: {:.5} seconds", elapsed.as_secs_f32());

    Ok(())
}
