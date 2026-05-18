use eframe::egui;
use rfd;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// =========================================================================
// --- CLASE: UnidadesLexicas (Diccionario y Especificación de Tokens) ---
// =========================================================================
struct UnidadesLexicas {
    tabla_tokens: HashMap<String, i32>,
}

impl UnidadesLexicas {
    fn new() -> Self {
        let mut m = HashMap::new();
        let tokens = [
            // 1-32: Palabras Reservadas del Lenguaje
            ("auto", 1), ("break", 2), ("case", 3), ("char", 4), ("const", 5),
            ("continue", 6), ("default", 7), ("do", 8), ("double", 9), ("else", 10),
            ("enum", 11), ("extern", 12), ("float", 13), ("for", 14), ("goto", 15),
            ("if", 16), ("int", 17), ("long", 18), ("register", 19), ("return", 20),
            ("short", 21), ("signed", 22), ("sizeof", 23), ("static", 24), ("struct", 25),
            ("switch", 26), ("typedef", 27), ("union", 28), ("unsigned", 29), ("void", 30),
            ("volatile", 31), ("while", 32),
            
            // 100-103: Funciones estándar de <stdio.h>
            ("printf", 100), ("scanf", 101), ("gets", 102), ("puts", 103),
            // 104-105: Funciones estándar de <stdlib.h>
            ("malloc", 104), ("free", 105),
            // 106-109: Funciones estándar de <string.h>
            ("strlen", 106), ("strcpy", 107), ("strcmp", 108), ("strcat", 109),
            // 110-112: Funciones estándar de <math.h>
            ("sqrt", 110), ("pow", 111), ("abs", 112),
            // 113-115: Funciones estándar de <conio.h>
            ("exit", 113), ("getch", 114), ("clrscr", 115), ("NULL", 116),
            
            // Directivas del Preprocesador
            ("#include", 200), ("#define", 201),

            // Símbolos y Operadores (Simples y Compuestos)
            ("(", 70), (")", 71), ("{", 72), ("}", 73), (",", 74), (";", 75),
            ("[", 76), ("]", 77), ("?", 78), (":", 79), ("=", 80),
            ("+", 81), ("-", 82), ("*", 83), ("/", 84), ("%", 85),
            ("++", 86), ("--", 87), ("+=", 88), ("-=", 89), ("*=", 90), ("/=", 91), ("%=", 92),
            ("==", 93), ("!=", 94), ("<", 95), (">", 96), ("<=", 97), (">=", 98),
            ("&&", 120), ("||", 121), ("!", 122), ("&", 123), ("|", 124), ("^", 125), ("~", 126),
            ("<<", 127), (">>", 128),
        ];
        for (lex, tok) in tokens { m.insert(lex.to_string(), tok); }
        Self { tabla_tokens: m }
    }

    fn get_token_palabra(&self, lexema: &str) -> i32 {
        if let Some(&tok) = self.tabla_tokens.get(lexema) { tok } 
        else { 400 } // Identificador (ID)
    }

    fn get_token_simbolo(&self, lexema: &str) -> i32 {
        if let Some(&tok) = self.tabla_tokens.get(lexema) { tok } 
        else { 0 }
    }

    fn get_descripcion_token(&self, token_id: i32) -> String {
        match token_id {
            1..=32 => "Palabra Reservada del Lenguaje".to_string(),
            100..=103 => "Función Estándar <stdio.h>".to_string(),
            104..=105 => "Función de Memoria <stdlib.h>".to_string(),
            106..=109 => "Función de Cadena <string.h>".to_string(),
            110..=112 => "Función Matemática <math.h>".to_string(),
            113..=115 => "Función de Consola <conio.h>".to_string(),
            116 => "Macro Constante (Librería Estándar)".to_string(),
            200 | 201 => "Directiva del Preprocesador".to_string(),
            202 => "Cuerpo de la Definición Macro (#define)".to_string(),
            205 => "Cabecera de Biblioteca (Header)".to_string(),
            70..=98 | 120..=128 => "Operador / Delimitador".to_string(),
            400 => "Identificador (ID)".to_string(),
            500 => "Constante Entera (Int)".to_string(),
            501 => "Constante Real (Float)".to_string(),
            600 => "Literal String".to_string(),
            601 => "Literal Char".to_string(),
            700 => "Comentario de Línea (//)".to_string(),        // NUEVO
            701 => "Comentario de Bloque (/* */)".to_string(),    // NUEVO
            _ => "Token Desconocido / Error".to_string(),
        }
    }
}

