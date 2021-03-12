class Tome < Formula
  HOMEPAGE = "https://github.com/zph/tome"

  desc "Modern replacement for 'sub'"
  homepage HOMEPAGE
  url HOMEPAGE
  license "MIT"
  revision 2
  version '0.1.0'

  head "#{HOMEPAGE}.git"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    #   bash_completion.install "completions/completions.bash" => "exa"
    #   zsh_completion.install  "completions/completions.zsh"  => "_exa"
    #   fish_completion.install "completions/completions.fish" => "exa.fish"
  end

  test do
    # (testpath/"test.txt").write("")
    # assert_match "test.txt", shell_output("#{bin}/exa")
  end
end
