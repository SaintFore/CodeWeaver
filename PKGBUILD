pkgname=code-weaver
_reponame=CodeWeaver
pkgver=0.1.6
pkgrel=1
pkgdesc="A tool to weave context for AI"
arch=("x86_64")
license=("MIT")
url="https://github.com/SaintFore/CodeWeaver"
depends=("gcc-libs")
makedepends=("rust")

source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$_reponame-$pkgver" || return
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$_reponame-$pkgver" || return
  cargo build --release
}

package() {
  cd "$_reponame-$pkgver" || return
  install -Dm755 "target/release/$pkgname" \
    "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE \
    "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
