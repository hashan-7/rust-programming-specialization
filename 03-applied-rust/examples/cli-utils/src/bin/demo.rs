use cli_utils::colors::*;

fn main() {
    println!("\n========================================");
    println!("{}", bold("   🦀 Welcome to CLI-Utils Demo! 🦀   "));
    println!("========================================\n");

    println!("{}", red("🔴 [ERROR]   This is a colored error message in red!"));
    println!("{}", green("🟢 [SUCCESS] This is a success message in green!"));
    println!("{}", blue("🔵 [INFO]    This is an informational message in blue!"));
    println!("{}", bold("⚡ [BOLD]    This is bold emphasized text!"));
    println!("{}", reset("⚪ [NORMAL]  Back to normal standard text.\n"));

    println!("========================================");
    println!("{}", green("✨ CLI-Utils demo executed successfully! ✨"));
    println!("========================================\n");
}