#[derive(Clone)]
struct TokenData {
    linea: usize,
    columna: usize,
    lexema: String,
    token_id: i32,
    descripcion: String,
}

// =========================================================================
// --- CLASE: AnalizadorLexico (Motor de Autómatas) ---
// =========================================================================
struct AnalizadorLexico {
    unidades: UnidadesLexicas,
}

impl AnalizadorLexico {
    fn new() -> Self {
        Self { unidades: UnidadesLexicas::new() }
    }

    fn get_alfabeto_alfanumerico(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '#'
    }

    fn get_alfabeto_numero(&self, c: char) -> bool {
        c.is_numeric() || c == '.'
    }

    fn get_alfabeto_simbolo(&self, c: char) -> bool {
        let simbolos = "(){}[],;?=+-*/%&|^~<>!:";
        simbolos.contains(c)
    }

    fn entero_real(&self, caracteres: &[char], j: &mut usize) -> (String, i32, String) {
        let mut lex = String::new();
        let mut puntos = 0;
        while *j < caracteres.len() && self.get_alfabeto_numero(caracteres[*j]) {
            if caracteres[*j] == '.' { puntos += 1; }
            lex.push(caracteres[*j]);
            *j += 1;
        }
        if puntos > 1 {
            (lex, 0, "ERROR: Número real mal formado (múltiples puntos)".to_string())
        } else if puntos == 1 {
            (lex, 501, self.unidades.get_descripcion_token(501))
        } else {
            (lex, 500, self.unidades.get_descripcion_token(500))
        }
    }

