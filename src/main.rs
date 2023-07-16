/// KASH: K-mer Anchoring of Sub-genomes via Hierarchical clustering
/// Hyunsu Lim, 2023
///
//use kash::sort;
//use kash::getmer;
//use kash::count;
//use kodama::{Method, linkage, Dendrogram};

fn main() {
    todo!()
}


#[cfg(test)]
mod test {
    use kash::sort;
    use kash::getmer;
    use kash::count;
    use std::str;

    #[test]
    fn test_dendrogram() {
        let kmer_count = count::read_and_count_parallel();
        //for (key, value) in kmer_count.iter() {
        //    for (kmer, count) in value.iter() {
        //        println!("{:?} {:?} {:?}", str::from_utf8(key).unwrap(), str::from_utf8(kmer).unwrap(), count);
        //    }
        //}
        let occurrence_values = sort::sort::sort_by_coverage_wrap(&kmer_count);
        // println!("{:?}", occurrence_values.0);
        let derivatives = getmer::differentiate(&occurrence_values.0);
        //println!("{:?}", derivatives);
        let index = getmer::find_index(derivatives);
        //println!("{:?}", index);
        let kmer_count_has_index = getmer::get_mers_from_index_parallel(
            &kmer_count,
            index[0],
            occurrence_values.1);

        // display kmer_count_has_index
        for (key, value) in kmer_count_has_index.iter() {
            println!("{:?} {:?}", str::from_utf8(key).unwrap(), value.iter().map(|x| str::from_utf8(x).unwrap()).collect::<Vec<&str>>());
        }
        let (names, matrix) = getmer::build_condensed_distance_matrix(&kmer_count_has_index);
        let dendrogram = getmer::dendrogram(names.len(), matrix);

        getmer::to_pickle_with_serde_names(names);
        getmer::to_pickle_with_serde_dend(dendrogram);
    }
}
