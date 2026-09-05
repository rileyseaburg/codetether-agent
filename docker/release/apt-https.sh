#!/bin/sh
# Configure official apt repositories without changing third-party mirrors.
# An alternate apt directory supports offline fixture checks.
set -eu
apt_dir=${1:-/etc/apt}
for source in "$apt_dir/sources.list" "$apt_dir/sources.list.d/"*.list "$apt_dir/sources.list.d/"*.sources; do
  [ -f "$source" ] || continue
  sed -E -i \
    's#(^|[[:space:]])http://(archive\.ubuntu\.com|security\.ubuntu\.com|ports\.ubuntu\.com|deb\.debian\.org|security\.debian\.org)(:80)?(/|[[:space:]]|$)#\1https://\2\4#g' \
    "$source"
done
mkdir -p "$apt_dir/apt.conf.d"
cat > "$apt_dir/apt.conf.d/80-codetether-transport" <<'APT'
Acquire::Retries "3";
Acquire::http::Timeout "20";
Acquire::https::Timeout "20";
Acquire::ForceIPv4 "true";
APT::Update::Error-Mode "any";
APT
