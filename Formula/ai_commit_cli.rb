class AiCommitCli < Formula
  desc "AI-powered Git commit message generator"
  homepage "https://github.com/hiraishikentaro/ai_commit_cli"
  url "https://github.com/hiraishikentaro/ai_commit_cli/archive/refs/tags/v0.0.2.tar.gz"
  sha256 "0019dfc4b32d63c1392aa264aed2253c1e0c2fb09216f8e2cc269bbfb8bb49b5"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test that the binary exists
    assert_path_exists "#{bin}/ai_commit_cli"

    # Test version output
    assert_match "ai_commit_cli #{version}", shell_output("#{bin}/ai_commit_cli --version")

    # Test help output
    assert_match "Usage:", shell_output("#{bin}/ai_commit_cli --help")

    # Test basic functionality (create a test git repository)
    system "git", "init", "test-repo"
    cd "test-repo" do
      touch "test.txt"
      system "git", "add", "test.txt"
      # Test that the command runs without error
      system "#{bin}/ai_commit_cli"
    end
  end
end
