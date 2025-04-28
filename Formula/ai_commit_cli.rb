class AiCommitCli < Formula
  desc "AI Commit CLI"
  homepage ""
  url "https://github.com/hiraishikentaro/ai_commit_cli/archive/refs/tags/v0.0.1.tar.gz"
  version "0.0.1"

  def install
    bin.install "aic"
  end

  test do
    system "#{bin}/aic -version"
  end
end
