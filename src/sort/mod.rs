pub mod sort {
    use std::collections::{HashMap, BTreeMap};
    use rayon::prelude::*;
    use std::sync::{Arc, Mutex};

    // occurence_count_parallel with rayon and Arc and Mutex
    fn occurence_count_parallel(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, usize>>) -> HashMap<usize, usize> {
        let kmer_occurence: Arc<Mutex<HashMap<usize, usize>>> = Arc::new(Mutex::new(HashMap::new()));

        kmer_count.par_iter().for_each(|(_, value)| {
            value.iter().for_each(|(_, count)| {
                let mut kmer_occurence = kmer_occurence.lock().unwrap();
                if let Some(occurence) = kmer_occurence.get_mut(count) {
                    *occurence += 1;
                } else {
                    kmer_occurence.insert(*count, 1);
                }
            });
        });

        Arc::try_unwrap(kmer_occurence)
            .unwrap()
            .into_inner()
    .unwrap()
    }

    // sort hashmap by key, returns a vector of sorted values
    fn sort_by_coverage(kmer_occurence: HashMap<usize, usize>) -> BTreeMap<usize, usize> {
        let mut coverage: BTreeMap<usize, usize> = BTreeMap::new();
        for (key, value) in kmer_occurence.iter() {
            if let Some(coverage_value) = coverage.get_mut(value) {
                *coverage_value += (*key) * (*value);
            } else {
                coverage.insert(*value, (*key) * (*value));
            }
        }
        coverage
    }

    pub fn sort_by_coverage_wrap(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, usize>>) -> Vec<usize> {
        let kmer_occurence = occurence_count_parallel(kmer_count);
        //println!("{:?}", kmer_occurence);
        //println!("coverage_per_occurrence {:?}", coverage_per_occurrence);
        let coverage_values = sort_by_coverage(kmer_occurence);
        // println!("{:?}", coverage_values);
        //convert BTreeMap to HashMap
        let mut final_values: Vec<usize> = Vec::new();
        for (key, value) in coverage_values.iter() {
            for _ in 0..*key {
                final_values.push(*value / *key);
            }
        }
        final_values // Sorted values by occurrence (x-axis is k-mer occurrence, y-axis is number of k-mers)
    }
}
