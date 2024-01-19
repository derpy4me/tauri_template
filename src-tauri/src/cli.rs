use api::cli;
use tauri::api;

pub fn match_cli_args(matches: cli::Matches) {
    let args = matches.args;
    let keys = args.keys();
    // if keys.any(|s| s == "test") {
    //     show_testing_message();
    // }
    for key in keys {
        if key == "test" {
            let test_data = args.get(key).unwrap();
            if test_data.value == true {
                show_testing_message();
            }
        }
    }
}

fn show_testing_message() {
    println!("Testing arg received");
}
