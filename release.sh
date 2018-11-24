#!/bin/bash
########################################################
# Script to build and package a release                #
# usage: ./release.sh                                  #
########################################################

# Fail on the first error, rather than continuing
set -e

VERSION=$(cat Cargo.toml | sed -n -e 's/^.*version = "//p'  | sed -e 's/^"//' -e 's/"$//')
FILENAME="releases/kuberoutes-${VERSION}.tar.gz"

cargo build --release
rm -rf releases
mkdir -p releases
tar -czf $FILENAME --directory=target/release kuberoutes
SHA256=$(shasum -a 256 $FILENAME | cut -d " " -f 1)

FORMULA=$(cat <<EOM
class Kuberoutes < Formula
  desc "Tool to visualise network policies in Kubernetes with kubectl"
  homepage "https://github.com/atkinchris/kuberoutes"
  url "https://github.com/atkinchris/kuberoutes/releases/download/v$VERSION/kuberoutes-$VERSION.tar.gz"
  sha256 "$SHA256"

  bottle :unneeded

  depends_on "kubernetes-cli"

  def install
    bin.install "kuberoutes"
  end

  test do
    system "#{bin}/kuberoutes", "--version"
  end
end
EOM
)

echo "$FORMULA" > releases/kuberoutes.rb

git tag --force "v${VERSION}"
git push --tags
