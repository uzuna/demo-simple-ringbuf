mod helper;
mod r0;
mod r1;
mod r2;
#[macro_use]
mod r3;

use std::{fmt::Display, str::FromStr, thread::spawn, vec};

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
        let producer = core_ids.pop().unwrap();
        let consumer = core_ids.pop().unwrap();
        Self { producer, consumer }
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

impl Display for CorePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.producer.id, self.consumer.id)
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
    #[structopt(short, long, default_value = "R0S", possible_values = &RingBufType::variants(), case_insensitive = true)]
    ringbuf: RingBufType,
    #[structopt(short, long)]
    cores: Option<CorePair>,
}

arg_enum! {
    #[derive(Debug)]
    enum RingBufType {
        R0S,
        R1S,
        R2S,
        R2M,
        R3S,
        R3M,
    }
}

fn bench_single_thread<R: RingBufTrait<i32>>(rb: &mut R, opt: &Opt) -> String {
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
    format!("{} ops in {:5} ms  {:9} ops/ms", count, ms, count / ms)
}

fn bench_single_thread_pc<P: RingBufProducer<i32>, C: RingBufConsumer<i32>>(
    p: P,
    c: C,
    opt: &Opt,
) -> String {
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
    format!("{} ops in {:5} ms  {:9} ops/ms", count, ms, count / ms)
}

fn bench_multi_thread_pc<
    P: RingBufProducer<i32> + Send + 'static,
    C: RingBufConsumer<i32> + Send + 'static,
>(
    p: P,
    c: C,
    opt: &Opt,
) -> String {
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
            let mut count = enqueue_count;
            while 0 < count {
                if p.enqueue(count as i32) {
                    count -= 1;
                }
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
            let mut count = enqueue_count;
            while 0 < count {
                if c.dequeue().is_some() {
                    count -= 1;
                }
            }
        }
    });
    h_p.join().unwrap();
    h_c.join().unwrap();
    let end = std::time::Instant::now();
    let ms = (end - start).as_millis() as usize;
    let count = opt.enqueue_count * opt.loop_count * 2;
    format!("{} ops in {:5} ms  {:9} ops/ms", count, ms, count / ms)
}

fn main() {
    let opt = Opt::from_args();

    let result = match opt.ringbuf {
        RingBufType::R0S => {
            let mut ringbuf = RingBuf0::<i32>::with_capacity(opt.buffer_capacity);
            bench_single_thread(&mut ringbuf, &opt)
        }
        RingBufType::R1S => {
            let mut ringbuf = RingBuf1::<i32>::with_capacity(opt.buffer_capacity);
            bench_single_thread(&mut ringbuf, &opt)
        }
        RingBufType::R2S => {
            let (p, c, _) = r2::make::<i32>(opt.buffer_capacity);
            bench_single_thread_pc(p, c, &opt)
        }
        RingBufType::R2M => {
            let (p, c, _) = r2::make::<i32>(opt.buffer_capacity);
            bench_multi_thread_pc(p, c, &opt)
        }
        RingBufType::R3S => {
            let (p, c, _) = r3::make::<i32>(opt.buffer_capacity);
            bench_single_thread_pc(p, c, &opt)
        }
        RingBufType::R3M => {
            let (p, c, _b) = r3::make::<i32>(opt.buffer_capacity);
            // _b.as_ref().show_cache();
            bench_multi_thread_pc(p, c, &opt)
        }
    };
    println!(
        "Run {} {}: {result}",
        opt.ringbuf,
        if opt.cores.is_some() {
            format!("{}", opt.cores.unwrap())
        } else {
            "     ".to_string()
        }
    );
}

#[cfg(test)]
mod tests {
    use crate::helper::{RingBufConsumer, RingBufProducer, RingBufTrait};

    fn check_ringbuf<R: RingBufTrait<i32>>(mut ringbuf: R) {
        for i in 0..10 {
            ringbuf.enqueue(i);
        }
        for i in 0..10 {
            assert_eq!(ringbuf.dequeue(), Some(i));
        }
    }
    fn check_pc<P: RingBufProducer<i32>, C: RingBufConsumer<i32>>(p: P, c: C) {
        for i in 0..10 {
            p.enqueue(i);
        }
        for i in 0..10 {
            assert_eq!(c.dequeue(), Some(i));
        }
    }
    #[test]
    fn test_queue() {
        let cap = 10;
        check_ringbuf(crate::r0::RingBuf::<i32>::with_capacity(cap));
        check_ringbuf(crate::r1::RingBuf::<i32>::with_capacity(cap));
        let (p, c, _) = crate::r2::make::<i32>(cap);
        check_pc(p, c);
        let (p, c, _) = crate::r3::make::<i32>(cap);
        check_pc(p, c);
    }
}
