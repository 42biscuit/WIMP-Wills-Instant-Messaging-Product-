use std::net::SocketAddr;

#[derive(Clone,Debug)]

pub struct User(pub(crate) String, pub(crate) SocketAddr);
