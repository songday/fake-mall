use async_std::task;
use fake_mall;

fn main() -> Result<(), std::io::Error> {
    task::block_on(fake_mall::web::start())
}
