.PHONY: all clean

# Nombre del proyecto
PROJECT = $(project)

# Ruta del directorio del proyecto
SRC_DIR = $(PROJECT)/src

all: $(PROJECT)

$(PROJECT):
	@echo "Creando estructura de directorios para $(PROJECT)..."
	mkdir -p $(SRC_DIR)
	touch $(SRC_DIR)/main.rs
	touch $(PROJECT)/Cargo.toml
	touch $(PROJECT)/README.md
	@echo "[package]" > $(PROJECT)/Cargo.toml
	@echo "name = \"$(PROJECT)\"" >> $(PROJECT)/Cargo.toml
	@echo "version = \"0.1.0\"" >> $(PROJECT)/Cargo.toml
	@echo "edition = \"2021\"" >> $(PROJECT)/Cargo.toml
	@echo "" >> $(PROJECT)/Cargo.toml
	@echo "# $(PROJECT)" > $(PROJECT)/README.md
	@echo "fn main() {" > $(SRC_DIR)/main.rs
	@echo "    println!(\"Hello, $(PROJECT)!\");" >> $(SRC_DIR)/main.rs
	@echo "}" >> $(SRC_DIR)/main.rs
	@echo "Estructura de directorios y archivos para $(PROJECT) creada."

clean:
	rm -rf $(PROJECT)
	@echo "Directorio $(PROJECT) eliminado."