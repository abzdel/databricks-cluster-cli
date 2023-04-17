#!/bin/bash

# Get all cluster IDs
cluster_ids=$(databricks clusters list --output JSON | jq -r '.clusters[].cluster_id')

# Loop through each cluster ID and delete it
for id in $cluster_ids
do
  echo "Deleting cluster with ID: $id"
  databricks clusters permanent-delete --cluster-id $id
done
