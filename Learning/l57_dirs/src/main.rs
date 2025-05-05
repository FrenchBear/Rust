// l57_dirs

#![allow(unused)]

use std::path::PathBuf;

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(mut stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            if stripped.starts_with('/') || stripped.starts_with('\\') {
                stripped = &stripped[1..];
            }
            return home.join(stripped.trim_start_matches('/'));
        }
    }
    PathBuf::from(path)
}

fn main() {
    println!("home_dir:       {:?}", dirs::home_dir());
    println!();
    println!("cache_dir:      {:?}", dirs::cache_dir());
    println!("config_dir:     {:?}", dirs::config_dir());
    println!("data_dir:       {:?}", dirs::data_dir());
    println!("data_local_dir: {:?}", dirs::data_local_dir());
    println!("executable_dir: {:?}", dirs::executable_dir());
    println!("preference_dir: {:?}", dirs::preference_dir());
    println!("runtime_dir:    {:?}", dirs::runtime_dir());
    println!("state_dir:      {:?}", dirs::state_dir());
    println!();
    println!("audio_dir:      {:?}", dirs::audio_dir());
    println!("desktop_dir:    {:?}", dirs::desktop_dir());
    println!("document_dir:   {:?}", dirs::document_dir());
    println!("download_dir:   {:?}", dirs::download_dir());
    println!("font_dir:       {:?}", dirs::font_dir());
    println!("picture_dir:    {:?}", dirs::picture_dir());
    println!("public_dir:     {:?}", dirs::public_dir());
    println!("template_dir:   {:?}", dirs::template_dir());
    println!("video_dir:      {:?}", dirs::video_dir());


    let tilde_path = "~/.config/my_app/config.toml";
    let expanded_path = expand_tilde(tilde_path);
    println!("\nOriginal path:  {}", tilde_path);
    println!("Expanded path:  {}", expanded_path.display());

    let tilde_path = r"~\temp\f1.txt";
    let expanded_path = expand_tilde(tilde_path);
    println!("\nOriginal path:  {}", tilde_path);
    println!("Expanded path:  {}", expanded_path.display());

    let normal_path = "/tmp/some_file.txt";
    let expanded_normal = expand_tilde(normal_path);
    println!("\nOriginal path:  {}", normal_path);
    println!("Expanded path:  {}", expanded_normal.display());
}
