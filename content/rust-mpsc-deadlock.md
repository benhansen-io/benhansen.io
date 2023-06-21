+++
date = 2023-06-20
title = "Debugging a Rust MPSC Channel Deadlock"
+++

My son is listening to his favorite song on the tablet I have been building for him. It has an activity that allows him to add effects to the song with the various buttons but he told my spouse that you can't push too many buttons or it might freeze. Why is he afraid to push buttons?

A deadlock... but originally the only behavior I noticed is the tablet made a terrible sound as it repeated a short segment of audio over and over again and it became completely unresponsive until physically switched off.

It only happened several times a day if at all so I ignored it for several weeks while making other improvements. One time it happened while I had the output monitor streaming on my laptop and saw watchdog timeout errors printed (a watchdog timeout in this case is when a low-priority thread never gets scheduled on a CPU indicating the CPU is under excessive load or stuck in a loop). I was happy to see it because I was wondering if the issue was in the hardware of the tablet and software is easier for me to debug so yea!

The watch dog timer error made it clear that the audio backend task was running non-stop. The other core (the ESP32-S3 is dual core) was running the idle task so it appeared that the only blocked task was the audio backend task. This will turn out to be a false assumption.

The first issue was getting a good stack trace. The tablet is built using an ESP32-S3 which has a built-in USB interface for debugging so that turned out to be pretty easy using OpenOCD.


The stack trace showed the stuck audio task was in a call to [try_recv] on a std::sync::mpsc::Receiver:

```
#0  esp_crosscore_isr (arg=0x3fca1560 <reason>) at /home/ben/esp/esp-idf/components/esp_system/crosscore_int.c:91
#1  0x403774b0 in _xt_lowint1 () at /home/ben/esp/esp-idf/components/freertos/port/xtensa/xtensa_vectors.S:1114
#2  0x403819f0 in vTaskDelay (xTicksToDelay=0) at /home/ben/esp/esp-idf/components/hal/esp32s3/include/hal/cpu_ll.h:38
#3  0x42003224 in sched_yield () at /home/ben/esp/esp-idf/components/pthread/pthread.c:466
#4  0x420bf4eb in std::sync::mpmc::list::Channel<T>::try_recv ()
#5  0x420c149f in <awedio::sounds::wrappers::controllable::Controllable<S> as awedio::sound::Sound>::on_start_of_batch ()
#6  0x42018c68 in <awedio::sounds::wrappers::controllable::Controllable<S> as awedio::sound::Sound>::on_start_of_batch ()
#7  0x42018fbe in awedio_esp32::audio_task::h6eb04a8ebc958b4b ()
#8  0x40382bac in vPortTaskWrapper (pxCode=0x42018f50 <_ZN12awedio_esp3210audio_task17h6eb04a8ebc958b4bE.llvm.17939224925020937006>, pvParameters=0x3fcea8c4) at /home/ben/esp/esp-idf/components/freertos/port/xtensa/port.c:142
```

