#![allow(unused)]

use clap::{builder::PossibleValue, Arg, ArgAction, Command, ValueHint};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use nu_cli::NuCompleter;
use nu_command::create_default_context;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    Value,
};
use nu_test_support::fs;

use reedline::Suggestion;

const SEP: char = std::path::MAIN_SEPARATOR;

pub fn basic_command(name: &'static str) -> Command {
    Command::new(name)
        .arg(
            Arg::new("config")
                .short('c')
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .conflicts_with("config")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("test")
                .about("Subcommand")
                .arg(Arg::new("debug").short('d').action(ArgAction::Count)),
        )
}

pub fn feature_sample_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            Arg::new("config")
                .action(ArgAction::Count)
                .help("some config file")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .arg(Arg::new("choice").value_parser(["first", "second"]))
        .subcommand(
            Command::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .action(ArgAction::Set)
                    .help("the case to test"),
            ),
        )
}

pub fn special_commands_command(name: &'static str) -> Command {
    feature_sample_command(name)
        .subcommand(
            Command::new("some_cmd")
                .about("tests other things")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .hide(true)
                        .action(ArgAction::Set)
                        .require_equals(true)
                        .help("the other case to test"),
                )
                .arg(Arg::new("path").num_args(1..)),
        )
        .subcommand(Command::new("some-cmd-with-hyphens").alias("hyphen"))
        .subcommand(Command::new("some-hidden-cmd").hide(true))
}

pub fn quoting_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .action(ArgAction::SetTrue)
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .action(ArgAction::SetTrue)
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .action(ArgAction::SetTrue)
                .help("For more information see `echo test`"),
        )
        .arg(
            Arg::new("backslash")
                .long("backslash")
                .action(ArgAction::SetTrue)
                .help("Avoid '\\n'"),
        )
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .action(ArgAction::SetTrue)
                .help("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .action(ArgAction::SetTrue)
                .help("Execute the shell command with $SHELL"),
        )
        .subcommands([
            Command::new("cmd-single-quotes").about("Can be 'always', 'auto', or 'never'"),
            Command::new("cmd-double-quotes").about("Can be \"always\", \"auto\", or \"never\""),
            Command::new("cmd-backticks").about("For more information see `echo test`"),
            Command::new("cmd-backslash").about("Avoid '\\n'"),
            Command::new("cmd-brackets").about("List packages [filter]"),
            Command::new("cmd-expansions").about("Execute the shell command with $SHELL"),
        ])
}

pub fn aliases_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .about("testing nushell completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .action(ArgAction::SetTrue)
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .action(ArgAction::Set),
        )
        .arg(Arg::new("positional"))
}

pub fn sub_subcommands_command(name: &'static str) -> Command {
    feature_sample_command(name).subcommand(
        Command::new("some_cmd")
            .about("top level subcommand")
            .visible_alias("some_cmd_alias")
            .subcommand(
                Command::new("sub_cmd").about("sub-subcommand").arg(
                    Arg::new("config")
                        .long("config")
                        .action(ArgAction::Set)
                        .value_parser([
                            PossibleValue::new("Lest quotes, aren't escaped.")
                                .help("help,with,comma"),
                            PossibleValue::new("Second to trigger display of options"),
                        ])
                        .help("the other case to test"),
                ),
            ),
    )
}

pub fn value_hint_command(name: &'static str) -> Command {
    Command::new(name)
        .arg(
            Arg::new("choice")
                .long("choice")
                .action(ArgAction::Set)
                .value_parser(["bash", "fish", "zsh"]),
        )
        .arg(
            Arg::new("unknown")
                .long("unknown")
                .value_hint(ValueHint::Unknown),
        )
        .arg(Arg::new("other").long("other").value_hint(ValueHint::Other))
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(ValueHint::DirPath),
        )
        .arg(
            Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(ValueHint::ExecutablePath),
        )
        .arg(
            Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(ValueHint::CommandName),
        )
        .arg(
            Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("command_with_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .value_hint(ValueHint::CommandWithArguments),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(ValueHint::Username),
        )
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_hint(ValueHint::Hostname),
        )
        .arg(Arg::new("url").long("url").value_hint(ValueHint::Url))
        .arg(
            Arg::new("email")
                .long("email")
                .value_hint(ValueHint::EmailAddress),
        )
}

