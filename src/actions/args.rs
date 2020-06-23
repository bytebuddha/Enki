use clap::{clap_app, App, AppSettings};

pub const HELP_TEMPLATE: &str = "USAGE:

    {usage}

FLAGS:
{flags}

SUBCOMMANDS:
{subcommands}
";

pub fn get_arguments<'a>() -> App<'a, 'a> {
    clap_app! (enki =>
        (usage: "<COMMAND> <ARGS>\n    or\n    -- echo \"run shell commands\"")
        (setting: AppSettings::NoBinaryName)
        (setting: AppSettings::SubcommandsNegateReqs)
        (global_setting: AppSettings::ColorNever)
        (global_setting: AppSettings::VersionlessSubcommands)
        (template: HELP_TEMPLATE)
        (@arg cmd: +raw +takes_value )
        (@subcommand view =>
            (setting: AppSettings::SubcommandRequiredElseHelp)
            (template: HELP_TEMPLATE)
            (@arg view: -v --view +takes_value "The view id to peform the operation")
            (@subcommand cursor =>
                (template: HELP_TEMPLATE)
                (setting: AppSettings::SubcommandRequiredElseHelp)
                (@subcommand up => )
                (@subcommand down => )
                (@subcommand left => )
                (@subcommand right => )
                (@subcommand pageup => )
                (@subcommand pagedown => )
                (@subcommand home => )
                (@subcommand end => )
                (@subcommand backspace => )
                (@subcommand delete => )
            )
            (@subcommand save =>
                (template: HELP_TEMPLATE)
                (@arg file_name: -f --file +takes_value "The file name to save the file as")
            )
            (@subcommand lang =>
                (template: HELP_TEMPLATE)
                (@arg language: -l --language +takes_value +required "The language to set")
            )
            (@subcommand find =>
                (template: HELP_TEMPLATE)
                (@arg query: +required +takes_value     "The thing to search for")
                (@arg previous: -p --previous conflicts_with[next] "Move to the previous find result")
                (@arg next: -n --next conflicts_with[previous] "Move to the next find result")
                (@arg regex: -r --regex "Use REGEX search")
                (@arg case: -c --case "Case Sensitive")
                (@arg words: -w --words "Search whole words")
            )
            (@subcommand insert =>
               (@arg chars: +required +takes_value "The text to insert")
            )
        )
        (@subcommand settings =>
            (template: HELP_TEMPLATE)
            (setting: AppSettings::SubcommandRequiredElseHelp)
            (@subcommand get =>
                (about: "Get the value of a configuration key")
                (template: HELP_TEMPLATE)
                (@arg key: -k --key +required +takes_value "The configuration key to fetch")
            )
            (@subcommand set =>
                (about: "Set the value of a configuration key")
                (template: HELP_TEMPLATE)
                (@arg key: -k --key +required +takes_value "The configuration key to set")
                (@arg value: -v --value +required +takes_value "The value to set for the given key")
            )
            (@subcommand bind =>
                (about: "Bind an event to a list of actions")
                (template: HELP_TEMPLATE)
                (@arg event: -e --event +required +takes_value +multiple "The event to bind too")
                (@arg actions: +raw +required "A list of actions to execute on the event")
            )
            (@subcommand unbind =>
                (about: "Unbind a Event -> Actions binding")
                (template: HELP_TEMPLATE)
                (@arg event: -e --event +required +takes_value +multiple "The event to remove the current binding from")
            )
        )
        (@subcommand editor =>
           (template: HELP_TEMPLATE)
           (about: "Manage the editor")
           (setting: AppSettings::SubcommandRequiredElseHelp)
           (@subcommand open =>
              (about: "Open a new view in editor")
              (template: HELP_TEMPLATE)
              (@arg file_name: -f --file +takes_value "The file to open.")
           )
           (@subcommand views =>
                (about: "List all views in the editor")
                (template: HELP_TEMPLATE)
                (@arg next: -n --next conflicts_with[previous] "Switch to the next view")
                (@arg previous: -p --previous conflicts_with[next] "Switch to the previous view")
           )
           (@subcommand languages =>
                (about: "List all languages available to the editor")
                (template: HELP_TEMPLATE)
           )
           (@subcommand plugins =>
                (about: "List all languages available to the editor")
                (template: HELP_TEMPLATE)
           )
           (@subcommand themes =>
                 (about: "List all themes available to the editor.")
                 (template: HELP_TEMPLATE)
                 (@arg theme: -s --set +takes_value "The theme to set")
           )
        )
        (@subcommand ui =>
             (template: HELP_TEMPLATE)
             (setting: AppSettings::SubcommandRequiredElseHelp)
             (template: HELP_TEMPLATE)
             (@subcommand debug =>
                 (about: "Toggle the debug display widget")
                 (template: HELP_TEMPLATE)
                 (@arg show: -s --show conflicts_with[hide] "Show the debug widget")
                 (@arg hide: -r --remove conflicts_with[show] "Hide the debug widget")
             )
             (@subcommand prompt =>
                 (template: HELP_TEMPLATE)
                 (about: "Toggle the prompt widget")
                 (@arg show: -s --show conflicts_with[hide] "Show the Action Prompt")
                 (@arg hide: -r --remove conflicts_with[show] "Hide the Action Prompt")
             )
        )
        (@subcommand quit =>
             (template: HELP_TEMPLATE)
        )
    )
}
