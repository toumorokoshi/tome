class Tome < Formula
  HOMEPAGE = "https://github.com/toumorokoshi/tome"

  desc "Modern replacement for 'sub'"
  homepage HOMEPAGE
  url HOMEPAGE
  license "MIT"
  version '0.1.0'
  revision 1

  head "#{HOMEPAGE}.git"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end
end
