extern crate async_docker;
extern crate http;
extern crate futures;
extern crate tokio;

use async_docker::{DockerApi, new_docker};
use futures::{future, Future};
use std::env;
use async_docker::build::ContainerArchiveOptionsBuilder;

fn main() {
    if env::args().count() < 3 {
        println!("Too few arguments (<3).");
        return;
    }

    let container = env::args().nth(1).unwrap();
    let local_path = env::args().nth(2).unwrap();
    let remote_path = env::args().nth(3).unwrap();

    let work = future::lazy(|| {
        let opts = ContainerArchiveOptionsBuilder::new()
            .local_path(local_path)
            .remote_path(remote_path)
            .build();
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .container(container.into())
            .archive_put(&opts)
            .then(|a| Ok(println!("{:?}", a)))
    });

    tokio::runtime::run(work);
}