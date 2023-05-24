mod common;
use reedline::Completer;

#[test]
fn completion_basic() {
    let mut completer = common::external_completion("basic.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["-c".into(), "-v".into()];
    common::match_suggestions(expected, suggestions);

    let input = "my-app test -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["-c".into(), "-d".into()];
    common::match_suggestions(expected, suggestions);
}

#[test]
fn completion_feature_sample() {
    let mut completer = common::external_completion("feature_sample.nu");

    let input = "my-app test --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--case".into(), "--version".into()];
    common::match_suggestions(expected, suggestions);

    let input = "my-app choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["first".into(), "second".into()];
    common::match_suggestions(expected, suggestions);

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--conf".into(),
        "--config".into(),
        "--version".into(),
        "-C".into(),
        "-V".into(),
        "-c".into(),
    ];
    common::match_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--conf".into(), "--config".into(), "--version".into()];
    common::match_suggestions(expected, suggestions);
}

#[test]
fn completion_special_commands() {
    let mut completer = common::external_completion("special_commands.nu");

    let input = "my-app some";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "my-app some_cmd".into(),
        "my-app some-hidden-cmd".into(),
        "my-app some-cmd-with-hyphens".into(),
    ];
    common::match_suggestions(expected, suggestions);

    let input = "my-app choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["first".into(), "second".into()];
    common::match_suggestions(expected, suggestions);

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--conf".into(),
        "--config".into(),
        "--version".into(),
        "-C".into(),
        "-V".into(),
        "-c".into(),
    ];
    common::match_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--conf".into(), "--config".into(), "--version".into()];
    common::match_suggestions(expected, suggestions);
}

#[test]
fn completion_quoting() {
    let mut completer = common::external_completion("quoting.nu");

    let input = "my-app cmd-s";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["my-app cmd-single-quotes".into()];
    common::match_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--backslash".into(),
        "--backticks".into(),
        "--brackets".into(),
        "--double-quotes".into(),
        "--expansions".into(),
        "--single-quotes".into(),
        "--version".into(),
    ];
    common::match_suggestions(expected, suggestions);
}

#[test]
fn completion_aliases() {
    let mut completer = common::external_completion("aliases.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--flag".into(),
        "--flg".into(),
        "--opt".into(),
        "--option".into(),
        "--version".into(),
        "-F".into(),
        "-O".into(),
        "-V".into(),
        "-f".into(),
        "-o".into(),
    ];
    common::match_suggestions(expected, suggestions);
}

#[test]
fn completion_sub_subcommands() {
    let mut completer = common::external_completion("sub_subcommands.nu");

    let input = "my-app";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "my-app".into(),
        "my-app test".into(),
        "my-app some_cmd".into(),
        "my-app some_cmd sub_cmd".into(),
    ];
    common::match_suggestions(expected, suggestions);

    let input = "my-app some_cmd sub_cmd -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--config".into(), "--version".into(), "-V".into()];
    common::match_suggestions(expected, suggestions);

    let input = "my-app some_cmd sub_cmd --config ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "\"Lest quotes, aren't escaped.\"".into(),
        "\"Second to trigger display of options\"".into(),
    ];
    common::match_suggestions(expected, suggestions);
}

#[test]
fn completion_value_hint() {
    let mut completer = common::external_completion("value_hint.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--choice".into(),
        "--cmd".into(),
        "--cmd-name".into(),
        "--dir".into(),
        "--email".into(),
        "--exe".into(),
        "--file".into(),
        "--host".into(),
        "--other".into(),
        "--path".into(),
        "--unknown".into(),
        "--url".into(),
        "--user".into(),
        "-H".into(),
        "-c".into(),
        "-d".into(),
        "-e".into(),
        "-f".into(),
        "-p".into(),
        "-u".into(),
    ];
    common::match_suggestions(expected, suggestions);

    let input = "my-app --choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["bash".into(), "fish".into(), "zsh".into()];
    common::match_suggestions(expected, suggestions);
}
