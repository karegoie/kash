use std::collections::HashMap;
use kodama::{Method, linkage, Dendrogram};
use serde_pickle;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

// differentiate the vector and return derivatives
pub fn differentiate(values: &Vec<usize>) -> Vec<i32> {
    let mut derivatives: Vec<i32> = Vec::new();
    for i in 0..values.len() - 1 {
        let derivative = values[i+1] as i32 - values[i] as i32;
        derivatives.push(derivative);
    }
    derivatives
}

// returns the index where derivative turns positive to negative
pub fn find_index(derivatives: Vec<i32>) -> Vec<usize> {
    let mut index = Vec::new();
    for i in 0..derivatives.len() - 1 {
        if derivatives[i] > 0 && derivatives[i+1] < 0 {
            index.push(i);
        }
    }
    index
}

pub fn get_mers_from_index(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, u16>>, index: u16) -> HashMap<Vec<u8>, Vec<Vec<u8>>> {
    let mut kmer_count_has_index: HashMap<Vec<u8>, Vec<Vec<u8>>> = HashMap::new();
    for (key, value) in kmer_count.iter() {
        for (kmer, count) in value.iter() {
            // if count equals to index, insert into kmer_count_has_index
            if *count == index + 1 {
                if let Some(kmer_count_has_index_value) = kmer_count_has_index.get_mut(key) {
                    kmer_count_has_index_value.push(kmer.to_vec());
                } else {
                    kmer_count_has_index.insert(key.to_vec(), vec![kmer.to_vec()]);
                }
            }
        }
    }
    kmer_count_has_index
}

pub fn get_mers_from_index_parallel(kmer_count: &HashMap<Vec<u8>, HashMap<Vec<u8>, usize>>, index: usize) -> HashMap<Vec<u8>, Vec<Vec<u8>>> {
    let kmer_count_has_index: Arc<Mutex<HashMap<Vec<u8>, Vec<Vec<u8>>>>> = Arc::new(Mutex::new(HashMap::new()));

    kmer_count.par_iter().for_each(|(key, value)| {
        value.iter().for_each(|(kmer, count)| {
            if *count == index {
                let mut kmer_count_has_index = kmer_count_has_index.lock().unwrap();
                if let Some(kmer_count_has_index_value) = kmer_count_has_index.get_mut(key) {
                    kmer_count_has_index_value.push(kmer.to_vec());
                } else {
                    kmer_count_has_index.insert(key.to_vec(), vec![kmer.to_vec()]);
                }
            }
        });
    });

    Arc::try_unwrap(kmer_count_has_index)
        .unwrap()
        .into_inner()
        .unwrap()
}

fn same_kmer_numer_as_distance(seq1: &Vec<Vec<u8>>, seq2: &Vec<Vec<u8>>) -> f32 {
    let mut distance = 0;
    for i in 0..seq1.len() {
        for j in 0..seq2.len() {
            if seq1[i] == seq2[j] {
                distance += 1;
            }
        }
    }
    if distance != 0 {
        ((1f32)/ (distance as f32 / (seq1.len() + seq2.len()) as f32)).log10()
    } else {
        (99999f32).log10()
    }
}

pub fn build_condensed_distance_matrix(target:&HashMap<Vec<u8>, Vec<Vec<u8>>>) -> (Vec<Vec<u8>>, Vec<f32>) {
    let mut condensed = vec![];
    let mut names = vec![];
    for key in target.keys() {
        names.push(key.to_vec());
    }
    for row in 0..target.len() - 1 {
        for col in row + 1..target.len() {
            condensed.push(same_kmer_numer_as_distance(&target.values().nth(row).unwrap(), &target.values().nth(col).unwrap()));
        }
    }
    (names, condensed)
}

pub fn dendrogram(length: usize, mut condens: Vec<f32>) -> Dendrogram<f32>{
    let dend = linkage(&mut condens, length, Method::Average);
    dend
}

pub fn to_pickle_with_serde_names(names: Vec<Vec<u8>>) {
    // convert names to &str
    let names_str: Vec<&str> = names.iter().map(|x| std::str::from_utf8(x).unwrap()).collect::<Vec<&str>>();
    let serialized = serde_pickle::to_vec(&names_str, Default::default()).unwrap();
    std::fs::write("data/names.pkl", serialized).unwrap();
}

pub fn to_pickle_with_serde_dend(dend: Dendrogram<f32>) {
    let serialized = serde_pickle::to_vec(&format!("{:?}", dend), Default::default()).unwrap();
    std::fs::write("data/dendrogram.pkl", serialized).unwrap();
}
