mod util;
mod cli;
mod ability;

use std::thread;
use std::time::Duration;

use indicatif;

use cli::cli_deal::Deal;
use util::{CliError, logger};
use log::{info, error};

fn main() {
    init_log();

    deal().map_or_else(|e| {
        error!("{}", e)
    }, |_| {
        info!("执行成功")
    });
}

fn init_log() {
    logger::init("rucli", |time, level, tag, msg| {
        println!("{} {} {} : {}", time, level, tag, msg)
    });
}

fn deal() -> Result<(), CliError> {
    let deal = Deal::read_args()?;
    // let bar = indicatif::ProgressBar::new(100);
    // println!("开始任务");
    // for i in 0..100 {
    //     deal.deal(i);
    //     bar.inc(1);
    //     // println!("开始任务进度：{}", bar.position());
    // }
    // bar.finish_with_message("done");

    deal.deal()
}
