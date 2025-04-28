class AiCommitCli < Formula
  desc "AI-powered commit message generator"
  homepage "https://github.com/hiraishikentaro/ai_commit_cli"
  url "https://github.com/hiraishikentaro/ai_commit_cli/archive/refs/tags/v0.0.1.tar.gz"
  version "0.0.1"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/aic"
  end

  test do
    assert_match "aic #{version}", shell_output("#{bin}/aic --version")
  end
end
