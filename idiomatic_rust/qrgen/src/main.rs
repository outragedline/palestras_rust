use clap::Parser;
use qrcode::QrCode;

#[derive(Debug, Parser)]
struct Cli {
    data: String,
}

fn main() {
    let cli = Cli::parse();
    let code = QrCode::new(cli.data)
        .unwrap()
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{code}")
}
