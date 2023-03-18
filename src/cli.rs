use clap::{Arg, App, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("thg")
        .about("Capture your thoughts in the terminal")
        .subcommand(SubCommand::with_name("+").about("Add a new thought").arg(Arg::with_name("tags").short("t").long("tags").value_name("TAGS").help("Add tags to the thought")))
        .subcommand(SubCommand::with_name("ls").about("List all thoughts"))
        .subcommand(SubCommand::with_name("rm").about("Remove a thought by ID").arg(Arg::with_name("id").required(true)))
}
