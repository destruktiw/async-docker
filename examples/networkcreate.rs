extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;


use std::env;
use shiplift::communicate::DockerApi;
use shiplift::communicate::new_docker;
use futures::future;
use futures::Future;
use shiplift::NetworkCreateOptions;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let network_name = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        let opts = &NetworkCreateOptions::builder(network_name.as_ref())
            .build();

        docker
            .networks()
            .create(&opts)
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
