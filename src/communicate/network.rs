use std::sync::Arc;
use std::borrow::Cow;
use transport::interact::InteractApi;
use futures::Future;
use Error;
use transport::parse::status_code;
use transport::parse::parse_to_trait;
use http::StatusCode;
use representation::rep::NetworkDetails;
use transport::interact::InteractApiExt;
use build::ContainerConnectionOptions;
use communicate::util::AsSlice;

/// Interface for accessing and manipulating a docker network
pub struct Network<'b> {
    interact: Arc<InteractApi>,
    id: Cow<'b, str>,
}

impl<'b> Network<'b> {
    /// Exports an interface exposing operations against a network instance
    pub(crate) fn new<S>(interact: Arc<InteractApi>, id: S) -> Network<'b>
        where
            S: Into<Cow<'b, str>>,
    {
        Network {
            interact,
            id: id.into(),
        }
    }

    /// a getter for the Network id
    pub fn id(&self) -> &str {
        &self.id
    }


    /// Inspects the current docker network instance's details
    pub fn inspect(&self) -> impl Future<Item=NetworkDetails, Error=Error> {
        let path = format!("/networks/{}", self.id);

        parse_to_trait::<NetworkDetails>(self.interact.get(path.as_str()))
    }

    /// Delete the network instance
    pub fn delete(&self) -> impl Future<Item=StatusCode, Error=Error> {
        let path = format!("/networks/{}", self.id);

        status_code(self.interact.delete(path.as_str()))
    }

    /// Connect container to network
    pub fn connect(&self, opts: &ContainerConnectionOptions) -> impl Future<Item=StatusCode, Error=Error> {
        let path = format!("/networks/{}/connect", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }

    /// Disconnect container to network
    pub fn disconnect(&self, opts: &ContainerConnectionOptions)
                      -> impl Future<Item=StatusCode, Error=Error> {
        let path = format!("/networks/{}/disconnect", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }
}
