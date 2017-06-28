// This is an example API in ECMAScript used for integration tests.
// It is not intended to ever execute.

function undocumented_helper() {
    console.log("I'm helping!");
}

/// Orphaned doc comment
/// This is useless

/// Reports API version
function report_version() {
    console.log("1.0.0");
}

/// Reports commit hash
/// Not valid in debug builds
function report_commit_hash() {
    console.log("abcde");
}

/**
    Reports release date
    Not valid in debug builds
*/
function report_release_date() {
    console.log("Coming soon!");
}