#!/bin/bash
#o  ./backup.sh
BACKUP_DIR="../backups/starter-kit-backup-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

rsync -av --exclude='target' --exclude='.git' --exclude='*.db' ./ "$BACKUP_DIR/"
cargo tree --depth 0 > "$BACKUP_DIR/dependencies-full.txt"
rustc --version > "$BACKUP_DIR/rustc-version.txt"
solana --version > "$BACKUP_DIR/solana-version.txt" 2>/dev/null

echo "✅ Бэкап создан: $BACKUP_DIR"