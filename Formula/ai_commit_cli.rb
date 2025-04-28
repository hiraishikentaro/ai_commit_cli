class AiCommitCli < Formula
  desc "AI-powered commit message generator"
  homepage "https://github.com/hiraishikentaro/ai_commit_cli"
  url "https://github.com/hiraishikentaro/ai_commit_cli/archive/refs/tags/v0.0.1.tar.gz"
  version "0.0.1"
  sha256 "4b1ad2b58490142788826b72d369c93d3f2bd77c6320ba0df1662e8677862c46"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    # Install both binaries
    bin.install "target/release/ai_commit_cli"
    bin.install_symlink bin/"ai_commit_cli" => "aic"
  end

  test do
    system "#{bin}/aic", "--version"
    system "#{bin}/ai_commit_cli", "--version"
  end
end
