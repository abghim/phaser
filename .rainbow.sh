phaser() {
	if [[ -d "./target/release/" ]]; then
		printf ""
	else
		printf '%b\n' "$(phaser ff0000 ff7f00 10 "")$(phaser ff7f00 ffff00 10 "")$(phaser ffff00 00ff00 10 "")$(phaser 00ff00 00ffff 10 "")$(phaser  00ffff 0000ff 10 "")$(phaser 0000ff 8b00ff 10 "")$(phaser 8b00ff ff00ff 10 "")"
	fi

	./target/release/phaser "$@"
}

printf '%b\n' "$(phaser ff0000 ff7f00 10 "")$(phaser ff7f00 ffff00 10 "")$(phaser ffff00 00ff00 10 "")$(phaser 00ff00 00ffff 10 "")$(phaser  00ffff 0000ff 10 "")$(phaser 0000ff 8b00ff 10 "")$(phaser 8b00ff ff00ff 10 "")"
