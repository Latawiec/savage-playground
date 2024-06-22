use std::{fmt::format, fs::File, io::{self, Read, Write}};

#[path = ".generated/proto/mod.rs"]
mod proto_chat;

use prost::Message;
use prost_types::Any;
use proto_chat::proto_chat::{ProtoChatHistory, ProtoChatMessage};
use room_server_interface::proto::{ client_input, client_output::{self, ClientOutput, DirectMessage, RoomBroadcast} };


fn main() {

    let mut proto_chat_history = ProtoChatHistory::default();
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Let's go!\n").unwrap();
    loop {
        
        let mut message_len_buff: [u8; std::mem::size_of::<u64>()] = [0, 0, 0, 0, 0, 0, 0, 0];
        if let Err(error) = io::stdin().read_exact(&mut message_len_buff) {
            file.write_all(b"Couldn't read bytesize\n").unwrap();
        }

        let input_data_len = u64::from_be_bytes(message_len_buff);
        let mut input_message_buff = Vec::<u8>::new();
        input_message_buff.resize(input_data_len as usize, 0);
        if let Err(error) = io::stdin().read_exact(&mut input_message_buff) {
            file.write_all(b"Couldn't read bytesize\n").unwrap();
        }

        let instance_input_proto = client_input::ClientInput::decode(input_message_buff.as_slice()).unwrap();
        let client_id = instance_input_proto.client_id.unwrap();
        let client_msg = instance_input_proto.game_input_message.unwrap();
        let client_msg_type = client_msg.type_url.clone();

        file.write_all(b"Matched").unwrap();

        let mut client_output_proto = client_output::ClientOutputBatch::default();
        match client_msg_type.as_str() {
            "proto_chat.message" => {
                file.write_all(b"Hi").unwrap();
                let mut decoded_res = ProtoChatMessage::decode(client_msg.value.as_slice());
                if let Err(err) = decoded_res {
                    let formatted = format!("Error: {:?}", err);
                    file.write_all(formatted.as_bytes());
                    continue;
                }
                let mut decoded = decoded_res.unwrap();
                file.write_all(b"Hey").unwrap();
                decoded.user_id = Some(client_id.value);
                let as_any = Any {
                    type_url: client_msg_type,
                    value: decoded.encode_to_vec()
                };
                proto_chat_history.history.push(decoded);
                file.write_all(b"Pushed").unwrap();
                let client_output = ClientOutput {
                    game_output_message: Some(as_any)
                };
                let room_broadcast = RoomBroadcast {
                    client_output: Some(client_output)
                };
                client_output_proto.broadcast = Some(room_broadcast);
            }
            "proto_chat.request" => {
                if let Ok(proto_chat_request) = proto_chat::proto_chat::ProtoChatRequest::decode(client_msg.value.as_slice()) {
                    for request in &proto_chat_request.requests {
                        match proto_chat::proto_chat::ProtoChatRequestType::try_from(*request).unwrap() {
                            proto_chat::proto_chat::ProtoChatRequestType::History => {
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
            _ => {
                file.write_all(format!("Unmatched: {}\n", client_msg_type).as_bytes());
            }
        };
        file.write_all(b"Left").unwrap();
        let output_encoded = client_output_proto.encode_to_vec();
        let _ = io::stdout().write(&output_encoded.len().to_be_bytes());
        let _ = io::stdout().write(output_encoded.as_slice());
        let _ = io::stdout().flush();
        let notify_string = format!("Sent {} bytes of data.\n", output_encoded.len());
        file.write_all(notify_string.as_bytes());
    }
}