    fn analizar(&self, codigo: &str) -> (Vec<TokenData>, Vec<String>) {
        let mut tokens_encontrados = Vec::new();
        let mut errores = Vec::new();
        
        // Variables de persistencia para el autómata de bloque multilínea
        let mut en_comentario_bloque = false;
        let mut linea_comentario_abierto = 0;
        let mut col_comentario_abierto = 0;
        let mut lexema_bloque_acumulado = String::new();

        let mut esperando_header = false;
        let codigo_sanitizado = codigo.replace("\r", "");

        for (i, linea_texto) in codigo_sanitizado.lines().enumerate() {
            let num_linea = i + 1;
            let caracteres: Vec<char> = linea_texto.chars().collect();
            let mut j = 0;

            if esperando_header && !linea_texto.trim().starts_with('<') && !linea_texto.trim().starts_with('"') {
                esperando_header = false;
            }

            // Si venimos arrastrando un comentario de bloque de la línea anterior, añadir el salto de línea
            if en_comentario_bloque && !lexema_bloque_acumulado.is_empty() {
                lexema_bloque_acumulado.push('\n');
            }

            while j < caracteres.len() {
                let num_columna = j + 1;
                let c = caracteres[j];

                // 1. LÓGICA ACTIVA DEL AUTÓMATA DE COMENTARIOS
                if en_comentario_bloque {
                    if j + 1 < caracteres.len() && c == '*' && caracteres[j+1] == '/' {
                        lexema_bloque_acumulado.push('*');
                        lexema_bloque_acumulado.push('/');
                        j += 2;
                        en_comentario_bloque = false;

                        // Emitir el Token de Bloque Completado
                        tokens_encontrados.push(TokenData {
                            linea: linea_comentario_abierto,
                            columna: col_comentario_abierto,
                            lexema: lexema_bloque_acumulado.clone(),
                            token_id: 701,
                            descripcion: self.unidades.get_descripcion_token(701),
                        });
                        lexema_bloque_acumulado.clear();
                    } else {
                        lexema_bloque_acumulado.push(c);
                        j += 1;
                    }
                    continue;
                }

                // Detectar inicio de comentario estando fuera de uno
                if j + 1 < caracteres.len() && c == '/' {
                    // Caso A: Comentario de Línea
                    if caracteres[j+1] == '/' {
                        let mut lex_comentario = String::new();
                        while j < caracteres.len() {
                            lex_comentario.push(caracteres[j]);
                            j += 1;
                        }
                        tokens_encontrados.push(TokenData {
                            linea: num_linea,
                            columna: num_columna,
                            lexema: lex_comentario,
                            token_id: 700,
                            descripcion: self.unidades.get_descripcion_token(700),
                        });
                        continue;
                    }
                    
                    // Caso B: Inicio de Comentario de Bloque
                    if caracteres[j+1] == '*' {
                        en_comentario_bloque = true;
                        linea_comentario_abierto = num_linea;
                        col_comentario_abierto = num_columna;
                        lexema_bloque_acumulado.push('/');
                        lexema_bloque_acumulado.push('*');
                        j += 2;
                        continue;
                    }
                }

                if c.is_whitespace() {
                    j += 1;
                    continue;
                }

                // 2. Autómata Contextual de Headers
                if esperando_header {
                    if c == '<' || c == '"' {
                        let cierre = if c == '<' { '>' } else { '"' };
                        let mut lib_lex = String::new();
                        lib_lex.push(c); 
                        j += 1;
                        let mut cerrado = false;
                        
                        while j < caracteres.len() {
                            lib_lex.push(caracteres[j]);
                            if caracteres[j] == cierre {
                                j += 1; cerrado = true; break;
                            }
                            j += 1;
                        }
                        if cerrado {
                            tokens_encontrados.push(TokenData {
                                linea: num_linea, columna: num_columna, lexema: lib_lex, token_id: 205,
                                descripcion: self.unidades.get_descripcion_token(205),
                            });
                        } else {
                            errores.push(format!("Línea {}, Col {}: ERROR -> Header sin cerrar con '{}'.", num_linea, num_columna, cierre));
                        }
                        esperando_header = false;
                        continue;
                    }
                }

                // 3. Autómata de Cadenas de Texto Literales "..."
                if c == '"' {
                    let mut lex = String::new();
                    lex.push(c); j += 1;
                    let mut cerrado = false;
                    while j < caracteres.len() {
                        if caracteres[j] == '\\' && j + 1 < caracteres.len() {
                            lex.push(caracteres[j]); lex.push(caracteres[j+1]); j += 2;
                            continue;
                        }
                        if caracteres[j] == '"' { lex.push('"'); j += 1; cerrado = true; break; }
                        lex.push(caracteres[j]); j += 1;
                    }
                    if cerrado {
                        tokens_encontrados.push(TokenData { linea: num_linea, columna: num_columna, lexema: lex, token_id: 600, descripcion: self.unidades.get_descripcion_token(600) });
                    } else {
                        errores.push(format!("Línea {}, Col {}: ERROR -> Cadena de texto sin cerrar.", num_linea, num_columna));
                    }
                    continue;
                }

                // 4. Autómata de Carácter Literal 'a'
                if c == '\'' {
                    let mut lex = String::new();
                    lex.push(c); j += 1;
                    if j < caracteres.len() {
                        if caracteres[j] == '\\' && j + 1 < caracteres.len() {
                            lex.push(caracteres[j]); lex.push(caracteres[j+1]); j += 2;
                        } else {
                            lex.push(caracteres[j]); j += 1;
                        }
                    }
                    if j < caracteres.len() && caracteres[j] == '\'' {
                        lex.push('\''); j += 1;
                        tokens_encontrados.push(TokenData { linea: num_linea, columna: num_columna, lexema: lex, token_id: 601, descripcion: self.unidades.get_descripcion_token(601) });
                    } else {
                        errores.push(format!("Línea {}, Col {}: ERROR -> Carácter literal inválido.", num_linea, num_columna));
                        if j < caracteres.len() { j += 1; }
                    }
                    continue;
                }

                // 5. Autómata de Constantes Numéricas
                if c.is_numeric() || (c == '.' && j + 1 < caracteres.len() && caracteres[j+1].is_numeric()) {
                    let (lex, id, desc) = self.entero_real(&caracteres, &mut j);
                    if id == 0 { errores.push(format!("Línea {}, Col {}: {}", num_linea, num_columna, desc)); }
                    else { tokens_encontrados.push(TokenData { linea: num_linea, columna: num_columna, lexema: lex, token_id: id, descripcion: desc }); }
                    continue;
                }

                // 6. Identificadores, Palabras Reservadas y Macros del Preprocesador
                if c.is_alphabetic() || c == '#' || c == '_' {
                    let mut lex = String::new();
                    while j < caracteres.len() && self.get_alfabeto_alfanumerico(caracteres[j]) {
                        lex.push(caracteres[j]); j += 1;
                    }
                    let id = self.unidades.get_token_palabra(&lex);
                    let desc = self.unidades.get_descripcion_token(id);

                    tokens_encontrados.push(TokenData { linea: num_linea, columna: num_columna, lexema: lex.clone(), token_id: id, descripcion: desc });

                    if id == 200 { 
                        esperando_header = true; 
                    } else if id == 201 { 
                        while j < caracteres.len() && caracteres[j].is_whitespace() {
                            j += 1;
                        }
                        
                        let columna_macro = j + 1; 
                        let mut macro_content = String::new();
                        
                        while j < caracteres.len() {
                            macro_content.push(caracteres[j]);
                            j += 1;
                        }
                        
                        let contenido_limpio = macro_content.trim().to_string();
                        if !contenido_limpio.is_empty() {
                            tokens_encontrados.push(TokenData {
                                linea: num_linea,
                                columna: columna_macro,
                                lexema: contenido_limpio,
                                token_id: 202,
                                descripcion: self.unidades.get_descripcion_token(202),
                            });
                        }
                    }
                    continue;
                }

                // 7. Símbolos y Operadores
                if self.get_alfabeto_simbolo(c) {
                    let mut lex = c.to_string();
                    j += 1;

                    if j < caracteres.len() {
                        let prueba_doble = format!("{}{}", c, caracteres[j]);
                        let id_doble = self.unidades.get_token_simbolo(&prueba_doble);
                        if id_doble != 0 {
                            lex = prueba_doble;
                            j += 1;
                        }
                    }

                    let id = self.unidades.get_token_simbolo(&lex);
                    if id != 0 {
                        let desc = self.unidades.get_descripcion_token(id);
                        tokens_encontrados.push(TokenData { linea: num_linea, columna: num_columna, lexema: lex, token_id: id, descripcion: desc });
                    } else {
                        errores.push(format!("Línea {}, Col {}: ERROR -> Símbolo [{}] desconocido.", num_linea, num_columna, lex));
                    }
                    continue;
                }

                errores.push(format!("Línea {}, Col {}: ERROR -> Carácter ilegal nativo.", num_linea, num_columna));
                j += 1;
            }
        }

        if en_comentario_bloque {
            errores.push(format!("ERROR FIN DE ARCHIVO: Comentario de bloque (/*) abierto en línea {} nunca se cerró.", linea_comentario_abierto));
        }

        (tokens_encontrados, errores)
    }
}

