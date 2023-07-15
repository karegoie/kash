//sort HashMap<&[u8], HashMap<&[u8], u16>> by occurrence of k-mer
pub mod sort {
    use std::collections::HashMap;
    use rayon::prelude::*;
    fn occurence_count(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, u16>>) -> HashMap<u16, usize> {
        let mut kmer_occurence: HashMap<u16, usize> = HashMap::new();
        for (_, value) in kmer_count.iter() {
            for (_, count) in value.iter() {
                let count = *count;
                if let Some(occurence) = kmer_occurence.get_mut(&count) {
                    *occurence += 1;
                } else {
                    kmer_occurence.insert(count, 1);
                }
            }
        }
        kmer_occurence
    }

    // occurence_count_parallel with rayon
    fn occurence_count_parallel(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, u16>>) -> HashMap<u16, usize> {
        let mut kmer_occurence: HashMap<u16, usize> = HashMap::new();
        kmer_count.par_iter().for_each(|(_, value)| {
            value.par_iter().for_each(|(_, count)| {
                let count = *count;
                if let Some(occurence) = kmer_occurence.get_mut(&count) {
                    *occurence += 1;
                } else {
                    kmer_occurence.insert(count, 1);
                }
            });
        });
        kmer_occurence
    }

    // sort hashmap by key, returns a vector of sorted values
    fn sort_by_key(kmer_occurence: HashMap<u16, usize>) -> Vec<usize> {
        let mut sorted_keys: Vec<u16> = kmer_occurence.keys().cloned().collect();
        sorted_keys.sort();

        let mut sorted_values: Vec<usize> = Vec::new();
        for key in sorted_keys {
            if let Some(value) = kmer_occurence.get(&key) {
                sorted_values.push(*value);
            }
        }
        sorted_values
    }

    pub fn sort_by_occurrence(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, u16>>) -> Vec<usize> {
        let kmer_occurence = occurence_count(kmer_count);
        let sorted_values = sort_by_key(kmer_occurence);
        sorted_values // Sorted values by occurrence (x-axis is k-mer occurrence, y-axis is number of k-mers)
    }
}
