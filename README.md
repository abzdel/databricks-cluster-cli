[![Makefile CI](https://github.com/nogibjj/databricks-cluster-cli/actions/workflows/makefile.yml/badge.svg)](https://github.com/nogibjj/databricks-cluster-cli/actions/workflows/makefile.yml)

# recreating steps

1. Set up Azure Databricks workspace (picture coming soon to a github repo near you)
2. install azure cli (make install-azure - will be automated into the build system) + install databricks cli
3. authenticate to azure (az login)
4. authenticate to databricks (databricks configure (--token?)")
4.5. get token by going to databricks workspace - user settings - access tokens - generate new token
5. make all
6. use tool
./cluster create <your-cluster-name> <(OPTIONAL) desired-optimization>
