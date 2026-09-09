mod crypto;

use anyhow::{Context, Result};
use clap::Parser;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use std::{
	fs,
	io::{BufRead, Read, Write, stderr, stdin, stdout},
	path::PathBuf,
};
use zeroize::Zeroizing;

const STYLE: Styles = Styles::styled()
	.header(AnsiColor::Green.on_default().effects(Effects::BOLD))
	.usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
	.literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
	.placeholder(AnsiColor::Cyan.on_default())
	.error(AnsiColor::Red.on_default().effects(Effects::BOLD))
	.valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
	.invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD));

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    styles = STYLE,
    help_template(
        "\
{before-help}{name} {version} - {about}
{usage-heading} {usage}
{all-args}{after-help}
"
    )
)]
struct Args {
	/// Input file (read from stdin if not given)
	#[arg()]
	file: Option<PathBuf>,
	/// Decrypt [default: encrypt]
	#[arg(short, long)]
	decrypt: bool,
	/// Use file as the secret
	#[arg(short, long)]
	keyfile: Option<PathBuf>,
	/// Read password from stdin instead of prompting
	#[arg(short = 'p', long)]
	password: bool,
}

fn main() -> Result<()> {
	let args = Args::parse();
	if args.password && args.file.is_none() {
		anyhow::bail!("-p/--password requires an input file");
	}

	if args.password && args.keyfile.is_some() {
		anyhow::bail!("Cannot use both -p/--password and -k/--keyfile");
	}
	let input_data = read_input(args.file.as_ref())?;
	let secret = if let Some(keyfile) = args.keyfile.as_ref() {
		read_keyfile(keyfile)?
	} else if args.password {
		read_password_stdin()?
	} else {
		prompt("Password: ")?
	};
	let output_data = if args.decrypt { crypto::decrypt(&input_data, &secret)? } else { crypto::encrypt(&input_data, &secret)? };
	stdout().write_all(&output_data).context("write output")?;
	stdout().flush().context("flush stdout")?;
	Ok(())
}

fn read_input(file: Option<&PathBuf>) -> Result<Vec<u8>> {
	if let Some(file) = file {
		fs::read(file).with_context(|| format!("read input file '{}'", file.display()))
	} else {
		let mut data = Vec::new();
		let mut stdin = std::io::stdin().lock();
		stdin.read_to_end(&mut data).context("read stdin")?;
		Ok(data)
	}
}

fn read_keyfile(file: &PathBuf) -> Result<Zeroizing<Vec<u8>>> {
	let data = fs::read(file).with_context(|| format!("read key file '{}'", file.display()))?;
	if data.is_empty() {
		anyhow::bail!("keyfile is empty");
	}

	Ok(Zeroizing::new(data))
}

fn read_password_stdin() -> Result<Zeroizing<Vec<u8>>> {
	let mut password = Zeroizing::new(Vec::new());
	stdin().lock().read_until(b'\n', &mut password).context("read password from stdin")?;
	if password.last() == Some(&b'\n') {
		password.pop();
		if password.last() == Some(&b'\r') {
			password.pop();
		}
	}
	if password.is_empty() {
		anyhow::bail!("password is empty");
	}

	Ok(password)
}

fn prompt(prompt_text: &str) -> Result<Zeroizing<Vec<u8>>> {
	eprint!("{prompt_text}");
	stderr().flush().context("flush stderr")?;
	let password = console::Term::stderr().read_secure_line().context("read password")?;
	if password.is_empty() {
		anyhow::bail!("password is empty");
	}

	Ok(Zeroizing::new(password.into_bytes()))
}
