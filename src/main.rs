extern crate steganography;

use std::env;
use steganography::{
    util::*,
    encoder::*,
    decoder::*,
};

fn encoder(file: &str, input_image: &str, output_image: &str) {
    let secret_message = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(_) => panic!("Error reading the text file"),
    };

    let message = secret_message.to_string();
    let payload = str_to_bytes(&message);
    let destination_image = file_as_dynamic_image(input_image.to_string());

    let enc = Encoder::new(payload, destination_image);
    let result = enc.encode_alpha();

    save_image_buffer(result, output_image.to_string());
    println!("Message encoded successfully!");
}

fn decoder(input_image: &str) {
    let encoded_image = file_as_image_buffer(input_image.to_string());

    let dec = Decoder::new(encoded_image);
    let out_buffer = dec.decode_alpha();

    let clean_buffer: Vec<u8> = out_buffer.into_iter()
                                           .filter(|b| *b != 0xff_u8)
                                           .collect();

    let message = bytes_to_str(clean_buffer.as_slice());
    println!("{}", message);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <encode/decode> <input_image> [<input_text_file>] [<output_image>]", args[0]);
        return;
    }

    let mode = &args[1];
    let input_image = &args[2];

    match mode.as_str() {
        "encode" => {
            if args.len() < 5 {
                println!("Usage: {} encode <input_text_file> <input_image> <output_image>", args[0]);
                return;
            }

            let input_text_file = &args[3];
            let output_image = &args[4];
            encoder(input_text_file, input_image, output_image);
        },

        "decode" => {
            decoder(input_image);
        },

        _ => {
            println!("Invalid mode. Use 'encode' or 'decode'.");
        }
    }
}
