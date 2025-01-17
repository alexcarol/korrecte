# korrecte

Collection of lints to detect common pitfalls or security issues on your Kubernetes cluster.

It currently connects through Kubernetes API and requires that credentials are present available for the user who launches the application.


## How to run

After installing the Rust compiler and downloading the source code, run:

```bash
cargo run
```

If credentials are available, you should see a list of all the findings detected by the application. For example:

```bash
overlapping_probes on hello-node [default]. Metadata: {"liveness_start": "10s", "container": "hello-node", "readiness_max_delay": "11s"}
never_restart_with_liveness_probe on hello-node-hardcoded-env-var [test]. Metadata: {}
environment_passwords on hello-node-hardcoded-env-var [test]. Metadata: {"environment_var": "ADMIN_PASSWORD"}
environment_passwords on hello-node-hardcoded-env-var [test]. Metadata: {"environment_var": "ADMIN_TOKEN"}
environment_passwords on hello-node-hardcoded-env-var [test]. Metadata: {"environment_var": "KEY_SERVICE"}
never_restart_with_liveness_probe on hello-node-never-restart [test]. Metadata: {}
required_labels on kube-addon-manager-minikube [kube-system]. Metadata: {"missing_labels": "[\"app\"]"}
service_without_matching_labels on my-service [default]. Metadata: {}
```

## Customization

There are some lints that can be parametrized through a TOML file. You can copy the default `korrecte.toml` file and run the program with the `--config` flag:

```bash
cargo run -- --config /path/to/file.toml
```

## File linting

Instead of requiring a running Kubernetes cluster, `korrecte` is able to lint YAML manifests instead. Note that those lints that requires to read some state on the cluster, may not work as expected when running in this mode. For example, the `service_without_matching_labels` lints, searches all the possible matching pod, but it probably needs access to pods that are not defined on the manifest.

To lint a specific file, you can run:

```bash
cargo run -- --source file --path <path to file>
``` 

If the path is a directory, it will iterate over all the files and will apply the lints on each of them. 

## Roadmap ideas

- Allow filtering by namespace or by name regex
- Change exit code if the `korrecte` finds any issue
- Make the application deployable, evaluate the lints continuously and create an API to retrieve them
- Add more reporting hooks. For example, statsd, datadog, prometheus, ... 