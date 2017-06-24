// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait ABCIApplication {
    fn echo(&self, o: ::grpc::RequestOptions, p: super::types::RequestEcho) -> ::grpc::SingleResponse<super::types::ResponseEcho>;

    fn flush(&self, o: ::grpc::RequestOptions, p: super::types::RequestFlush) -> ::grpc::SingleResponse<super::types::ResponseFlush>;

    fn info(&self, o: ::grpc::RequestOptions, p: super::types::RequestInfo) -> ::grpc::SingleResponse<super::types::ResponseInfo>;

    fn set_option(&self, o: ::grpc::RequestOptions, p: super::types::RequestSetOption) -> ::grpc::SingleResponse<super::types::ResponseSetOption>;

    fn deliver_tx(&self, o: ::grpc::RequestOptions, p: super::types::RequestDeliverTx) -> ::grpc::SingleResponse<super::types::ResponseDeliverTx>;

    fn check_tx(&self, o: ::grpc::RequestOptions, p: super::types::RequestCheckTx) -> ::grpc::SingleResponse<super::types::ResponseCheckTx>;

    fn query(&self, o: ::grpc::RequestOptions, p: super::types::RequestQuery) -> ::grpc::SingleResponse<super::types::ResponseQuery>;

    fn commit(&self, o: ::grpc::RequestOptions, p: super::types::RequestCommit) -> ::grpc::SingleResponse<super::types::ResponseCommit>;

    fn init_chain(&self, o: ::grpc::RequestOptions, p: super::types::RequestInitChain) -> ::grpc::SingleResponse<super::types::ResponseInitChain>;

    fn begin_block(&self, o: ::grpc::RequestOptions, p: super::types::RequestBeginBlock) -> ::grpc::SingleResponse<super::types::ResponseBeginBlock>;

    fn end_block(&self, o: ::grpc::RequestOptions, p: super::types::RequestEndBlock) -> ::grpc::SingleResponse<super::types::ResponseEndBlock>;
}

// client

pub struct ABCIApplicationClient {
    grpc_client: ::grpc::Client,
    method_Echo: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestEcho, super::types::ResponseEcho>>,
    method_Flush: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestFlush, super::types::ResponseFlush>>,
    method_Info: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestInfo, super::types::ResponseInfo>>,
    method_SetOption: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestSetOption, super::types::ResponseSetOption>>,
    method_DeliverTx: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestDeliverTx, super::types::ResponseDeliverTx>>,
    method_CheckTx: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestCheckTx, super::types::ResponseCheckTx>>,
    method_Query: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestQuery, super::types::ResponseQuery>>,
    method_Commit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestCommit, super::types::ResponseCommit>>,
    method_InitChain: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestInitChain, super::types::ResponseInitChain>>,
    method_BeginBlock: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestBeginBlock, super::types::ResponseBeginBlock>>,
    method_EndBlock: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::types::RequestEndBlock, super::types::ResponseEndBlock>>,
}

impl ABCIApplicationClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ABCIApplicationClient {
            grpc_client: grpc_client,
            method_Echo: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Echo".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Flush: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Flush".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Info: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Info".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SetOption: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/SetOption".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeliverTx: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/DeliverTx".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CheckTx: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/CheckTx".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Query: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Query".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Commit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Commit".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_InitChain: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/InitChain".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BeginBlock: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/BeginBlock".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_EndBlock: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/EndBlock".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ABCIApplicationClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ABCIApplicationClient::with_client(c)
        })
    }
}

impl ABCIApplication for ABCIApplicationClient {
    fn echo(&self, o: ::grpc::RequestOptions, p: super::types::RequestEcho) -> ::grpc::SingleResponse<super::types::ResponseEcho> {
        self.grpc_client.call_unary(o, p, self.method_Echo.clone())
    }

    fn flush(&self, o: ::grpc::RequestOptions, p: super::types::RequestFlush) -> ::grpc::SingleResponse<super::types::ResponseFlush> {
        self.grpc_client.call_unary(o, p, self.method_Flush.clone())
    }

    fn info(&self, o: ::grpc::RequestOptions, p: super::types::RequestInfo) -> ::grpc::SingleResponse<super::types::ResponseInfo> {
        self.grpc_client.call_unary(o, p, self.method_Info.clone())
    }

    fn set_option(&self, o: ::grpc::RequestOptions, p: super::types::RequestSetOption) -> ::grpc::SingleResponse<super::types::ResponseSetOption> {
        self.grpc_client.call_unary(o, p, self.method_SetOption.clone())
    }

    fn deliver_tx(&self, o: ::grpc::RequestOptions, p: super::types::RequestDeliverTx) -> ::grpc::SingleResponse<super::types::ResponseDeliverTx> {
        self.grpc_client.call_unary(o, p, self.method_DeliverTx.clone())
    }

    fn check_tx(&self, o: ::grpc::RequestOptions, p: super::types::RequestCheckTx) -> ::grpc::SingleResponse<super::types::ResponseCheckTx> {
        self.grpc_client.call_unary(o, p, self.method_CheckTx.clone())
    }

    fn query(&self, o: ::grpc::RequestOptions, p: super::types::RequestQuery) -> ::grpc::SingleResponse<super::types::ResponseQuery> {
        self.grpc_client.call_unary(o, p, self.method_Query.clone())
    }

    fn commit(&self, o: ::grpc::RequestOptions, p: super::types::RequestCommit) -> ::grpc::SingleResponse<super::types::ResponseCommit> {
        self.grpc_client.call_unary(o, p, self.method_Commit.clone())
    }

    fn init_chain(&self, o: ::grpc::RequestOptions, p: super::types::RequestInitChain) -> ::grpc::SingleResponse<super::types::ResponseInitChain> {
        self.grpc_client.call_unary(o, p, self.method_InitChain.clone())
    }

    fn begin_block(&self, o: ::grpc::RequestOptions, p: super::types::RequestBeginBlock) -> ::grpc::SingleResponse<super::types::ResponseBeginBlock> {
        self.grpc_client.call_unary(o, p, self.method_BeginBlock.clone())
    }

    fn end_block(&self, o: ::grpc::RequestOptions, p: super::types::RequestEndBlock) -> ::grpc::SingleResponse<super::types::ResponseEndBlock> {
        self.grpc_client.call_unary(o, p, self.method_EndBlock.clone())
    }
}

// server

pub struct ABCIApplicationServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for ABCIApplicationServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl ABCIApplicationServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : ABCIApplication + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> ::grpc::Result<Self> {
        let service_definition = ABCIApplicationServer::new_service_def(h);
        Ok(ABCIApplicationServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition)?,
        })
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : ABCIApplication + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> ::grpc::Result<Self> {
        let service_definition = ABCIApplicationServer::new_service_def(h);
        Ok(ABCIApplicationServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool)?,
        })
    }

    pub fn new_service_def<H : ABCIApplication + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Echo".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.echo(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Flush".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.flush(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Info".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.info(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/SetOption".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.set_option(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/DeliverTx".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.deliver_tx(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/CheckTx".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.check_tx(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Query".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.query(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Commit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.commit(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/InitChain".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.init_chain(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/BeginBlock".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.begin_block(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/EndBlock".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.end_block(o, p))
                    },
                ),
            ],
        )
    }
}
