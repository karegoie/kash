//use needletail::{parser, Sequence};
use needletail::*;
use std::collections::HashMap;
use rayon::prelude::*;

fn set_parameters() -> HashMap<&'static str, &'static str> {
    let mut parameters = HashMap::new();
    parameters.insert("file", "data/test.fa");
    parameters.insert("kmer", "3");
    // parameters.insert("ploidy", "2");
    parameters
}

pub fn read_and_count_parallel() -> HashMap<Vec<u8>, HashMap<Vec<u8>, usize>> {
    let params = set_parameters(); // TODO: Remove it as an argument
    let filename = params["file"];
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    let mut kmer_count: HashMap<Vec<u8>, HashMap<Vec<u8>, usize>> = HashMap::new();

    // Save k-mer for each sequence in the HashMap
    while let Some(record) = reader.next() {
        let id = record.clone().expect("invalid record").id().to_owned();
        let seqrec = record.clone().expect("invalid record");
        let norm_seq = seqrec.normalize(false);
        let kmer_count_per_seq: HashMap<Vec<u8>, usize> = norm_seq
            .canonical_kmers(params["kmer"].parse().unwrap(), &norm_seq)
            .map(|(_, kmer, _)| kmer.to_owned())
            .collect::<Vec<Vec<u8>>>()
            .par_iter() // Parallelize the iteration
            .fold(
                || HashMap::new(),
                |mut acc, kmer| {
                    acc.entry(kmer.clone()).and_modify(|e| *e += 1).or_insert(1);
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut acc1, acc2| {
                    for (kmer, count) in acc2 {
                        acc1.entry(kmer).and_modify(|e| *e += count).or_insert(count);
                    }
                    acc1
                },
            );
        kmer_count.insert(id, kmer_count_per_seq);
    }
    kmer_count
}