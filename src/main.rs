mod helper;
mod r0;
mod r1;
mod r2;
#[macro_use]
mod r3;

use std::{str::FromStr, thread::spawn, vec};

use helper::{RingBufConsumer, RingBufProducer, RingBufTrait};
pub use r0::RingBuf as RingBuf0;
pub use r1::RingBuf as RingBuf1;
use structopt::{clap::arg_enum, StructOpt};

#[derive(Debug, Clone, Copy)]
struct CorePair {
    producer: core_affinity::CoreId,
    consumer: core_affinity::CoreId,
}

impl Default for CorePair {
    fn default() -> Self {
        let mut core_ids = core_affinity::get_core_ids().unwrap();
        let core0 = core_ids.pop().unwrap();
        Self {
            producer: core0,
            consumer: core0,
        }
    }
}

impl FromStr for CorePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err("expect 2 number".to_string());
        }
        let mut part_num = [0_usize; 2];
        for (i, part) in parts.iter().enumerate() {
            let num = part.parse::<usize>().map_err(|e| e.to_string())?;
            part_num[i] = num;
        }
        let core_ids = core_affinity::get_core_ids().unwrap();
        let mut producer = None;
        let mut consumer = None;
        for id in core_ids.iter() {
            if id.id == part_num[0] {
                producer = Some(*id)
            }
            if id.id == part_num[1] {
                consumer = Some(*id)
            }
        }
        if consumer.is_none() {
            return Err(format!("core id {} not found", part_num[1]));
        } else if producer.is_none() {
            return Err(format!("core id {} not found", part_num[0]));
        }
        Ok(Self {
            producer: producer.unwrap(),
            consumer: consumer.unwrap(),
        })
    }
}

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
    #[structopt(short, long)]
    cores: Option<CorePair>,
}

arg_enum! {
    #[derive(Debug)]
    enum RingBufType {
        R0,
        R1,
        R2S,
        R2M,
        R3S,
        R3M,
    }
}

fn bench_single_thread<R: RingBufTrait<i32>>(rb: &mut R, opt: &Opt) {
    let start = std::time::Instant::now();
    for _ in 0..opt.loop_count {
        for i in 0..opt.enqueue_count {
            rb.enqueue(i as i32);
        }
        for _ in 0..opt.enqueue_count {
            rb.dequeue();
        }
    }
    let end = std::time::Instant::now();
    let ms = (end - start).as_millis() as usize;
    let count = opt.enqueue_count * opt.loop_count * 2;
    println!("{:9} ops/ms {} enqueue in {:5} ms", count / ms, count, ms);
}

fn bench_single_thread_pc<P: RingBufProducer<i32>, C: RingBufConsumer<i32>>(p: P, c: C, opt: &Opt) {
    let start = std::time::Instant::now();
    for _ in 0..opt.loop_count {
        for i in 0..opt.enqueue_count {
            p.enqueue(i as i32);
        }
        for _ in 0..opt.enqueue_count {
            c.dequeue();
        }
    }
    let end = std::time::Instant::now();
    let ms = (end - start).as_millis() as usize;
    let count = opt.enqueue_count * opt.loop_count * 2;
    println!("{:9} ops/ms {} enqueue in {:5} ms", count / ms, count, ms);
}

fn bench_multi_thread_pc<
    P: RingBufProducer<i32> + Send + 'static,
    C: RingBufConsumer<i32> + Send + 'static,
>(
    p: P,
    c: C,
    opt: &Opt,
) {
    let _core_ids = core_affinity::get_core_ids().unwrap();
    let CorePair {
        producer: core_p,
        consumer: core_c,
    }: CorePair = if opt.cores.is_some() {
        opt.cores.unwrap()
    } else {
        CorePair::default()
    };

    let start = std::time::Instant::now();
    let loop_count = opt.loop_count;
    let enqueue_count = opt.enqueue_count;
    let h_p = spawn(move || {
        if !core_affinity::set_for_current(core_p) {
            println!("set_for_current failed");
        }
        for _ in 0..loop_count {
            for i in 0..enqueue_count {
                p.enqueue(i as i32);
            }
        }
    });
    let loop_count = opt.loop_count;
    let enqueue_count = opt.enqueue_count;
    let h_c = spawn(move || {
        if !core_affinity::set_for_current(core_c) {
            println!("set_for_current failed");
        }

        for _ in 0..loop_count {
            for _ in 0..enqueue_count {
                c.dequeue();
            }
        }
    });
    h_p.join().unwrap();
    h_c.join().unwrap();
    let end = std::time::Instant::now();
    let ms = (end - start).as_millis() as usize;
    let count = opt.enqueue_count * opt.loop_count * 2;
    println!("{:9} ops/ms {} enqueue in {:5} ms", count / ms, count, ms);
}

fn main() {
    let opt = Opt::from_args();
    println!("Run {:?} {:?}", opt.ringbuf, opt.cores);

    match opt.ringbuf {
        RingBufType::R0 => {
            let mut ringbuf = RingBuf0::<i32>::with_capacity(opt.buffer_capacity);
            bench_single_thread(&mut ringbuf, &opt);
        }
        RingBufType::R1 => {
            let mut ringbuf = RingBuf1::<i32>::with_capacity(opt.buffer_capacity);
            bench_single_thread(&mut ringbuf, &opt);
        }
        RingBufType::R2S => {
            let (p, c, _) = r2::make::<i32>(opt.buffer_capacity);
            bench_single_thread_pc(p, c, &opt);
        }
        RingBufType::R2M => {
            let (p, c, _) = r2::make::<i32>(opt.buffer_capacity);
            bench_multi_thread_pc(p, c, &opt);
        }
        RingBufType::R3S => {
            let (p, c, _) = r3::make::<i32>(opt.buffer_capacity);
            bench_single_thread_pc(p, c, &opt);
        }
        RingBufType::R3M => {
            let (p, c, b) = r3::make::<i32>(opt.buffer_capacity);
            bench_multi_thread_pc(p, c, &opt);
            b.as_ref().show_cache()
        }
    }
}
