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

pub trait IPD {
    fn game(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::ipd::MoveMessage>) -> ::grpc::StreamingResponse<super::ipd::MoveMessage>;
}

// client

pub struct IPDClient {
    grpc_client: ::grpc::Client,
    method_Game: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::ipd::MoveMessage, super::ipd::MoveMessage>>,
}

impl IPDClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        IPDClient {
            grpc_client: grpc_client,
            method_Game: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/protobuf.IPD/Game".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            IPDClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            IPDClient::with_client(c)
        })
    }
}

impl IPD for IPDClient {
    fn game(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::ipd::MoveMessage>) -> ::grpc::StreamingResponse<super::ipd::MoveMessage> {
        self.grpc_client.call_bidi(o, p, self.method_Game.clone())
    }
}

// server

pub struct IPDServer;


impl IPDServer {
    pub fn new_service_def<H : IPD + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/protobuf.IPD",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/protobuf.IPD/Game".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.game(o, p))
                    },
                ),
            ],
        )
    }
}
