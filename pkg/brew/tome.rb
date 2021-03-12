class Tome < Formula
  desc "Modern replacement for 'sub'"
  homepage "https://the.exa.website"
  url "https://github.com/zph/tome"
  sha256 "96e743ffac0512a278de9ca3277183536ee8b691a46ff200ec27e28108fef783"
  license "MIT"
  revision 2
  version '0.1.0'

  livecheck do
    url :stable
    strategy :github_latest
  end

  head do
    url "https://github.com/zph/tome.git"
  end

  depends_on "rust" => :build

  # uses_from_macos "zlib"

  # on_linux do
  #   depends_on "libgit2"
  # end

#   # Replace stale lock file. Remove at version bump.
#   resource "Cargo.lock" do
#     url "https://raw.githubusercontent.com/ogham/exa/61c5df7c111fc7451bf6b8f0dfdcb2b6b46577d0/Cargo.lock"
#     sha256 "0bc38c483120874c42b9ada35d13530f16850274cfa8ff1defc1e55bba509698"
#   end

  def install
    # Remove at version bump
    # unless build.head?
    #   rm_f "Cargo.lock"
    #   resource("Cargo.lock").stage buildpath
    # end

    system "cargo", "install", *std_cargo_args

    # # Remove in 0.9+
    # if build.head?
    #   bash_completion.install "completions/completions.bash" => "exa"
    #   zsh_completion.install  "completions/completions.zsh"  => "_exa"
    #   fish_completion.install "completions/completions.fish" => "exa.fish"

    #   args = %w[
    #     --standalone
    #     --to=man
    #   ]

    #   system "pandoc", *args, "man/exa.1.md", "-o", "exa.1"
    #   system "pandoc", *args, "man/exa_colors.5.md", "-o", "exa_colors.5"

    #   man1.install "exa.1"
    #   man5.install "exa_colors.5"
    # else
    #   bash_completion.install "contrib/completions.bash" => "exa"
    #   zsh_completion.install  "contrib/completions.zsh"  => "_exa"
    #   fish_completion.install "contrib/completions.fish" => "exa.fish"
    #   man1.install "contrib/man/exa.1"
    # end
  end

  test do
    # (testpath/"test.txt").write("")
    # assert_match "test.txt", shell_output("#{bin}/exa")
  end
end
