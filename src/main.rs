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
        let sorted_values = sort::sort::sort_by_occurrence(&kmer_count);
        let derivatives = getmer::differentiate(sorted_values);
        //println!("{:?}", derivatives);
        let index = getmer::find_index(derivatives);
        println!("{:?}", index);
        let kmer_count_has_index = getmer::get_mers_from_index_parallel(&kmer_count, index[0]);
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
