pkgname=code-weaver
pkgver=0.1.0
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
  cd "$pkgname-$pkgver" || return
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$pkgname-$pkgver" || return
  cargo build --release
}

package() {
  cd "$pkgname-$pkgver" || return
  install -Dm755 "target/release/$pkgname" \
    "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE \
    "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
