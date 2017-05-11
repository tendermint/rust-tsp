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
    fn Echo(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestEcho) -> ::grpc::GrpcSingleResponse<super::types::ResponseEcho>;

    fn Flush(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestFlush) -> ::grpc::GrpcSingleResponse<super::types::ResponseFlush>;

    fn Info(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestInfo) -> ::grpc::GrpcSingleResponse<super::types::ResponseInfo>;

    fn SetOption(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestSetOption) -> ::grpc::GrpcSingleResponse<super::types::ResponseSetOption>;

    fn DeliverTx(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestDeliverTx) -> ::grpc::GrpcSingleResponse<super::types::ResponseDeliverTx>;

    fn CheckTx(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestCheckTx) -> ::grpc::GrpcSingleResponse<super::types::ResponseCheckTx>;

    fn Query(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestQuery) -> ::grpc::GrpcSingleResponse<super::types::ResponseQuery>;

    fn Commit(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestCommit) -> ::grpc::GrpcSingleResponse<super::types::ResponseCommit>;

    fn InitChain(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestInitChain) -> ::grpc::GrpcSingleResponse<super::types::ResponseInitChain>;

    fn BeginBlock(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestBeginBlock) -> ::grpc::GrpcSingleResponse<super::types::ResponseBeginBlock>;

    fn EndBlock(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestEndBlock) -> ::grpc::GrpcSingleResponse<super::types::ResponseEndBlock>;
}

// client

pub struct ABCIApplicationClient {
    grpc_client: ::grpc::client::GrpcClient,
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
    pub fn with_client(grpc_client: ::grpc::client::GrpcClient) -> Self {
        ABCIApplicationClient {
            grpc_client: grpc_client,
            method_Echo: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Echo".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_Flush: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Flush".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_Info: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Info".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_SetOption: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/SetOption".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_DeliverTx: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/DeliverTx".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_CheckTx: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/CheckTx".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_Query: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Query".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_Commit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/Commit".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_InitChain: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/InitChain".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_BeginBlock: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/BeginBlock".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
            method_EndBlock: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/types.ABCIApplication/EndBlock".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::client::GrpcClientConf) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls, conf).map(|c| {
            ABCIApplicationClient::with_client(c)
        })
    }
}

impl ABCIApplication for ABCIApplicationClient {
    fn Echo(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestEcho) -> ::grpc::GrpcSingleResponse<super::types::ResponseEcho> {
        self.grpc_client.call_unary(o, p, self.method_Echo.clone())
    }

    fn Flush(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestFlush) -> ::grpc::GrpcSingleResponse<super::types::ResponseFlush> {
        self.grpc_client.call_unary(o, p, self.method_Flush.clone())
    }

    fn Info(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestInfo) -> ::grpc::GrpcSingleResponse<super::types::ResponseInfo> {
        self.grpc_client.call_unary(o, p, self.method_Info.clone())
    }

    fn SetOption(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestSetOption) -> ::grpc::GrpcSingleResponse<super::types::ResponseSetOption> {
        self.grpc_client.call_unary(o, p, self.method_SetOption.clone())
    }

    fn DeliverTx(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestDeliverTx) -> ::grpc::GrpcSingleResponse<super::types::ResponseDeliverTx> {
        self.grpc_client.call_unary(o, p, self.method_DeliverTx.clone())
    }

    fn CheckTx(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestCheckTx) -> ::grpc::GrpcSingleResponse<super::types::ResponseCheckTx> {
        self.grpc_client.call_unary(o, p, self.method_CheckTx.clone())
    }

    fn Query(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestQuery) -> ::grpc::GrpcSingleResponse<super::types::ResponseQuery> {
        self.grpc_client.call_unary(o, p, self.method_Query.clone())
    }

    fn Commit(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestCommit) -> ::grpc::GrpcSingleResponse<super::types::ResponseCommit> {
        self.grpc_client.call_unary(o, p, self.method_Commit.clone())
    }

    fn InitChain(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestInitChain) -> ::grpc::GrpcSingleResponse<super::types::ResponseInitChain> {
        self.grpc_client.call_unary(o, p, self.method_InitChain.clone())
    }

    fn BeginBlock(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestBeginBlock) -> ::grpc::GrpcSingleResponse<super::types::ResponseBeginBlock> {
        self.grpc_client.call_unary(o, p, self.method_BeginBlock.clone())
    }

    fn EndBlock(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestEndBlock) -> ::grpc::GrpcSingleResponse<super::types::ResponseEndBlock> {
        self.grpc_client.call_unary(o, p, self.method_EndBlock.clone())
    }
}

// server

pub struct ABCIApplicationServer {
    pub grpc_server: ::grpc::server::GrpcServer,
}

impl ::std::ops::Deref for ABCIApplicationServer {
    type Target = ::grpc::server::GrpcServer;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl ABCIApplicationServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : ABCIApplication + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H) -> Self {
        let service_definition = ABCIApplicationServer::new_service_def(h);
        ABCIApplicationServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : ABCIApplication + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::server::GrpcServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = ABCIApplicationServer::new_service_def(h);
        ABCIApplicationServer {
            grpc_server: ::grpc::server::GrpcServer::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : ABCIApplication + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Echo".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.Echo(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Flush".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.Flush(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Info".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.Info(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/SetOption".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.SetOption(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/DeliverTx".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.DeliverTx(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/CheckTx".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.CheckTx(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Query".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.Query(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/Commit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.Commit(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/InitChain".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.InitChain(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/BeginBlock".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.BeginBlock(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/types.ABCIApplication/EndBlock".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.EndBlock(o, p))
                    },
                ),
            ],
        )
    }
}
