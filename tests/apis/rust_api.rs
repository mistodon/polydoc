// This is an example API in Rust used for integration tests.
// It is not intended to ever execute.

fn undocumented_helper()
{
    println!("I'm helping!");
}

/// Orphaned doc comment
/// This is useless

/// Reports API version
pub fn report_version()
{
    println!("1.0.0");
}

/// Reports commit hash
/// Not valid in debug builds
pub fn report_commit_hash()
{
    println!("abcde");
}

/**
    Reports release date
    Not valid in debug builds
*/
pub fn report_release_date()
{
    println!("Coming soon!");
}