cd cluster-rs
cargo test --test test_create

# quickly grab all cluster ids. should be the first characters all together up until the first space
databricks clusters list | cut -d ' ' -f 1 | tail -n +2 > tests/cluster_ids.txt

cargo test --test test_delete

# delete every remaining cluster
cd ..
chmod +x delete_clusters.sh
./delete_clusters.sh