
fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {

    let mut changes = vec![0; 1001];

    for trip in trips.iter() {
        let passengers = trip[0];
        let from = trip[1] as usize;
        let to = trip[2] as usize;

        changes[from] += passengers;
        changes[to] -= passengers;
    }

    let mut current_passengers = 0;
    for change in changes {
        current_passengers += change;
        if current_passengers > capacity {
            return false;
        }
    }
    true 
}

fn main() {
    let trips1 = vec![vec![2, 1, 5], vec![3, 3, 7]];
    let capacity1 = 4;
    println!("{}", car_pooling(trips1, capacity1));
}