// =========================================================================
// --- INTERFAZ GRÁFICA INTERACTIVA (Estructura de la Aplicación) ---
// =========================================================================
struct MicroCApp {
    code_content: String,
    tokens: Vec<TokenData>,
    errores_log: Vec<String>,
    archivo: String,
    is_modified: bool,
    analizador: AnalizadorLexico,
    ultimo_titulo: String,
    
    // Contadores Estadísticos de la UI
    total_t: usize,
    reservadas_t: usize,
    identificadores_t: usize,
    simbolos_t: usize,
    literales_t: usize,
}

impl Default for MicroCApp {
    fn default() -> Self {
        Self {
            code_content: String::new(), tokens: Vec::new(), errores_log: Vec::new(),
            archivo: String::new(), is_modified: false, analizador: AnalizadorLexico::new(), ultimo_titulo: String::new(),
            total_t: 0, reservadas_t: 0, identificadores_t: 0, simbolos_t: 0, literales_t: 0,
        }
    }
}

impl MicroCApp {
    fn opc_nuevo_click(&mut self) {
        *self = Self::default();
    }

    fn opc_abrir_click(&mut self) {
        if let Some(path) = rfd::FileDialog::new().add_filter("Código C", &["c", "h"]).pick_file() {
            if let Ok(contenido) = fs::read_to_string(&path) {
                self.code_content = contenido;
                self.archivo = path.to_string_lossy().into_owned();
                self.is_modified = false;
                self.tokens.clear();
                self.errores_log.clear();
            }
        }
    }

