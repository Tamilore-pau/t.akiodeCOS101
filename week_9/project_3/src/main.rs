fn main() {
    // 3 separate vector datasets
    let dataset_1 = vec![("Aigbogun Alamba Daudu", "Internal Affairs", "South West")];
    let dataset_2 = vec![("Murtala Afeez Bendu", "Justice", "North East")];
    let dataset_3 = vec![("Okorocha Calistus Ogbona", "Defense", "South South")];

    // Combine into a single vector cleanly using iterators
    let merged_file: Vec<_> = vec![dataset_1, dataset_2, dataset_3]
        .into_iter()
        .flatten()
        .collect();

    println!("{:?}", merged_file);
}

