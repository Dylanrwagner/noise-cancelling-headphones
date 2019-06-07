use cpal::*;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::collections::VecDeque;

mod calibrate;

const DELAY: u64 = 180; //Delay in milliseconds, found through experimentation

///Take audio from the device's current default input and output a phase-reversed
/// version to default audio output, creating a noise-cancelling effect. 
/// Requires an additional input device inside the headphones to calculate delay and amplitude

fn main() {
    //Set up seperate thread for input and output
    let (tx, rx) = mpsc::channel();

    //set up default input device, which will act as the means to cancel the noise
    let default_in = default_input_device().expect("Error finding default input");
    let in_format = default_in.default_input_format().expect("Error getting default input format");
    //let SampleRate(samp_rate) = in_format.sample_rate;

    //A second input device inside the headphones to find some noise-cancelling parameters
    // assumes it, the default input device, and the built-in mic are the only input devices
    let calibrator = input_devices()
        .find(|x| { x.name() != "Built-in Microphone" && *x != default_in})
        .unwrap();
    print!("calibrating...  ");
    let amp = calibrate::calc_amplitude(&calibrator);
    println!("done");

    //Spawn thread that reads input and sends it to output thread
    thread::spawn(move || {
        let event_loop_in = EventLoop::new();
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

    //Number of samples to delay for
    //let delay_samples = (DELAY * samp_rate as f32).floor() as usize;

    let buf: Arc<Mutex<VecDeque<f32>>> = Arc::new(Mutex::new(VecDeque::new()));
    let buf_clone = buf.clone();
    thread::spawn(move || {
        loop {
            let elem = rx.recv().expect("Failed to receive to buffer");
            buf_clone.lock().unwrap().push_back(elem);
        }
    });
    //let mut buf: VecDeque<f32> = VecDeque::new(); //Buffer of input samples
    //Build buffer up to input size
    //for _ in 0..delay_samples {
    //    buf.push_back(rx.recv().expect("failed to build buffer"));
    //}

    let default_out = default_output_device().expect("Error finding default output");
    let event_loop_out = EventLoop::new();
    let out_format = default_out.default_output_format().expect("Error getting default output format");
    let output = event_loop_out.build_output_stream(&default_out, &out_format)
            .expect("Failed to build output stream");
    event_loop_out.play_stream(output);

    thread::sleep(std::time::Duration::from_millis(DELAY));
    event_loop_out.run(move |_, stream_data| {
        //Output audio in u16, i16, or f32 format
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    match buf.lock().unwrap().pop_front() {
                        Some(i) => *elem = (-i * amp) as u16,
                        None => *elem = 0, //Silently drop the sample if buffer is empty 
                    }
                    //buf.push_back(rx.recv().expect("receive failed on push in event loop"));
                    //*elem = -buf.pop_front().expect("pop failed in u16") as u16;
                    //*elem = -rx.recv().expect("pop failed in u16") as u16;
                    //println!("output {:?}", elem);
                }
                println!("Output u16");
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    match buf.lock().unwrap().pop_front() {
                        Some(i) => *elem = (-i * amp) as i16,
                        None => *elem = 0_i16,
                    }
                    //buf.push_back(rx.recv().expect("receive failed on push in event loop"));
                    //*elem = -buf.pop_front().expect("pop failed in i16") as i16;
                    //*elem = -rx.recv().expect("pop failed in u16") as i16;
                    //println!("output {:?}", elem);
                }
                println!("Output i16");
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    match buf.lock().unwrap().pop_front() {
                        Some(i) => *elem = -i * amp,
                        None => *elem = 0.0_f32,
                    }
                    //buf.push_back(rx.recv().expect("receive failed on push in event loop"));
                    //*elem = -buf.pop_front().expect("pop failed in f32 section");
                    //*elem = -rx.recv().expect("pop failed in u16");
                    //println!("output {:?}", elem);
                }
                println!("output f32");
            },
            _ => println!("no output"),
        };
    });
}
