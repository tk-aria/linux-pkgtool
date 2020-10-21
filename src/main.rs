#[macro_use]
extern crate clap;

use std::process::Command;
use clap::{App, Arg, SubCommand};

fn main() {

    let app = App::new(crate_name!())
        .version(crate_version!())              // Cargo.tomlのversionを参照する
        .author(crate_authors!())               // Cargo.tomlのauthorsを参照する
        .about(crate_description!())            // Cargo.tomlのdescriptionを参照する
        //.arg(Arg::with_name("pa")               // 位置引数を定義
        //    .help("sample positional argument")     // ヘルプメッセージ
        //    .required(true)                         // この引数は必須であることを定義
        //)
        .arg(Arg::with_name("flg")              // フラグを定義
            .help("sample flag")                // ヘルプメッセージ
            .short("I")                         // ショートコマンド
            .long("import")                     // ロングコマンド
        )
        .arg(Arg::with_name("flge")              // フラグを定義
            .help("sample flag")                // ヘルプメッセージ
            .short("E")                         // ショートコマンド
            .long("export")                     // ロングコマンド
        )
        .arg(Arg::with_name("opt")              // オプションを定義
            .help("sample option")              // ヘルプメッセージ
            .short("o")                         // ショートコマンド
            .long("opt")                        // ロングコマンド
            .takes_value(true)                  // 値を持つことを定義
        )
        .subcommand(SubCommand::with_name("sub")// サブコマンドを定義
            .about("sample subcommand")         // このサブコマンドについて
            .arg(Arg::with_name("subflg")       // フラグを定義
                .help("sample flag by sub")     // ヘルプメッセージ
                .short("f")                     // ショートコマンド
                .long("flag")                   // ロングコマンド
            )
        );

    // 引数を解析
    let matches = app.get_matches();

    // paが指定されていれば値を表示
    if let Some(o) = matches.value_of("flg") {
		//println!("Value for pa: {}", o);
		let mut list_dir = Command::new("ls");

		// Execute `ls` in the current directory of the program.
		list_dir.status().expect("process failed to execute");

		println!();

		// Change `ls` to execute in the root directory.
		list_dir.current_dir("/");

		// And then execute `ls` again but in the root directory.
		list_dir.status().expect("process failed to execute");
    }

    // optが指定されていれば値を表示
    if let Some(o) = matches.value_of("opt") {
        println!("Value for opt: {}", o);
    }

	// flgのON/OFFで表示するメッセージを切り替え
	if matches.is_present("flg") {
		let mut list_dir = Command::new("ls");

		// Execute `ls` in the current directory of the program.
		list_dir.status().expect("process failed to execute");

		println!();

		// Change `ls` to execute in the root directory.
		list_dir.current_dir("/");

		// And then execute `ls` again but in the root directory.
		list_dir.status().expect("process failed to execute");
	}

	if matches.is_present("flge") {
		let mut pwd_cmd = Command::new("pwd");
		pwd_cmd.status().expect("jpge");

		println!();
	}

    println!("flg is {}", if matches.is_present("flg") {"ON"} else {"OFF"});

    // subサブコマンドの解析結果を取得
    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub"); // subが指定されていればメッセージを表示
        // subflgのON/OFFで表示するメッセージを切り替え
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
}