use hermit_engine::run;

fn main() {
    pollster::block_on(run());
}
