all: pothan.exe launcher.exe shortcut.exe

pothan.exe: pothan/src/main.rs pothan/Cargo.toml
	@cd pothan; cargo build --release
	@echo "pothan.exe built."
	@cp pothan/target/release/pothan.exe ./

launcher.exe : launcher.c
	@gcc -mwindows launcher.c -o launcher.exe
	@echo "launcher.exe built."

clean:
	@- rm pothan.exe 2>/dev/null || true
	@- rm launcher.exe 2>/dev/null || true
	@cd pothan; cargo clean
