[![Makefile CI](https://github.com/nogibjj/databricks-cluster-cli/actions/workflows/makefile.yml/badge.svg)](https://github.com/nogibjj/databricks-cluster-cli/actions/workflows/makefile.yml)

# Databricks Cluster Management CLI

This is a command-line interface tool built in Rust and Bash that allows you to provision custom clusters in Databricks using the Databricks REST API.

# Architectural Diagram
coming soon to a github repo near you!

# Setup

1. Set up Azure Databricks workspace

Under *Azure Databricks*, create a new workspace
![image](https://user-images.githubusercontent.com/55398496/232632054-4cfbfaba-5ca7-46c7-91c0-27486761e85a.png)

You can set this up with whatever settings you'd prefer.


2. Install dependencies
```
make install
```

3. Authenticate to Azure
```
az login
```
This will redirect you to the Azure portal to sign into your account.

4. Get yourself a Databricks token. Assuming step 1 has finished, you can now launch your Databricks workspace. From there, naviagate to your email address in the top right corner and click *user settings* on the dropdown. Then, generate a new token.

![image](https://user-images.githubusercontent.com/55398496/232633600-b9147de9-88e3-4faa-bbcc-d949a7bfd3f7.png) <br><br>
![image](https://user-images.githubusercontent.com/55398496/232633670-f6df218e-41ff-4e27-ae6c-6e0f4021a386.png)

In your terminal, type:
```
databricks configure --token
```
You will then be prompted for the token you just generated on Databricks.

5. Run the program
You can provision your own cluster with the following format:
```
./cluster create --name <YOUR_CLUSTER_NAME> --optimize <PREFERRED_OPTIMIZATION>
```
Optimize is an optional flag which can be one of ***[general, memory, storage, compute]***, depending on what you'd like your cluster to be provisioned for. The goal of this tool is to make a developer's choices simpler, so I pre-picked specific cluster types that everyone should have access to for these categories.

Now that your cluster is ready, you can navigate back to your workspace and use it for whatever you need!

To list your clusters at any point, you can run:
```
./cluster list
```

6. Cleaning up
To delete all clusters, run the separate bash script:
```
chmod +x ./delete_clusters.sh
./delete_clusters.sh
```

If you only want to delete one cluster, grab its cluster ID from the ```./cluster list``` command, then run:
```
./cluster delete <YOUR_CLUSTER_ID>
```

# Tests
To run the tests, run ```make test``` in the project directory. The tests include unit tests that ensure the quality of the create and delete commands. Alternatively, you can cd into the source code folder (cluster-rs) and run ```cargo test```, but the Makefile step is recommended as it includes checks to make sure clusters aren't accidentally left idle.
