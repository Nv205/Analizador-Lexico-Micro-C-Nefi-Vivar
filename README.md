# Compilador-Analixador Lexico-Nefi-Vivar

## Información General

**Nombre:** Nefi André Vivar Castañeda  
**Carné:** 202425505  
**Curso:** Autómatas y Lenguajes  
**Proyecto:** Proyecto Compilador: Analizador Lexico 

---

# Descripción

El proyecto actual consiste en el desarrollo de un Analizador Lexico  micro-C utilizando Rust y una interfaz gráfica moderna basada en `egui/eframe`.

Actualmente el programa implementa la primera etapa de un compilador: el análisis léxico. El sistema es capaz de leer código fuente escrito en C, recorrerlo carácter por carácter y clasificar los lexemas encontrados mediante autómatas léxicos especializados.

El compilador implementa:

- Interfaz gráfica interactiva.
- Editor de código fuente.
- Sistema de apertura y guardado de archivos `.c` y `.h`.
- Analizador léxico funcional.
- Generación de tabla de tokens.
- Consola de errores léxicos.
- Identificación de:
  - Palabras reservadas.
  - Identificadores.
  - Números enteros.
  - Números reales.
  - Operadores simples y compuestos.
  - Comentarios.
  - Literales string.
  - Literales char.
  - Directivas del preprocesador.
  - Headers (`#include <stdio.h>`).
- Detección de errores léxicos.
- Estadísticas dinámicas del análisis.

El funcionamiento interno del compilador está basado en un árbol de decisión y múltiples subautómatas léxicos que permiten reconocer distintos tipos de tokens del lenguaje C.

---

# Funcionalidades Implementadas

## Gestión de Archivos

- Nuevo archivo.
- Abrir archivos `.c` y `.h`.
- Guardar.
- Guardar como.
- Actualización dinámica del título de ventana.
- Detección de cambios sin guardar.

---

## Analizador Léxico

El compilador implementa un autómata principal encargado de recorrer el código fuente y delegar el reconocimiento de patrones a distintos subautómatas especializados.

### Reconocimiento de Tokens

### Palabras Reservadas

Ejemplos:

```c
int
float
if
while
return
```

---

### Identificadores

Ejemplos:

```c
edad
contador
miVariable
```

---

### Números Enteros y Reales

Ejemplos:

```c
25
3.14
0.5
```

---

### Operadores y Delimitadores

Ejemplos:

```c
+
-
==
>=
&&
{
}
;
```

---

### Comentarios

Comentarios de línea:

```c
// comentario
```

Comentarios de bloque:

```c
/* comentario */
```

---

### Literales String

```c
"Hola Mundo"
```

---

### Literales Char

```c
'A'
'\n'
```

---

### Directivas del Preprocesador

```c
#include
#define
```

---

### Librerías/Header

```c
<stdio.h>
<math.h>
```

---

# Arquitectura del Proyecto

## Clase `UnidadesLexicas`

Funciona como la tabla de símbolos del lenguaje.

Responsabilidades:

- Asociar lexemas con tokens.
- Clasificar tokens.
- Generar descripciones léxicas.

---

## Clase `AnalizadorLexico`

Es el motor principal del compilador.

Responsabilidades:

- Recorrer el código fuente.
- Aplicar el árbol de decisión léxico.
- Ejecutar subautómatas especializados.
- Generar tokens.
- Detectar errores léxicos.

### Subautómatas implementados

- `entero_real()`
- `automata_comentario()`
- reconocimiento de strings
- reconocimiento de chars
- reconocimiento de operadores
- reconocimiento de identificadores

---

## Clase `MicroCApp`

Controla toda la interfaz gráfica del compilador.

Responsabilidades:

- Editor de código.
- Menús.
- Tabla de tokens.
- Consola de errores.
- Estadísticas del análisis.
- Gestión de archivos.

---

# Tecnologías Utilizadas

## Lenguaje Principal

- Rust

---

## Librerías

### 1. eframe / egui

Framework principal de la interfaz gráfica.

Funciones:

- Editor de texto.
- Tabla de análisis léxico.
- Consola de errores.
- Panel de estadísticas.
- Barra de menús.

---

### 2. rfd (Rust File Dialog)

Permite utilizar cuadros de diálogo nativos del sistema operativo.

Funciones:

- Abrir archivos.
- Guardar archivos.
- Guardar como.

---

### 3. std::fs

Manejo del sistema de archivos.

Funciones:

- Lectura de archivos.
- Escritura de archivos.

---

### 4. std::collections::HashMap

Estructura utilizada para implementar la tabla de tokens del lenguaje.

Funciones:

