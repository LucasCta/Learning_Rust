/* IMPERATIVE VERSION */

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut max: u64 = 0;
    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            sum += value;
        } println!("Group has: {sum}");
        if sum > max {
            max = sum;
        }
    } println!("The burdendst elf is carrying {max} calories");
    Ok(())
} 