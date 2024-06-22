use std::io::{Read, Write, self};

#[path = ".generated/proto/mod.rs"]
mod proto_chat;

use prost::Message;
use prost_types::Any;
use proto_chat::proto_chat::{ProtoChatHistory, ProtoChatMessage};
use room_server_interface::proto::{ client_input, client_output::{self, ClientOutput, DirectMessage, RoomBroadcast} };


fn main() {

    let mut proto_chat_history = ProtoChatHistory::default();

    loop {
        let mut message_len_buff: [u8; std::mem::size_of::<u64>()] = [0, 0, 0, 0, 0, 0, 0, 0];
        io::stdin().read_exact(&mut message_len_buff).unwrap();

        let input_data_len = u64::from_be_bytes(message_len_buff);
        let mut input_message_buff = Vec::<u8>::new();
        input_message_buff.resize(input_data_len as usize, 0);
        io::stdin().read_exact(&mut input_message_buff).unwrap();

        let instance_input_proto = client_input::ClientInput::decode(input_message_buff.as_slice()).unwrap();
        let client_id = instance_input_proto.client_id.unwrap();
        let client_msg = instance_input_proto.game_input_message.unwrap();
        let client_msg_type = client_msg.type_url.clone();

        let mut client_output_proto = client_output::ClientOutputBatch::default();
        match client_msg_type.as_str() {
            "proto_chat.message" => {
                let decoded = ProtoChatMessage::decode(client_msg.value.as_slice()).unwrap();
                proto_chat_history.history.push(decoded);
                let client_output = ClientOutput {
                    game_output_message: Some(client_msg)
                };
                let room_broadcast = RoomBroadcast {
                    client_output: Some(client_output)
                };
                client_output_proto.broadcast = Some(room_broadcast);
            }
            "proto_chat.request" => {
                if let Ok(proto_chat_request) = proto_chat::proto_chat::ProtoChatRequest::decode(client_msg.value.as_slice()) {
                    for request in &proto_chat_request.requests {
                        match proto_chat::proto_chat::Request::try_from(*request).unwrap() {
                            proto_chat::proto_chat::Request::History => {
                                let encoded_history = Any {
                                    type_url: "proto_chat.history".to_owned(),
                                    value: proto_chat_history.clone().encode_to_vec()
                                };
                                let client_output = ClientOutput {
                                    game_output_message: Some(encoded_history)
                                };
                                let direct_message = DirectMessage {
                                    client_id: Some(client_id.clone()),
                                    client_output: Some(client_output)
                                };
                                client_output_proto.direct_messages.push(direct_message);
                            }
                        }
                    }
                }
            }
            _ => {}
        };
        
        let output_encoded = client_output_proto.encode_to_vec();
        let _ = io::stdout().write(&output_encoded.len().to_ne_bytes());
        let _ = io::stdout().write(output_encoded.as_slice());
        let _ = io::stdout().flush();
    }
}
