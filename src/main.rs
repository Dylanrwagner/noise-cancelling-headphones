use cpal::*;
use std::thread;
use std::sync::mpsc;

fn main() {
    //Set up seperate thread for input and output
    let (tx, rx) = mpsc::channel();
    

    thread::spawn(|| {
        let default_in = default_input_device().expect("Error finding defalt input");
        let event_loop_in = EventLoop::new();
        let in_format = default_in.default_input_format().expect("Error getting default input format");
        let input = event_loop_in.build_input_stream(&default_in, &in_format)
                .expect("Error building input stream");
        event_loop_in.play_stream(input.to_owned());

        event_loop_in.run(move |_, stream_data| {
            match stream_data {
                StreamData::Input { buffer: UnknownTypeInputBuffer::U16(mut buffer) } => {
                    for elem in buffer.iter() {
                        //tx.send(elem).unwrap();
                    }
                    //println!();
                },
                StreamData::Input { buffer: UnknownTypeInputBuffer::I16(mut buffer) } => {
                    for elem in buffer.iter() {
                        //tx.send(elem).unwrap();
                    }
                    //println!();
                },
                StreamData::Input { buffer: UnknownTypeInputBuffer::F32(mut buffer) } => {
                    for elem in buffer.iter() {
                        tx.send(*elem).unwrap();
                    }
                    //println!();
                },
                _ => (),
            };
        });
    });

    let default_out = default_output_device().expect("Error finding defalt output");
    let event_loop_out = EventLoop::new();
    let out_format = default_out.default_output_format().expect("Error getting default output format");
    let output = event_loop_out.build_output_stream(&default_out, &out_format)
            .expect("Failed to build output stream");
    event_loop_out.play_stream(output);

    event_loop_out.run(move |_, stream_data| {
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    //*elem = *rx.recv().unwrap();
                    println!("output {:?}", elem);
                }
                //println!();
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    //*elem = *rx.recv().unwrap();
                    println!("output {:?}", elem);
                }
                //println!();
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    *elem *= -rx.recv().unwrap();
                    println!("output {:?}", elem);
                }
                //println!();
            },
            _ => (),
        };
    });
}
