mod pk;
fn main() {
    // Absolute path
    crate::pk::sound::instrument::clarinet();
    // Relative path
    pk::sound::instrument::clarinet();
}