    fn opc_guardar_click(&mut self) {
        if !self.archivo.is_empty() {
            let _ = fs::write(&self.archivo, &self.code_content);
            self.is_modified = false;
        } else {
            self.opc_guardar_como_click();
        }
    }

    fn opc_guardar_como_click(&mut self) {
        if let Some(path) = rfd::FileDialog::new().add_filter("Código C", &["c"]).set_file_name("codigo.c").save_file() {
            let _ = fs::write(&path, &self.code_content);
            self.archivo = path.to_string_lossy().into_owned();
            self.is_modified = false;
        }
    }

    fn opc_compilar_click(&mut self) {
        let (toks, errs) = self.analizador.analizar(&self.code_content);
        self.tokens = toks;
        self.errores_log = errs;
        
        self.total_t = self.tokens.len();
        
        self.reservadas_t = self.tokens.iter().filter(|t| 
            (t.token_id >= 1 && t.token_id <= 115) || 
            t.token_id == 200 || t.token_id == 201 || 
            t.token_id == 205
        ).count();
        
        self.identificadores_t = self.tokens.iter().filter(|t| t.token_id == 400).count();
        self.simbolos_t = self.tokens.iter().filter(|t| (t.token_id >= 70 && t.token_id <= 98) || (t.token_id >= 120 && t.token_id <= 128)).count();
        self.literales_t = self.tokens.iter().filter(|t| t.token_id >= 500 && t.token_id <= 601).count();
    }

