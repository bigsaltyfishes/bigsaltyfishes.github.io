def main [
    scripts_dir: string,
    assets_dir: string,
    output_dir: string
] {
    # Run the asset compression script
    let compress_script = $scripts_dir | path join "compress_assets.nu"
    if not ($compress_script | path exists) {
        error make {msg: $"Compression script '($compress_script)' does not exist."}
    }
    if (nu $compress_script $assets_dir $output_dir | complete).exit_code != 0 {
        error make {msg: "Failed to compress assets."}
    }

    # Run the index generation script
    let index_script = $scripts_dir | path join "generate_index.nu"
    if not ($index_script | path exists) {
        error make {msg: $"Index generation script '($index_script)' does not exist."}
    }
    if (nu $index_script $assets_dir ($output_dir | path join "assets" "_assets" "articles") | complete).exit_code != 0 {
        error make {msg: "Failed to generate index."}
    }
}