- Búsqueda rápida de lexemas.
- Asociación token → descripción.

---

### 5. std::path::PathBuf

Gestión segura de rutas de archivos.

Funciones:

- Compatibilidad multiplataforma.
- Manejo de rutas de archivos.

---

# Estructura del Analizador Léxico

El compilador sigue el siguiente flujo de análisis:

```text
Leer carácter
      ↓
¿Espacio en blanco?
      ↓
¿Comentario?
      ↓
¿Cadena?
      ↓
¿Literal Char?
      ↓
¿Número?
      ↓
¿Identificador o palabra reservada?
      ↓
¿Operador o símbolo?
      ↓
Error léxico
```

Cada decisión dirige el flujo hacia un autómata especializado encargado de validar el patrón correspondiente.

---

# Requisitos Previos

Antes de ejecutar el proyecto asegúrate de tener instalado:

## Rust Toolchain

Instalar desde:

https://rust-lang.org/es/learn/get-started/

Debe incluir:

- rustup
- cargo
- rustc

---

## Dependencias del Sistema

### Windows

Generalmente no requiere configuración adicional.

### Linux

Puede requerir:

```bash
libgtk-3-dev
libssl-dev
libclang-dev
```

---

# Dependencias del Proyecto

## Cargo.toml

```toml
[dependencies]
eframe = "0.27"
rfd = "0.14"
```

---

# Instrucciones de Ejecución

## 1. Verificar el Proyecto (Abrir desde terminal de Vs-code o cualquier otro editor de codigo.)

```bash
cargo check
```

---

## 2. Compilar el Proyecto

```bash
cargo build
```

---

## 3. Ejecutar el Compilador

```bash
cargo run
```

---

# Pruebas de Funcionamiento

## Nuevo Archivo

- Limpia el editor.
- Reinicia tokens y errores.

---

## Abrir Archivo

- Permite cargar archivos `.c` y `.h`.

---

## Guardar

- Guarda automáticamente si el archivo existe.
- Solicita ubicación si es nuevo.

---

## Compilar

El botón:

```text
OpcCompilar_Click()
```

realiza:

- análisis léxico completo,
- generación de tokens,
- clasificación léxica,
- detección de errores,
- actualización de estadísticas.

---

## Tabla de Tokens

Muestra:

- línea,
- columna,
- lexema,
- token,
- descripción.

---

## Consola de Errores

Muestra errores como:

```text
Cadena sin cerrar
Carácter ilegal
Número mal formado
Comentario sin cerrar
```

---

# Estado Actual del Proyecto

Actualmente el proyecto implementa correctamente la etapa de análisis léxico del compilador Micro C.

Próximas fases posibles:

- Análisis sintáctico.
- Árbol sintáctico.
- Tabla de símbolos avanzada.
- Análisis semántico.
- Generación de código intermedio.
- Optimización.

---


# Capturas de Pantalla

<img width="992" height="780" alt="imagen" src="https://github.com/user-attachments/assets/4509f5b8-51e1-4336-9a90-a0931d4e5169" />
<img width="992" height="780" alt="imagen" src="https://github.com/user-attachments/assets/9f17c72a-6615-46d4-888f-b151bff794e8" />
<img width="970" height="766" alt="imagen" src="https://github.com/user-attachments/assets/40644b5c-e563-4c7d-b178-5247b148e3f5" />
<img width="990" height="778" alt="imagen" src="https://github.com/user-attachments/assets/c5dae85d-0da6-49c4-84c4-ff98ac3bf97e" />
<img width="990" height="778" alt="imagen" src="https://github.com/user-attachments/assets/ccf240ff-87ec-45e8-a209-d8d96514241b" />

# Video Demostrativo


https://youtu.be/uyT3AZCdOAM

https://youtu.be/kGSAHHee-zs

https://youtu.be/XNcZ2sGYKDk

https://youtu.be/kQQg3jEz-ZM

# Diagrama Fase 2

<img width="1197" height="614" alt="imagen" src="https://github.com/user-attachments/assets/2b255233-8d23-483f-b1eb-c72fd3b19286" />

# Automatas Fase 2

Palabras Reservadas:
<img width="912" height="351" alt="imagen" src="https://github.com/user-attachments/assets/d95a02a9-8f61-4835-bf4c-eb44b701775c" />

Constantes Numericas
<img width="1195" height="768" alt="imagen" src="https://github.com/user-attachments/assets/651b1a19-0cbb-474e-83c5-e7deb3b05a02" />

Comentarios:
<img width="1024" height="627" alt="imagen" src="https://github.com/user-attachments/assets/f92f3a53-753c-4417-90f4-fc5bd1dd0ccd" />













