extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod types_grpc;
pub mod types;

pub fn hello() -> String {
    "Hello!".to_string()
}

pub struct Server {

}

impl Server {
    pub fn new(lAddr: &str, connectionType: &str) -> Option<Server> {
        match connectionType {
            "grpc" => {
                println!("GRPC");
                Some(Server{})
            },
            "socket" => {
                println!("SOCKET");
                Some(Server{})
            },
            _ => {
                println!("UNKNOWN");
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
