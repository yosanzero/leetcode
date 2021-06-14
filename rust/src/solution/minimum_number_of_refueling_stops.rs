use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut location = 0;
        let mut car_fuel = start_fuel;
        let mut stop = 0;
        let mut curr_station_index = 0;

        let mut station_fuels = BinaryHeap::new();

        while location + car_fuel < target {
            location += car_fuel;

            while curr_station_index < stations.len() && stations[curr_station_index][0] <= location {
                station_fuels.push(stations[curr_station_index][1]);
                curr_station_index += 1;
            }

            car_fuel = match station_fuels.pop() {
                Some(last_station_fuel) => last_station_fuel,
                None => return -1
            };

            stop += 1;
        }
        stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::min_refuel_stops(1000, 10, vec![vec![3,133],vec![5,418],vec![11,202],vec![12,381],vec![29,14],vec![43,423],vec![56,299],vec![60,353],vec![64,267],vec![66,224],vec![68,248],vec![76,106],vec![78,121],vec![81,410],vec![92,486],vec![105,311],vec![107,354],vec![111,461],vec![119,481],vec![134,328],vec![142,485],vec![151,139],vec![178,318],vec![179,339],vec![184,268],vec![223,344],vec![224,258],vec![232,484],vec![262,487],vec![287,272],vec![288,444],vec![298,174],vec![299,409],vec![302,80],vec![305,240],vec![308,199],vec![324,298],vec![335,104],vec![345,349],vec![352,359],vec![390,249],vec![391,113],vec![395,380],vec![407,411],vec![408,302],vec![410,463],vec![415,43],vec![432,46],vec![441,197],vec![447,401],vec![452,157],vec![456,306],vec![459,303],vec![469,155],vec![471,260],vec![482,255],vec![489,312],vec![491,455],vec![494,243],vec![499,120],vec![506,228],vec![563,103],vec![568,251],vec![569,114],vec![592,33],vec![600,293],vec![609,21],vec![627,279],vec![629,260],vec![635,210],vec![649,56],vec![663,403],vec![665,124],vec![695,200],vec![708,209],vec![712,210],vec![719,216],vec![743,396],vec![744,62],vec![746,180],vec![777,159],vec![778,346],vec![779,450],vec![782,211],vec![796,220],vec![837,130],vec![847,238],vec![853,92],vec![857,353],vec![866,238],vec![885,436],vec![887,278],vec![906,11],vec![912,387],vec![920,241],vec![939,363],vec![946,92],vec![955,465],vec![969,245],vec![980,308]]));
        assert_eq!(1, Solution::min_refuel_stops(100, 50, vec![vec![25,50],vec![50,25]]));
        assert_eq!(0, Solution::min_refuel_stops(1, 1, vec![]));
        assert_eq!(-1, Solution::min_refuel_stops(100, 1, vec![vec![10,100]]));
        assert_eq!(2, Solution::min_refuel_stops(100, 10, vec![vec![10,60],vec![20,30],vec![30,30],vec![60,40]]));
        assert_eq!(3, Solution::min_refuel_stops(100, 25, vec![vec![25,25],vec![50,25],vec![75,25]]));
    }
}
