#[derive(Debug)]
pub enum Error {
    ClientCode(u32),
    ClientCodeWithOrigin(u32, u32),
}
