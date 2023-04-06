mod fractal;
mod draw;
use fractal::Fractal;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "fractal_generator")]
struct Opt {
    #[structopt(short, long, default_value = "4")]
    depth: u32,

    #[structopt(short, long, default_value = "100")]
    size: usize,

    #[structopt(short = "n", long, default_value = "100")]
    num_points: usize,
}

fn main() {
    let opt = Opt::from_args();
    let fractal = Fractal::new(opt.depth.try_into().unwrap(), opt.size, opt.num_points);
    draw::draw_fractal(&fractal, opt.size);
}

