#!/bin/bash

# Get all cluster IDs
cluster_ids=$(databricks clusters list | awk '{if(NR>1) print $1}')

echo "Deleting all clusters used for test cases"

# Loop through each cluster ID and delete it
for id in $cluster_ids
do
  echo "Deleting cluster with ID: $id"
  databricks clusters permanent-delete --cluster-id $id
done
