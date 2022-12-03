use took::Timer;

struct Job {
    name: &'static str,
    func: fn(),
}

fn jobs() -> &'static [&'static Job] {
    &[&Job {
        name: "day01",
        func: day01::main,
    },&Job {
        name: "day02",
        func: day02::main,
    },&Job {
        name: "day03",
        func: day03::main,
    }]
}

fn main() {
    let timer = Timer::new();
    jobs().iter().for_each(|j| {
        let job_timer = Timer::new();
        (j.func)();
        job_timer.took().describe(j.name)
    });
    timer.took().describe("everything");
}
