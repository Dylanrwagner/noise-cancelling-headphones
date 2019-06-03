use cpal::*;
use std::thread;
use std::sync::*;

//Calculate the delay caused by the system, and the amplitude of the sound that leaks
// into the headphones. Return a tuple containing the amplitude and the delay
fn calibrate(def_in: Device, calibrator: Device, def_out: Device) -> (f32, usize) {
    let amplitude: f32 = calc_amplitude(&calibrator);
    let delay: usize = calc_delay(def_in, calibrator, def_out);
    (amplitude, delay)
}

//Find amplitude of sound leaking in to the headphones
fn calc_amplitude(calibrator: &Device) -> f32 {
    let event_loop = EventLoop::new();
    let buf: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    //Idea for atomic bool for tracking whether a thread should continue borrowed
    // from cpal's "record_wav" example.
    //Original: https://github.com/tomaka/cpal/blob/master/examples/record_wav.rs
    let calibrating: Arc<atomic::AtomicBool> = Arc::new(atomic::AtomicBool::new(true));

    let buf_clone = buf.clone();
    let calibrating_clone = calibrating.clone();
    let format = calibrator.default_input_format().expect("Error getting default input format");
    let input = event_loop.build_input_stream(&calibrator, &format)
            .expect("Error building input stream");
    //Collect samples to find amplitude
    event_loop.play_stream(input.to_owned());
    thread::spawn(move || {
        event_loop.run(move |_, stream_data| {
            if !calibrating_clone.load(atomic::Ordering::Relaxed) { return; }
            //catch input in u16, i16, or f32 format
            match stream_data {
                StreamData::Input { buffer: UnknownTypeInputBuffer::U16(buffer) } => {
                    for elem in buffer.iter() {
                        buf_clone.lock().unwrap().push(*elem as f32);
                    }
                },
                StreamData::Input { buffer: UnknownTypeInputBuffer::I16(buffer) } => {
                    for elem in buffer.iter() {
                        buf_clone.lock().unwrap().push(*elem as f32);
                    }
                },
                StreamData::Input { buffer: UnknownTypeInputBuffer::F32(buffer) } => {
                    for elem in buffer.iter() {
                        buf_clone.lock().unwrap().push(*elem);
                    }
                },
                _ => println!("no input"),
            };
        });

    });

    //collect samples for some time
    thread::sleep(std::time::Duration::from_secs(1));
    calibrating.store(false, atomic::Ordering::Relaxed);

    //find peak-to-peak amplitude of recorded sound
    let mut peak_to_peak: f32 = buf.lock().unwrap().iter().fold(-std::f32::INFINITY, |max, &item| {
        if max <= item { item }
        else { max }
    });
    peak_to_peak -= buf.lock().unwrap().iter().fold(std::f32::INFINITY, |min, &item| {
        if min >= item { item }
        else { min }
    });
    peak_to_peak / 2.0_f32.sqrt()
}

fn calc_delay(def_in: Device, cali: Device, def_out: Device) -> usize {
    unimplemented!("calculate delay");
}

/// Find the local maxima of the given vector. Return the numbers and
/// their indices
fn local_maxima(input: Vec<f32>) -> Vec<(f32, usize)> {
    if input.len() == 0 {
        return Vec::new();
    }
    else if input.len() == 1 {
        return [(input[0], 0)].to_vec();
    }
    let mut result = Vec::new();
    if input[0] >= input[1] {
        result.push((input[0], 0));
    }
    for i in 1..input.len()-1 {
        if input[i] >= input[i-1] && input[i] >= input[i+1] {
            result.push((input[i], i));
        }
    }
    if input[input.len()-1] >= input[input.len()-2] {
        result.push((input[input.len()-1], input.len()-1));
    }
    result
}

#[test]
fn test_local_maxima() {
    assert_eq!(local_maxima(Vec::new()), Vec::new());
    assert_eq!(local_maxima([1.0,3.0,2.0].to_vec()), [(3.0, 1)].to_vec());
    assert_eq!(local_maxima([5.0,2.0].to_vec()), [(5.0,0)].to_vec());
    assert_eq!(local_maxima([1.0].to_vec()), [(1.0,0)].to_vec());
    assert_eq!(local_maxima([1.0, 3.0, 3.0, 2.0].to_vec()), [(3.0,1),(3.0,2)].to_vec());
}
