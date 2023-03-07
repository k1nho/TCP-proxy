use std::sync::{Arc, Mutex};

/// Round Robin to balance the targets load
/// **targets** : The vector of targets to select from
/// **target_id** : A shared mutex guard state per app for each tcp port listener to participate in the round robin
///
/// # Return
/// A String representing the target server
pub fn round_robin(targets: &Vec<String>, target_id: &Arc<Mutex<usize>>) -> String {
    let mut id = target_id.lock().unwrap();
    *id += 1;

    targets[*id % targets.len()].clone()
}
