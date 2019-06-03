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
    //let (tx, rx) = mpsc::channel();

    //set up default input device, which will act as the means to cancel the noise
    let default_in = default_input_device().expect("Error finding default input");

    //A second input device inside the headphones to find some noise-cancelling parameters
    // assumes it, the default input device, and the built-in mic are the only input devices
    let calibrator = input_devices()
        .find(|x| { x.name() != "Built-in Microphone" && *x != default_in})
        .unwrap();

    println!("calibrator name = {}", calibrator.name());
    println!("supported default input formats: ");
    for i in default_in.supported_input_formats().unwrap() {
        println!("\tmax_sample_rate: {:?} min_sample_rate: {:?} data_type {:?}", i.max_sample_rate,
            i.min_sample_rate, i.data_type);
    }

    println!("supported calibrator input formats: ");
    for i in calibrator.supported_input_formats().unwrap() {
        println!("\tmax_sample_rate: {:?} min_sample_rate: {:?} data_type {:?}", i.max_sample_rate,
            i.min_sample_rate, i.data_type);
    }
    //let (in_format, cali_format) = match_rates(default_in, calibrator);
    //println!("Default input: sample rate {:?}, type {:?}", in_format.max_sample_rate, in_format.data_type);
    //println!("Calibrator: sample rate {:?}, type {:?}", cali_format.max_sample_rate, cali_format.data_type);

    //Spawn thread that reads input and sends it to output thread
    //thread::spawn(|| {
    //    let event_loop_in = EventLoop::new();
    //    let in_format = default_in.default_input_format().expect("Error getting default input format");
    //    let input = event_loop_in.build_input_stream(&default_in, &in_format)
    //            .expect("Error building input stream");
    //    event_loop_in.play_stream(input.to_owned());

    //    event_loop_in.run(move |_, stream_data| {
    //        //catch input in u16, i16, or f32 format
    //        match stream_data {
    //            StreamData::Input { buffer: UnknownTypeInputBuffer::U16(buffer) } => {
    //                for elem in buffer.iter() {
    //                    tx.send(*elem as f32).expect("input thread failed on send u16");
    //                }
    //                //println!("input U16");
    //                //println!();
    //            },
    //            StreamData::Input { buffer: UnknownTypeInputBuffer::I16(buffer) } => {
    //                for elem in buffer.iter() {
    //                    tx.send(*elem as f32).expect("input thread failed on send i16");
    //                }
    //                //println!("input I16");
    //                //println!();
    //            },
    //            StreamData::Input { buffer: UnknownTypeInputBuffer::F32(buffer) } => {
    //                for elem in buffer.iter() {
    //                    tx.send(*elem).expect("input thread failed on send f32");
    //                }
    //                //println!("input F32");
    //                //println!();
    //            },
    //            _ => println!("no input"),
    //        };
    //    });
    //});

    //let mut buf: VecDeque<f32> = VecDeque::new(); //Buffer of input samples
    ////Build buffer up to input size
    //for _ in 0..BUF_SIZE {
    //    buf.push_back(rx.recv().expect("failed to build buffer"));
    //}

    //let default_out = default_output_device().expect("Error finding default output");
    //let event_loop_out = EventLoop::new();
    //let out_format = default_out.default_output_format().expect("Error getting default output format");
    //let output = event_loop_out.build_output_stream(&default_out, &out_format)
    //        .expect("Failed to build output stream");
    //event_loop_out.play_stream(output);

    //event_loop_out.run(move |_, stream_data| {
    //    //Output audio in u16, i16, or f32 format
    //    match stream_data {
    //        StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
    //            for elem in buffer.iter_mut() {
    //                match buf.pop_front() {
    //                    Some(i) => *elem = -i as u16,
    //                    None => (), //Silently drop the sample if buffer is empty (not sure why this is happening)
    //                }
    //                //*elem = -buf.pop_front().expect("pop failed in u16") as u16;
    //                //*elem = -rx.recv().expect("pop failed in u16") as u16;
    //                //println!("output {:?}", elem);
    //            }
    //            //println!("Output u16");
    //        },
    //        StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
    //            for elem in buffer.iter_mut() {
    //                match buf.pop_front() {
    //                    Some(i) => *elem = -i as i16,
    //                    None => (),
    //                }
    //                //*elem = -buf.pop_front().expect("pop failed in i16") as i16;
    //                //*elem = -rx.recv().expect("pop failed in u16") as i16;
    //                //println!("output {:?}", elem);
    //            }
    //            //println!("Output i16");
    //        },
    //        StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
    //            for elem in buffer.iter_mut() {
    //                match buf.pop_front() {
    //                    Some(i) => *elem = -i,
    //                    None => (),
    //                }
    //                //*elem = -buf.pop_front().expect("pop failed in f32 section");
    //                //*elem = -rx.recv().expect("pop failed in u16");
    //                //println!("output {:?}", elem);
    //            }
    //            //println!("output f32");
    //        },
    //        _ => println!("no output"),
    //    };
    //    buf.push_back(rx.recv().expect("receive failed on push in event loop"));
    //});
}

//Find sample rates that work for default input and calibrator devices
// returns a tuple for the default and calibrator formats, respectively
//fn match_rates(def_in: Device, cali: Device) -> (SupportedFormat, SupportedFormat) {
//    //Find calibrator formats which the default format can match
//    let mut usable_cali_formats = cali.supported_input_formats()
//        .expect("failed to get supported formats (line 132)")
//        .filter(|x| { 
//        //else { return false; }
//        let in_formats = def_in.supported_input_formats().expect("failed to get supported formats (line 136)");
//        if in_formats
//            .filter(|y| y.max_sample_rate == x.max_sample_rate)
//            .collect::<Vec<SupportedFormat>>()
//            .len() != 0 {
//            return true;
//        }
//        return false;
//    });
//
//    //Find the best usable calibrator format (highest sample rate)
//    let mut max_rate = usable_cali_formats.next().expect("failed to get next usable format");
//    for i in usable_cali_formats {
//        if i.max_sample_rate > max_rate.max_sample_rate {
//            max_rate = i;
//        }
//    }
//
//    let cali_format = max_rate;
//
//    //match selected calibrator rate for default input. find f32
//    let in_format = def_in.supported_input_formats()
//        .expect("failed to get supported formats")
//        .find(|x| {
//        if x.max_sample_rate != cali_format.max_sample_rate { return false }
//        false
//    })
//    .expect("No usable in_format");
//    (in_format, cali_format)
//}
