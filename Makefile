all: inflator.exe deflator.exe
	@$(MAKE) -C pothan
	@mkdir -p build
	@mkdir -p build/pothan
	@cp inflator.exe build/
	@cp deflator.exe build/
	@cp pothan/pothan.exe build/pothan/
	@cp pothan/shortcut.exe build/pothan/
	@mkdir -p build/App
	@cp pothan/launcher.exe build/App/
	@mkdir -p build/App/modules
	@mkdir -p build/App/xiphos

inflator.exe: inflator.c
	@gcc inflator.c -o inflator.exe
	@echo "inflator.exe built."

deflator.exe: deflator.c
	@gcc deflator.c -o deflator.exe
	@echo "deflator.exe built."

clean:
	@- rm -r build 2>/dev/null || true
	@- rm inflator.exe 2>/dev/null || true
	@- rm deflator.exe 2>/dev/null || true
	@$(MAKE) -C pothan clean
