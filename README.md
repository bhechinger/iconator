# Mainmatter Rust Assessment

> Please take no more than ~1 hour to complete this assignment. We’ll discuss your solutions in a call in a few days. Feel free to ask questions if the assignment is unclear at any point.

## Context

This repository contains a simple file-icon lookup library as may be used by IDEs for their project tree sidebar. `get_icon_for_file` and `get_icon_for_folder` accept a filepath and return the numeric ID of a corresponding icon if one is found. Each icon ID corresponds to an svg icon in `./icons`.

## Assignment

1. Implement a web server than allows users to query for file/folder icons over the network. 
2. Write a short (<= 1 page) design/implementation note describing:
  - Design choices and tradeoffs you made (high-level architecture, web server choice, libraries you picked etc.)
  - How you would productionize this service (deployment, observability, etc.)
  
Keep your audience in mind. We’re not interested in a perfect solution but one where we see your thought-process and working style in action.
  
You may add these notes at the end of this file if you wish.

# Notes

## How to use
* Run the service: `cargo run`
* Call the service for a file: `curl localhost:3000/file/foo.txt`
* Call the service for a folder: `curl localhost:3000/folder/.github`
* Call the service for an icon file: `curl http://localhost:3000/icons/617.svg`
* Call the service to get an error: `curl localhost:3000/file/foo.fake`

## Design choices and tradeoffs
### Web server and libraries
* Web server choice of `axum` as it's easy to use and performant
* Libraries
    * `axum-prometheus` for easy win metrics
    * `lazy_static` to serve the icon images

### High level architecture
* simple interface
    * one endpoint for files: `/file/`
    * one endpoint for folders: `/folder/`
    * one endpoint for icon images: `/icons/`
* calls to the file/folder endpoints return a full url to the icon file
    * the generated URL is... not smart. It doesn't examine the request to determine the host/port to use, it just blindly uses `localhost:3000`

### Errors
* Currently errors return no response only an HTTP code
* Maybe add response text for clarity

## Productionization
### Deployment
* Deployment assumes Kubernetes
* Helm chart to deploy a multi-replica pod with a LoadBalancer service as well as service accounts and RBAC as needed
* Implement liveness/readiness endpoints in the service to give Kubernetes better control over misbehaving pods

### Observability
* Better logging output
* Tracing support added
* Expanded metrics with more details (if needed)
* Metrics, logs, and traces collected and shipped to aggregation/dashboard like Grafana or Datadog