The mpsc channel is used in the [awedio](https://github.com/10buttons/awedio) audio library I developed for the tablet project. It is used to send commands from an application task to the audio rendering task (e.g. pause a sound or change the volume).

Not knowing why the task was stuck in try_recv I tried to make it easier to reproduce the issue. I wasn't able to reproduce the issue very reliably without physically pressing a lot of buttons even if I automated what pushing the buttons did in a loop.

After running out of time for the day, the answer came to me that evening while walking around the playground while my kids were playing.

I remembered that the main task which sends tasks to the audio backend is pinned to a single core. I also remembered that I set the audio thread to a higher priority than the main task to avoid audio buffer underruns. So while I originally thought the main task was idle (e.g. waiting for input), when I got back to my computer, a stack trace confirmed that it was actually paused while sending a command across the channel. The audio task was allowed to be on either core. When I push a button an interrupt is generated causing an interrupt service routine to run. Since an ISR runs with very high priority it could interrupt the main thread right at a time when it was sending a command to the audio thread. Apparently the audio thread can then be resumed on its core since it can run on either core.

So it appears try_recv is blocking on the sender. The stack trace shows that try_recv calls sched_yield which I believe is an attempt to let the sender run again to unblock the receiver. That doesn't work on FreeRTOS if the tasks have different priorities since the higher priority task will always run if it can.

At first I wasn't sure if the issue was with ESP's implementation of the Rust std library (maybe it should call a different function to suspend the receiver task) but reading the docs of `sched_yield` its documented to not yield to tasks of lower priority.

After some thought it seemed possible to be able to reproduce the issue on Linux using a standard Rust toolchain. By pinning two threads to only run on the same core and giving the receiver thread a much higher priority than the sender thread, we reliably see deadlocks after a few seconds in the following code:

```rust
const PINNED_CORE: usize = 2;

let (sender, receiver) = channel::<usize>();

std::thread::Builder::new()
    .name("sending".to_owned())
    .spawn(move || {
        thread_priority::set_current_thread_priority(ThreadPriority::Min).unwrap();
        core_affinity::set_for_current(core_affinity::CoreId { id: PINNED_CORE });

        loop {
            sender.send(42).unwrap();
        }
    })
    .unwrap();

let num_received = Arc::new(std::sync::atomic::AtomicUsize::new(0));

std::thread::Builder::new()
    .name("receiving".to_owned())
    .spawn({
        let num_received = num_received.clone();
        move || {
            thread_priority::set_current_thread_priority(ThreadPriority::Max).unwrap();
            core_affinity::set_for_current(core_affinity::CoreId { id: PINNED_CORE });

            loop {
                let start = Instant::now();
                let try_receive_result = receiver.try_recv();
                let elapsed = start.elapsed();
                if elapsed > Duration::from_secs(1) {
                    println!("try_recv blocked for {:.2} seconds", elapsed.as_secs_f32());
                }
                match try_receive_result {
                    Ok(_) => {
                        num_received.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                    Err(TryRecvError::Empty) => {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(TryRecvError::Disconnected) => unreachable!(),
                }
            }
        }
    })
    .unwrap();

loop {
    std::thread::sleep(Duration::from_millis(500));
    println!(
        "Receiving thread has received {}",
        num_received.load(std::sync::atomic::Ordering::SeqCst)
    )
}
```
&nbsp; &nbsp; ([full code available](https://github.com/benhansen-io/mpsc_deadlock_reproducer))

Example output of code on Linux showing the deadlock:

```
...
Receiving thread has received 95667535
Receiving thread has received 95667535
Receiving thread has received 95667535
Receiving thread has received 95667535
try_recv blocked for 34.72 seconds
Receiving thread has received 95738090
Receiving thread has received 100046892
Receiving thread has received 103967861
...
```

The better stack traces on Linux showed the code deadlocked at a point between where the sender first writes the value and where it writes a flag to indicate the value has been written. Unfortunately if the receiver attempts to read a value when it is being written it *waits* for the flag to say that the value has been fully written before continuing even in the try_recv path.

After reviewing the documentation of [try_recv] which says (emphasis mine):


> Attempts to return a pending value on this receiver **without blocking**.
>
> This method will **never block** the caller in order to wait for data to become available. Instead, this will always return *immediately* with a possible option of pending data on the channel.

I was confident that the behavior does not match the documentation.  I filed [a Rust issue](https://github.com/rust-lang/rust/issues/112723) on GitHub. It is not normal for the Rust standard library to let me down. The conditions to hit the deadlock are rare and it is being addressed. Thanks to everyone who works on Rust and the standard library. It has been a pleasure to use.

Thanks for taking the time to read. If your interested in hearing more about the programmable button tablet I am making for kids in Rust check out [10buttons.com](https://www.10buttons.com) and follow along.

## Some takeaways

* This is my second blog post entry and I found it very helpful for me personally. I started writing this blog post half-way through the investigation and it helped me organizing my thoughts and dig deeper. I might not have written the Linux reproducer code and filed a bug report without writing the blog post. It also provides a nice reflection allowing me to see where I got stuck and hopefully improve my process in the future.
* Having the ability to easily debug an ESP32-S3 using GDB over USB is amazing and helps a lot in debugging non-trivial issues like this.
* In this case it would have been nice if the ESP-IDF FreeRTOS implementation would have moved the audio thread to the IDLE core so the main task could resume on its pinned core when the audio thread calls sched_yield.

[try_recv]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.try_recv
