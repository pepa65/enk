mod crypto;
use anyhow::Context;
use clap::Parser;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use console::Key;
use std::{
	fs,
	io::{Read, Write, stderr, stdout},
	path::PathBuf,
};

const STYLE: Styles = Styles::styled()
	.header(AnsiColor::Green.on_default().effects(Effects::BOLD))
	.usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
	.literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
	.placeholder(AnsiColor::Cyan.on_default())
	.error(AnsiColor::Red.on_default().effects(Effects::BOLD))
	.valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
	.invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD));

#[derive(Debug, Parser)]
#[command(version, about, styles = STYLE, help_template(
	"\
{before-help}{name} {version} - {about}
{usage-heading} {usage}
{all-args}{after-help}
"
))]
struct Args {
	/// Input file (omit to read from stdin)
	#[arg()]
	file: Option<PathBuf>,
	/// Decrypt [default: encrypt]
	#[arg(short, long)]
	decrypt: bool,
	/*/// Output file
	#[arg(short, long)]
	output: Option<PathBuf>,*/
	/// Use a file as encryption/decryption key
	#[arg(short, long)]
	keyfile: Option<PathBuf>,
	/// Password as argument (instead of prompting)
	#[arg(short, long)]
	password: Option<String>,
	/// Remove input file
	#[arg(short, long)]
	remove: bool,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	// get data
	let input_data = if let Some(file) = &args.file {
		fs::read(file).context("read input file")?
	} else {
		let mut data = Vec::new();
		let mut stdin = std::io::stdin().lock();
		stdin.read_to_end(&mut data).context("read stdin")?;
		data
	};

	// get key
	let key = if let Some(raw_key) = args.password {
		crypto::Key::new(raw_key)?
	} else if let Some(key_file) = args.keyfile {
		let key_data = fs::read(key_file).context("read key file")?;
		crypto::Key::new(key_data)?
	} else {
		let raw_key = prompt("Password: ")?;
		crypto::Key::new(raw_key)?
	};

	// perform encryption/decryption
	let output_data = if args.decrypt { crypto::decrypt(&input_data, key)? } else { crypto::encrypt(&input_data, key)? };

	// output
	stdout().write_all(&output_data).context("write to stdout")?;
	stdout().flush().context("flush stdout")?;

	if args.remove
		&& let Some(file) = &args.file
	{
		std::fs::remove_file(file).context("remove input file")?;
	}
	Ok(())
}

fn prompt(prompt_text: &str) -> anyhow::Result<String> {
	let mut input = String::new();
	eprint!("{prompt_text}");
	stderr().flush().context("flush stdout")?;
	loop {
		let term = console::Term::stderr();
		let character = term.read_key().context("read key from terminal")?;
		match character {
			Key::Enter => {
				eprintln!();
				break;
			}
			Key::Char(c) => {
				input.push(c);
				eprint!("*");
				stderr().flush().context("flush stdout")?;
			}
			Key::Backspace => {
				if !input.is_empty() {
					term.clear_chars(1).context("clear character")?;
				}
				input.pop();
			}
			_other_key => continue,
		};
	}
	Ok(input)
}
