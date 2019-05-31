use cpal::*;
use std::thread;
use std::sync::mpsc;
use std::collections::VecDeque;

mod calibrate;

const BUF_SIZE: usize = 1024;

///Take audio from the device's current default input and output a phase-reversed
/// version to default audio output, creating a noise-cancelling effect. 
/// Requires an additional input device inside the headphones to calculate delay and amplitude

fn main() {
    //Set up seperate thread for input and output
    let (tx, rx) = mpsc::channel();

    //Spawn thread that reads input and sends it to output thread
    thread::spawn(|| {
        let default_in = default_input_device().expect("Error finding default input");
        let event_loop_in = EventLoop::new();
        let in_format = default_in.default_input_format().expect("Error getting default input format");
        let input = event_loop_in.build_input_stream(&default_in, &in_format)
                .expect("Error building input stream");
        event_loop_in.play_stream(input.to_owned());

        event_loop_in.run(move |_, stream_data| {
            //catch input in u16, i16, or f32 format
            match stream_data {
                StreamData::Input { buffer: UnknownTypeInputBuffer::U16(buffer) } => {
                    for elem in buffer.iter() {
                        tx.send(*elem as f32).expect("input thread failed on send u16");
                    }
                    println!("input U16");
                    //println!();
                },
                StreamData::Input { buffer: UnknownTypeInputBuffer::I16(buffer) } => {
                    for elem in buffer.iter() {
                        tx.send(*elem as f32).expect("input thread failed on send i16");
                    }
                    println!("input I16");
                    //println!();
                },
                StreamData::Input { buffer: UnknownTypeInputBuffer::F32(buffer) } => {
                    for elem in buffer.iter() {
                        tx.send(*elem).expect("input thread failed on send f32");
                    }
                    println!("input F32");
                    //println!();
                },
                _ => println!("no input"),
            };
        });
    });

    let mut buf: VecDeque<f32> = VecDeque::new(); //Buffer of input samples
    //Build buffer up to input size
    for _ in 0..BUF_SIZE {
        buf.push_back(rx.recv().expect("failed to build buffer"));
    }

    let default_out = default_output_device().expect("Error finding default output");
    let event_loop_out = EventLoop::new();
    let out_format = default_out.default_output_format().expect("Error getting default output format");
    let output = event_loop_out.build_output_stream(&default_out, &out_format)
            .expect("Failed to build output stream");
    event_loop_out.play_stream(output);

    event_loop_out.run(move |_, stream_data| {
        //Output audio in u16, i16, or f32 format
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    match buf.pop_front() {
                        Some(i) => *elem = -i as u16,
                        None => (), //Silently drop the sample if buffer is empty (not sure why this is happening)
                    }
                    //*elem = -buf.pop_front().expect("pop failed in u16") as u16;
                    //*elem = -rx.recv().expect("pop failed in u16") as u16;
                    println!("output {:?}", elem);
                }
                //println!("Output u16");
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    match buf.pop_front() {
                        Some(i) => *elem = -i as i16,
                        None => (),
                    }
                    //*elem = -buf.pop_front().expect("pop failed in i16") as i16;
                    //*elem = -rx.recv().expect("pop failed in u16") as i16;
                    println!("output {:?}", elem);
                }
                //println!("Output i16");
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    match buf.pop_front() {
                        Some(i) => *elem = -i,
                        None => (),
                    }
                    //*elem = -buf.pop_front().expect("pop failed in f32 section");
                    //*elem = -rx.recv().expect("pop failed in u16");
                    println!("output {:?}", elem);
                }
                //println!("output f32");
            },
            _ => println!("no output"),
        };
        buf.push_back(rx.recv().expect("receive failed on push in event loop"));
    });
}
