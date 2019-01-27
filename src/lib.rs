use crates_index::*;
use std::fs;

pub fn app_main() -> std::io::Result<()> {
    // 有効なcrateを抽出します
    let crates_index = crates_index::Index::new("crates.io-index");
    let crates: Vec<Crate> = crates_index
        .crates()
        .filter(|c| {
            // yankされてたらはずす
            !c.latest_version().is_yanked() &&
            // リリースしてないパッケージはワイルドカード指定できないので外す
                !c.latest_version().version().contains('-')
        })
        .collect();

    // Cargoのパースライブラリは使い物にならないので文字列処理で連結します
    let cargo_template_toml = fs::read_to_string("rust-crate-acme-everything/Cargo.template.toml")?;
    let cargo_toml = format!(
        "{}{}\n",
        cargo_template_toml,
        crates
            .into_iter()
            .map(|c| format!("{} = \"*\"", c.name()))
            .collect::<Vec<String>>()
            .join("\n")
    );
    fs::write("rust-crate-acme-everything/Cargo.toml", cargo_toml)?;

    Ok(())
}