    fn opc_salir_click(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for MicroCApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let titulo_actual = if self.archivo.is_empty() {
            format!("MicroC IDE - [Sin Título]{}", if self.is_modified { " *" } else { "" })
        } else {
            format!("MicroC IDE - {}{}", self.archivo, if self.is_modified { " *" } else { "" })
        };

        if self.ultimo_titulo != titulo_actual {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(titulo_actual.clone()));
            self.ultimo_titulo = titulo_actual;
        }
        
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Archivos", |ui| {
                    if ui.button("Nuevo_").clicked() { self.opc_nuevo_click(); ui.close_menu(); }
                    if ui.button("Abrir").clicked() { self.opc_abrir_click(); ui.close_menu(); }
                    if ui.button("Guardar_").clicked() { self.opc_guardar_click(); ui.close_menu(); }
                    if ui.button("Guardar Como_").clicked() { self.opc_guardar_como_click(); ui.close_menu(); }
                    ui.separator();
                    if ui.button("Salir").clicked() { self.opc_salir_click(ctx); ui.close_menu(); }
                });

                ui.menu_button("Editar", |ui| {
                    if ui.button("Deshacer").clicked() { ui.close_menu(); }
                    if ui.button("Rehacer").clicked() { ui.close_menu(); }
                });
                
                ui.menu_button("Compilar", |ui| {
                    if ui.button("🚀 Compilar").clicked() {
                        self.opc_compilar_click();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Ayuda", |ui| {
                    if ui.button("🌐 Manual de Usuario / Documentación").clicked() {
                        ctx.open_url(egui::OpenUrl::new_tab("https://github.com/Nv205/Analizador-Lexico-Micro-C-Nefi-Vivar.git")); 
                        ui.close_menu();
                    }
                });
            });
        });

        // PANEL LATERAL: Métricas en tiempo de ejecución
        egui::SidePanel::right("panel_metricas").resizable(false).default_width(230.0).show(ctx, |ui| {
            ui.heading("📊 Estadísticas de la UI");
            ui.separator();
            ui.label(format!("Total de Tokens: {}", self.total_t));
            ui.label(format!("Palabras / Directivas: {}", self.reservadas_t));
            ui.label(format!("Identificadores (ID): {}", self.identificadores_t));
            ui.label(format!("Operadores / Símbolos: {}", self.simbolos_t));
            ui.label(format!("Literales Analizados: {}", self.literales_t));
            ui.separator();
            if !self.errores_log.is_empty() {
                ui.label(egui::RichText::new(format!("❌ Errores: {}", self.errores_log.len())).color(egui::Color32::LIGHT_RED).strong());
            } else if self.total_t > 0 {
                ui.label(egui::RichText::new("✅ Código Compilado").color(egui::Color32::LIGHT_GREEN).strong());
            }
        });

        // PANEL CENTRAL: Zona de visualización de datos
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| {
                    ui.label("📝 Editor de Texto (Código Fuente):");
                    let edit_resp = ui.add(egui::TextEdit::multiline(&mut self.code_content)
                        .font(egui::TextStyle::Monospace).desired_width(f32::INFINITY).desired_rows(24));
                    if edit_resp.changed() { self.is_modified = true; }
                });

                cols[1].vertical(|ui| {
                    ui.label("📋 Rejilla de Componentes Léxicos (egui::Grid):");
                    egui::ScrollArea::vertical().id_source("scroll_tabla").max_height(300.0).show(ui, |ui| {
                        egui::Grid::new("tabla_analisis").striped(true).min_col_width(40.0).show(ui, |ui| {
                            ui.label(egui::RichText::new("LÍNEA").strong());
                            ui.label(egui::RichText::new("COL").strong());
                            ui.label(egui::RichText::new("LEXEMA").strong());
                            ui.label(egui::RichText::new("TOKEN").strong());
                            ui.label(egui::RichText::new("DESCRIPCIÓN").strong());
                            ui.end_row();

                            for t in &self.tokens {
                                ui.label(t.linea.to_string());
                                ui.label(t.columna.to_string());
                                
                                // Pintar los comentarios con un color verde sutil diferenciado
                                if t.token_id == 700 || t.token_id == 701 {
                                    ui.label(egui::RichText::new(&t.lexema).color(egui::Color32::from_rgb(46, 139, 87)));
                                } else {
                                    ui.label(egui::RichText::new(&t.lexema).color(egui::Color32::LIGHT_BLUE));
                                }
                                
                                ui.label(t.token_id.to_string());
                                ui.label(&t.descripcion);
                                ui.end_row();
                            }
                        });
                    });

                    ui.separator();
                    ui.label("🛑 Consola de Errores:");
                    egui::ScrollArea::vertical().id_source("scroll_consola").max_height(100.0).show(ui, |ui| {
                        if self.errores_log.is_empty() {
                            ui.label(egui::RichText::new("[No hay errores registrados]").color(egui::Color32::GRAY));
                        } else {
                            for err in &self.errores_log {
                                ui.label(egui::RichText::new(err).color(egui::Color32::LIGHT_RED).monospace());
                            }
                        }
                    });
                });
            });
        });
    }
}

// =========================================================================
// --- FUNCIÓN PRINCIPAL ---
// =========================================================================
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Compilador MicroC", options, Box::new(|_| Box::new(MicroCApp::default())))
}