pub fn assert_matches_path(
    expected_path: impl AsRef<std::path::Path>,
    gen: impl clap_complete::Generator,
    mut cmd: Command,
    name: &'static str,
) {
    let mut buf = vec![];
    clap_complete::generate(gen, &mut cmd, name, &mut buf);

    snapbox::Assert::new()
        .action_env("SNAPSHOTS")
        .normalize_paths(false)
        .matches_path(expected_path, buf);
}

// creates a new engine with the current path into the completions fixtures folder
fn new_engine() -> (PathBuf, EngineState, Stack) {
    // Target folder inside assets
    let mut dir = fs::root().join("tests");
    dir.push("snapshots");

    let mut dir_str = dir
        .clone()
        .into_os_string()
        .into_string()
        .unwrap_or_default();
    dir_str.push(SEP);

    // Create a new engine with default context
    let mut engine_state = create_default_context();

    // New stack
    let mut stack = Stack::new();

    // Add pwd as env var
    stack.add_env_var(
        "PWD".to_string(),
        Value::String {
            val: dir_str.clone(),
            span: nu_protocol::Span::new(0, dir_str.len()),
        },
    );

    #[cfg(windows)]
    stack.add_env_var(
        "Path".to_string(),
        Value::String {
            val: "c:\\some\\path;c:\\some\\other\\path".to_string(),
            span: nu_protocol::Span::new(0, dir_str.len()),
        },
    );

    #[cfg(not(windows))]
    stack.add_env_var(
        "PATH".to_string(),
        Value::String {
            val: "/some/path:/some/other/path".to_string(),
            span: nu_protocol::Span::new(0, dir_str.len()),
        },
    );

    // Merge environment into the permanent state
    let merge_result = engine_state.merge_env(&mut stack, &dir);
    assert!(merge_result.is_ok());

    (dir, engine_state, stack)
}

// match a list of suggestions with the expected values
pub fn match_suggestions(expected: Vec<String>, suggestions: Vec<Suggestion>) {
    let expected_len = expected.len();
    let suggestions_len = suggestions.len();
    if expected_len != suggestions_len {
        panic!(
            "\nexpected {expected_len} suggestions but got {suggestions_len}: \n\
            Suggestions: {suggestions:#?} \n\
            Expected: {expected:#?}\n"
        )
    }
    expected.iter().zip(suggestions).for_each(|it| {
        assert_eq!(it.0, &it.1.value);
    });
}

pub fn external_completion(file_name: &str) -> NuCompleter {
    // Create a new engine
    let (dir, mut engine_state, mut stack) = new_engine();

    let path = dir.join(file_name);
    let mut buf = Vec::new();
    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Failed to open {}", path.display()));
    file.read_to_end(&mut buf)
        .unwrap_or_else(|_| panic!("Failed to open {}", path.display()));

    let (_, delta) = {
        let mut working_set = StateWorkingSet::new(&engine_state);
        let block = parse(&mut working_set, None, &buf, false);
        assert!(working_set.parse_errors.is_empty());

        (block, working_set.render())
    };

    assert!(engine_state.merge_delta(delta).is_ok());

    // Merge environment into the permanent state
    assert!(engine_state.merge_env(&mut stack, &dir).is_ok());

    let latest_block_id = engine_state.num_blocks() - 1;

    // Change config adding the external completer
    let mut config = engine_state.get_config().clone();
    config.external_completer = Some(latest_block_id);
    engine_state.set_config(&config);

    // Instantiate a new completer
    NuCompleter::new(Arc::new(engine_state), stack)
}
