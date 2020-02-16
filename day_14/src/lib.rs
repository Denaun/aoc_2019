use std::collections::HashMap;
use std::convert::Infallible;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Chemical {
    Ore,
    Fuel,
    Other(String),
}

impl FromStr for Chemical {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ORE" => Ok(Chemical::Ore),
            "FUEL" => Ok(Chemical::Fuel),
            _ => Ok(Chemical::Other(s.to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Reaction {
    pub inputs: HashMap<Chemical, usize>,
    pub output: usize,
}

trait AddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

impl<K, V, S> AddOrInsert<K, V> for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: AddAssign + Default,
    S: BuildHasher,
{
    fn add_or_insert(self: &mut Self, k: K, v: V) {
        *self.entry(k).or_default() += v;
    }
}

pub fn solve_for<S1: BuildHasher, S2: BuildHasher>(
    reactions: &HashMap<Chemical, Reaction, S1>,
    chemical: &Chemical,
    quantity: usize,
    mut leftovers: HashMap<Chemical, usize, S2>,
) -> (HashMap<Chemical, usize>, HashMap<Chemical, usize, S2>) {
    if quantity == 0 {
        return (HashMap::new(), leftovers);
    }
    let already_available = *leftovers.get(chemical).unwrap_or(&0);
    if already_available >= quantity {
        // Use part of the leftovers and don't run any reaction.
        *leftovers.get_mut(&chemical).unwrap() -= quantity;
        return (HashMap::new(), leftovers);
    }
    leftovers.remove(&chemical); // We are going to consume all the leftovers.
    let quantity = quantity - already_available;
    let reaction = &reactions[chemical];
    let ratio = (reaction.output + quantity - 1) / reaction.output;
    let produced = reaction.output * ratio;
    if produced > quantity {
        leftovers.insert(chemical.clone(), produced - quantity);
    }
    let mut result = HashMap::new();
    for (in_chemical, in_quantity) in reaction.inputs.iter() {
        if reactions.contains_key(in_chemical) {
            let (child_result, new_leftovers) =
                solve_for(reactions, in_chemical, in_quantity * ratio, leftovers);
            for (c, q) in child_result {
                result.add_or_insert(c, q);
            }
            leftovers = new_leftovers;
        } else {
            result.add_or_insert(in_chemical.clone(), in_quantity * ratio);
        }
    }
    (result, leftovers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_input(data: &str) -> HashMap<Chemical, Reaction> {
        let read_component = |s: &str| -> (Chemical, usize) {
            let mut parts = s.split(' ');
            let quantity = parts.next().unwrap();
            let chemical = parts.next().unwrap();
            assert_eq!(parts.next(), None);
            (chemical.parse().unwrap(), quantity.parse().unwrap())
        };
        data.lines()
            .map(|line| {
                let mut parts = line.split(" => ");
                let inputs = parts.next().unwrap();
                let output = parts.next().unwrap();
                assert_eq!(parts.next(), None);
                let (out_chemical, out_quantity) = read_component(output);
                (
                    out_chemical,
                    Reaction {
                        inputs: inputs.split(", ").map(read_component).collect(),
                        output: out_quantity,
                    },
                )
            })
            .collect()
    }

    #[test]
    fn example_1() {
        let reactions = read_input(
            "10 ORE => 10 A\n\
             1 ORE => 1 B\n\
             7 A, 1 B => 1 C\n\
             7 A, 1 C => 1 D\n\
             7 A, 1 D => 1 E\n\
             7 A, 1 E => 1 FUEL",
        );
        let (requirements, leftovers) = solve_for(&reactions, &Chemical::Fuel, 1, HashMap::new());
        assert_eq!(
            requirements,
            [(Chemical::Ore, 31)].iter().cloned().collect()
        );
        assert_eq!(
            leftovers,
            [(Chemical::Other("A".to_owned()), 2)]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn example_2() {
        let reactions = read_input(
            "9 ORE => 2 A\n\
             8 ORE => 3 B\n\
             7 ORE => 5 C\n\
             3 A, 4 B => 1 AB\n\
             5 B, 7 C => 1 BC\n\
             4 C, 1 A => 1 CA\n\
             2 AB, 3 BC, 4 CA => 1 FUEL",
        );
        assert_eq!(
            solve_for(&reactions, &Chemical::Fuel, 1, HashMap::new()).0,
            [(Chemical::Ore, 165)].iter().cloned().collect()
        );
    }

    #[test]
    fn example_3() {
        let reactions = read_input(
            "157 ORE => 5 NZVS\n\
             165 ORE => 6 DCFZ\n\
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
             179 ORE => 7 PSHF\n\
             177 ORE => 5 HKGWZ\n\
             7 DCFZ, 7 PSHF => 2 XJWVT\n\
             165 ORE => 2 GPVTF\n\
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );
        assert_eq!(
            solve_for(&reactions, &Chemical::Fuel, 1, HashMap::new()).0,
            [(Chemical::Ore, 13312)].iter().cloned().collect()
        );
    }

    #[test]
    fn example_4() {
        let reactions = read_input(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
             17 NVRVD, 3 JNWZP => 8 VPVL\n\
             53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
             22 VJHF, 37 MNCFX => 5 FWMGM\n\
             139 ORE => 4 NVRVD\n\
             144 ORE => 7 JNWZP\n\
             5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
             5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
             145 ORE => 6 MNCFX\n\
             1 NVRVD => 8 CXFTF\n\
             1 VJHF, 6 MNCFX => 4 RFSQX\n\
             176 ORE => 6 VJHF",
        );
        assert_eq!(
            solve_for(&reactions, &Chemical::Fuel, 1, HashMap::new()).0,
            [(Chemical::Ore, 180_697)].iter().cloned().collect()
        );
    }

    #[test]
    fn example_5() {
        let reactions = read_input(
            "171 ORE => 8 CNZTR\n\
             7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
             114 ORE => 4 BHXH\n\
             14 VRPVC => 6 BMBT\n\
             6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
             6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
             15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
             13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
             5 BMBT => 4 WPTQ\n\
             189 ORE => 9 KTJDG\n\
             1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
             12 VRPVC, 27 CNZTR => 2 XDBXC\n\
             15 KTJDG, 12 BHXH => 5 XCVML\n\
             3 BHXH, 2 VRPVC => 7 MZWV\n\
             121 ORE => 7 VRPVC\n\
             7 XCVML => 6 RJRHP\n\
             5 BHXH, 4 VRPVC => 5 LTCX",
        );
        assert_eq!(
            solve_for(&reactions, &Chemical::Fuel, 1, HashMap::new()).0,
            [(Chemical::Ore, 2_210_736)].iter().cloned().collect()
        );
    }

    #[test]
    fn day_14_part_1() {
        let reactions = read_input(include_str!("input"));
        assert_eq!(
            solve_for(&reactions, &Chemical::Fuel, 1, HashMap::new()).0,
            [(Chemical::Ore, 114_125)].iter().cloned().collect()
        );
    }
}
