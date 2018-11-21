extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::communicate::{new_docker, DockerApi};
use async_docker::models::NetworkConfig;
use futures::{future, Future};
use std::env;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let network_name = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        let opts = &NetworkConfig::new(network_name);

        docker
            .networks()
            .create(&opts)
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
