use fxhash::FxHashSet;

pub struct ScannerReading {
    pub beacons: Vec<[i32; 3]>,
    pub beacon_distance_set: FxHashSet<usize>,
}