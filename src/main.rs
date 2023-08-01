mod helper;
mod r0;

pub use r0::RingBuf0;
use structopt::{clap::arg_enum, StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "2097152")]
    buffer_capacity: usize,
    #[structopt(short, long, default_value = "1000")]
    enqueue_count: usize,
    #[structopt(short, long, default_value = "500000")]
    loop_count: usize,
    #[structopt(short, long, default_value = "R0")]
    ringbuf: RingBufType,
}

arg_enum! {
    #[derive(Debug)]
    enum RingBufType {
        R0,
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt.ringbuf {
        RingBufType::R0 => {
            let mut ringbuf: RingBuf0<u32> = RingBuf0::<u32>::with_capacity(opt.buffer_capacity);
            let start = std::time::Instant::now();
            for _ in 0..opt.loop_count {
                for i in 0..opt.enqueue_count {
                    ringbuf.enqueue(i as u32);
                }
                for _ in 0..opt.enqueue_count {
                    ringbuf.dequeue();
                }
            }
            let end = std::time::Instant::now();
            let ms = (end - start).as_millis() as usize;
            let count = opt.enqueue_count * opt.loop_count * 2;
            println!("{} ops/ms {} enqueue in {} ms", count / ms, count, ms);
        }
    }
}
